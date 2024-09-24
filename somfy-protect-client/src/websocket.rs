use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use futures_util::lock::Mutex;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::Stream;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use log::debug;
use log::info;
use log::trace;
use log::warn;
use oauth2::AccessToken;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

const WEBSOCKET_BASE_URL: &str = "wss://websocket.myfox.io/events/websocket";
const KEEPALIVE_INTERVAL_SECONDS: u64 = 45;

pub struct SomfyWebsocketClientBuilder {
    url: String,
}

impl Default for SomfyWebsocketClientBuilder {
    fn default() -> Self {
        Self {
            url: WEBSOCKET_BASE_URL.to_string(),
        }
    }
}

impl SomfyWebsocketClientBuilder {
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub async fn connect(
        &self,
        token: AccessToken,
    ) -> Result<(
        SomfyWebsocketConnection,
        impl Stream<Item = Result<EventMessage>>,
    )> {
        info!("Connecting to {}", self.url);
        let url = url::Url::parse_with_params(&self.url, &[("token", token.secret())])?;
        let (ws_stream, _initial_message) = connect_async(url.as_str()).await?;

        let (write, read) = ws_stream.split();

        let connection = SomfyWebsocketConnection::new(write);
        let connection2 = connection.clone();

        let somfy_event_stream = read
            .map_ok(move |item| (item, connection2.clone()))
            .map_err(|err| anyhow!(err))
            .and_then(|(message, connection)| async move {
                handle_incoming_message(message.clone(), connection.clone()).await
            })
            .filter_map(|result| async {
                match result {
                    Ok(Some(event)) => Some(Ok(event)),
                    Err(error) => Some(Err(error)),
                    _ => None,
                }
            });

        Ok((connection, somfy_event_stream))
    }
}

#[derive(Clone)]
pub struct SomfyWebsocketConnection {
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>>>,
}
impl SomfyWebsocketConnection {
    fn new(
        write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>,
    ) -> Self {
        SomfyWebsocketConnection {
            write: Arc::new(Mutex::new(write)),
        }
    }

    async fn acknowledge(&self, message_id: String) -> Result<()> {
        let mut write = self.write.lock().await;
        debug!(">>> Ack({message_id})");
        let ack = EventAck::new(message_id);
        write.send(ack.into()).await?;
        Ok(())
    }

    async fn ping(&self) -> Result<()> {
        let mut write = self.write.lock().await;
        let now = chrono::Utc::now().timestamp_millis() as u32;
        debug!(">>> Ping({now})");
        write
            .send(TungsteniteMessage::Ping(now.to_ne_bytes().into()))
            .await?;
        Ok(())
    }

    async fn pong(&self, payload: Vec<u8>) -> Result<()> {
        let mut write = self.write.lock().await;
        debug!(">>> Pong({payload:?})");
        write.send(TungsteniteMessage::Pong(payload)).await?;
        Ok(())
    }

    pub async fn close(&self) -> Result<()> {
        let mut write = self.write.lock().await;
        debug!(">>> Close()");
        write.send(TungsteniteMessage::Close(None)).await?;
        write.close().await?;
        Ok(())
    }
}

async fn handle_incoming_message(
    message: TungsteniteMessage,
    connection: SomfyWebsocketConnection,
) -> Result<Option<EventMessage>> {
    trace!("<<< {message:?}");
    if message.is_close() {
        debug!("<<< Close({message})");
        info!("Received close event from websocket server");
        Ok(None)
    } else if message.is_ping() {
        debug!("<<< Ping({message})");
        connection.pong(message.into()).await?;
        Ok(None)
    } else if message.is_pong() {
        if let Some(ping_send_time) = message.clone().into_data().try_into().ok() {
            let ping_send_time = u32::from_ne_bytes(ping_send_time);
            let now = chrono::Utc::now().timestamp_millis() as u32;
            debug!(
                "<<< Pong({}) latency: {}ms",
                ping_send_time,
                now - ping_send_time
            );
        } else {
            debug!("<<< Pong({message:?})");
        }
        schedule_next_keepalive_ping(&connection);
        Ok(None)
    } else if message.is_text() {
        match message.into() {
            SomfyIncomingMessage::TokenError => {
                Err(anyhow!("Invalid token used for authentication"))
            }
            SomfyIncomingMessage::UnparseableEvent(error, body) => Err(anyhow!(
                "Received unsupported event data structure: {error}\n{body}"
            )),
            SomfyIncomingMessage::SomfyEvent(event) => {
                let message_id = event.message_id.clone();
                match event.data {
                    EventData::ConnectionReady => {
                        schedule_next_keepalive_ping(&connection);
                    }
                    _ => {}
                };
                // send ack
                if let Some(message_id) = message_id {
                    connection.acknowledge(message_id).await?;
                }
                Ok(Some(event))
            }
        }
    } else {
        Err(anyhow!("Unsupported message: {message:?}"))
    }
}

