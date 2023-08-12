use self::model::SomfyEvent;
use futures_util::StreamExt;
use log::{debug};
use oauth2::AccessToken;
use serde_derive::Serialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

const WEBSOCKET_BASE_URL: &str = "wss://websocket.myfox.io/events/websocket";

pub async fn open<L>(token: AccessToken, listener: L)
where
    L: SomfyWebsocketListener + Copy,
{
    let url =
        url::Url::parse_with_params(WEBSOCKET_BASE_URL, &[("token", token.secret())]).unwrap();
    match connect_async(url).await {
        Ok((ws_stream, _)) => {
            let (_write, read) = ws_stream.split();
            read.for_each(|item| async move {
                debug!("Incoming message {:?}", item);
                let _response = match item {
                    Ok(message) => handle_incoming_message(message),
                    Err(error) => {
                        listener.on_error(error);
                        None
                    }
                };
            })
            .await;
        }
        Err(error) => {
            listener.on_error(error);
        }
    }
}

fn handle_incoming_message(
    _item: TungsteniteMessage,
) -> Option<String> {
    todo!()
}

pub trait SomfyWebsocketListener {
    fn on_ping<PONG>(&self, send_pong: PONG)
    where
        PONG: FnOnce() -> ();

    fn on_message<ACK>(&self, message: SomfyEvent, send_ack: ACK)
    where
        ACK: FnOnce() -> ();

    fn on_error(&self, message: TungsteniteError);

    fn on_close(&self, status: String, message: String);
}

#[derive(Serialize, Debug)]
struct SomfyAck {
    ack: bool,
    message_id: String,
    client: String,
}
impl From<String> for SomfyAck {
    fn from(message_id: String) -> Self {
        SomfyAck {
            ack: true,
            message_id,
            client: "Android".to_string(),
        }
    }
}

mod model {
    use chrono::{DateTime, Utc};
    use serde_derive::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct SomfyEvent {
        message_id: String,

        #[serde(flatten)]
        data: EventData,
    }

    #[derive(Deserialize, Debug)]
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
        AlarmTrespass {
            site_id: String,
            device_id: String,
            device_type: String,
            start_at: DateTime<Utc>,
            start_siren_at: DateTime<Utc>,
            end_at: DateTime<Utc>,
            end_siren_at: DateTime<Utc>,
            manual_alarm: bool,
        },

        #[serde(rename = "alarm.panic")]
        AlarmPanic {
            site_id: String,
            device_id: Option<String>,
            device_type: Option<String>,
            start_at: DateTime<Utc>,
            start_siren_at: DateTime<Utc>,
            end_at: DateTime<Utc>,
            end_siren_at: DateTime<Utc>,
            manual_alarm: bool,
        },

        #[serde(rename = "alarm.end")]
        AlarmEnd {
            site_id: String,
            device_id: Option<String>,
            device_type: Option<String>,
            end_at: DateTime<Utc>,
            end_siren_at: DateTime<Utc>,
            stopped_by_user_id: String,
        },

        #[serde(rename = "presence_in")]
        PresenceIn,

        #[serde(rename = "presence_out")]
        PresenceOut,

        #[serde(rename = "device.status")]
        DeviceStatus,

        #[serde(rename = "site.device.testing.status")]
        SiteDeviceTestingStatus,
    }

    #[derive(Deserialize, Debug)]
    pub enum SecurityLevel {
        #[serde(rename = "disarmed")]
        Disarmed,
        #[serde(rename = "partial")]
        Partial,
        #[serde(rename = "armed")]
        Armed,
    }
}