fn schedule_next_keepalive_ping(connection: &SomfyWebsocketConnection) {
    trace!("scheduling next keepalive ping in {KEEPALIVE_INTERVAL_SECONDS}s");
    let connection = connection.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(KEEPALIVE_INTERVAL_SECONDS)).await;
        match connection.ping().await {
            Err(error) => {
                warn!("Error sending keepalive ping, this means the connection will likely be closed by remote server: {error}");
            }
            _ => {}
        };
    });
}

impl From<TungsteniteMessage> for SomfyIncomingMessage {
    fn from(message: TungsteniteMessage) -> Self {
        let text = message.to_text().unwrap();
        if text == "websocket.error.token" {
            return SomfyIncomingMessage::TokenError;
        }
        return match serde_json::from_str::<EventMessage>(text) {
            Ok(event) => SomfyIncomingMessage::SomfyEvent(event),
            Err(error) => SomfyIncomingMessage::UnparseableEvent(error.into(), text.into()),
        };
    }
}

enum SomfyIncomingMessage {
    SomfyEvent(EventMessage),
    UnparseableEvent(Error, String),
    TokenError,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct EventMessage {
    pub message_id: Option<String>,

    #[serde(flatten)]
    pub data: EventData,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "key")]
pub enum EventData {
    #[serde(rename = "websocket.connection.ready")]
    ConnectionReady,

    #[serde(rename = "security.level.change")]
    SecurityLevelChange {
        site_id: String,
        security_level: SecurityLevel,
    },

    #[serde(rename = "alarm.trespass")]
    AlarmTrespass(AlarmDetails),

    #[serde(rename = "alarm.panic")]
    AlarmPanic(AlarmDetails),

    #[serde(rename = "alarm.end")]
    AlarmEnd {
        site_id: String,
        device_id: Option<String>,
        device_type: Option<String>,
        end_at: Option<String>,
        end_siren_at: Option<String>,
        stopped_by_user_id: Option<String>,
    },

    #[serde(rename = "presence_in")]
    PresenceIn,

    #[serde(rename = "presence_out")]
    PresenceOut,

    #[serde(rename = "device.status")]
    DeviceStatus {
        /// Profiles of users.
        profiles: Option<Vec<String>>,
        /// The device site identifier.
        site_id: String,
        ///
        #[serde(rename = "type")]
        status_type: Option<String>,
        /// The device unique identifier.
        device_id: String,
        /// Is device lost ? (Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
        device_lost: Option<bool>,
        /// rLink Quality.
        rlink_quality: Option<i32>,
        /// rLink Quality Percent.
        rlink_quality_percent: Option<i32>,
        /// Battery level.
        battery_level: Option<i32>,
        /// Is Tag recalibration required (Tag).
        recalibration_required: Option<bool>,
        /// Is Tag cover present (Tag).
        cover_present: Option<bool>,
        /// Last status date (Box, AIO, AIO+, Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
        last_status_at: Option<String>,
        /// The device diagnosis.
        diagnosis: Option<Diagnosis>,
    },

    #[serde(rename = "site.device.testing.status")]
    SiteDeviceTestingStatus {
        /// Profiles of users.
        profiles: Option<Vec<String>>,
        /// The device site identifier.
        site_id: String,
        ///
        #[serde(rename = "type")]
        status_type: Option<String>,
        /// The site testing diagnosis.
        diagnosis: Option<SiteDiagnosis>,
    },
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct AlarmDetails {
    /// The device site identifier.
    pub site_id: String,
    /// The device unique identifier.
    pub device_id: Option<String>,
    ///
    pub device_type: Option<String>,
    /// Starting time of last alarm.
    pub start_at: Option<String>,
    /// Starting siren time of last alarm.
    pub start_siren_at: Option<String>,
    /// End time of last alarm.
    pub end_at: Option<String>,
    /// End siren time of last alarm.
    pub end_siren_at: Option<String>,
    /// Is last alarm manual or automatic.
    pub manual_alarm: Option<bool>,
}

pub type SecurityLevel = somfy_protect_openapi::models::site_output::SecurityLevel;

pub type Diagnosis = somfy_protect_openapi::models::Diagnosis;

pub type SiteDiagnosis = somfy_protect_openapi::models::SiteDiagnosis;

#[derive(Serialize, Debug)]
pub struct EventAck {
    ack: bool,
    message_id: String,
    client: String,
}

impl EventAck {
    fn new(message_id: String) -> Self {
        EventAck {
            ack: true,
            message_id,
            client: "Android".to_string(),
        }
    }
}

impl Into<TungsteniteMessage> for EventAck {
    fn into(self) -> TungsteniteMessage {
        let ack_body = serde_json::to_string(&self).unwrap();
        TungsteniteMessage::Text(ack_body)
    }
}
