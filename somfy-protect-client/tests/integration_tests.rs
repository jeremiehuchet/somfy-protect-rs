use reqwest::{header::CONTENT_TYPE, Client};
use serde_json::Value;
use somfy_protect_client::{
    client::{SomfyProtectClient, SomfyProtectClientBuilder},
    websocket::SomfyWebsocketClientBuilder,
};
use testcontainers::{
    core::{ContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage,
};

async fn start_somfy_mock() -> ContainerAsync<GenericImage> {
    GenericImage::new("somfy-protect-api-mock", "latest")
        .with_exposed_port(ContainerPort::Tcp(3000))
        .with_wait_for(WaitFor::message_on_stdout(
            "Somfy Protect API mock listening on port 3000",
        ))
        .start()
        .await
        .unwrap()
}

async fn client_for(container: &ContainerAsync<GenericImage>) -> SomfyProtectClient {
    let listening_port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(3000)
        .unwrap();
    SomfyProtectClientBuilder::default()
        .with_auth_base_url(format!("http://127.0.0.1:{listening_port}/auth",))
        .with_api_base_url(format!("http://127.0.0.1:{listening_port}/api",))
        .with_client_credentials("somfy", "somfy secret")
        .with_user_credentials("user@somfy.com", "user password")
        .build()
}

async fn websocket_for(container: &ContainerAsync<GenericImage>) -> SomfyWebsocketClientBuilder {
    let listening_port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(3000)
        .unwrap();
    SomfyWebsocketClientBuilder::default()
        .with_url(format!("ws://127.0.0.1:{listening_port}/websocket"))
}

async fn assert_mock_invocation_count(
    container: &ContainerAsync<GenericImage>,
    feature: &str,
    expected_count: u32,
) {
    let listening_port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(3000)
        .unwrap();
    let http_client = Client::builder().build().expect("an http client");
    let request = http_client
        .get(format!("http://127.0.0.1:{listening_port}/mock/{feature}"))
        .build()
        .unwrap();
    let actual_count: u32 = http_client
        .execute(request)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    assert_eq!(
        actual_count, expected_count,
        "expecting {expected_count} {feature}"
    );
}

async fn given_websocket_server_will_send(
    container: &ContainerAsync<GenericImage>,
    messages: Vec<&str>,
) {
    let listening_port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(3000)
        .unwrap();
    let http_client = Client::builder().build().expect("an http client");
    let request = http_client
        .put(format!(
            "http://127.0.0.1:{listening_port}/mock/ws-messages-to-send"
        ))
        .header(CONTENT_TYPE, "application/json")
        .body(format!("[{}]", messages.join(",")))
        .build()
        .unwrap();
    let response = http_client.execute(request).await.unwrap();
    assert_eq!(response.status(), 200, "mock messages successfully set")
}

async fn assert_server_received_acks(
    container: &ContainerAsync<GenericImage>,
    expected_acks: &[&str],
) {
    let expected_messages = expected_acks
        .iter()
        .map(|message_id| {
            format!(r#"{{"ack":true,"message_id":"{message_id}","client":"Android"}}"#)
        })
        .collect();
    assert_server_received_messages(container, expected_messages).await;
}

async fn assert_server_received_messages(
    container: &ContainerAsync<GenericImage>,
    expected_messages: Vec<String>,
) {
    let listening_port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(3000)
        .unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    let http_client = Client::builder().build().expect("an http client");
    let request = http_client
        .get(format!(
            "http://127.0.0.1:{listening_port}/mock/ws-messages-received"
        ))
        .build()
        .unwrap();
    let received_messages = http_client
        .execute(request)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap()
        .as_array()
        .unwrap()
        .clone();

    for expected_message in expected_messages {
        let expected_message: Value = serde_json::from_str(&expected_message).unwrap();
        assert_eq!(
            received_messages.contains(&expected_message),
            true,
            r#"
            missing message: {expected_message:?}
            in list: {received_messages:?}
            "#
        );
    }
}

mod api {
    use somfy_protect_openapi::models::site_alarm::Status;
    use somfy_protect_openapi::models::site_output::SecurityLevel;

    use crate::{assert_mock_invocation_count, client_for, start_somfy_mock};

    #[tokio::test]
    async fn can_list_sites() {
        let container = start_somfy_mock().await;
        let client = client_for(&container).await;

        let sites = client.list_sites().await.unwrap();
        assert_eq!(sites.len(), 1, "expect 1 site id");
        let site = sites.get(0).unwrap();
        assert_eq!(site.site_id, "Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT", "site id");
        assert_eq!(site.name, Some("Hausalarm".to_string()), "site name");
        assert_eq!(
            site.security_level,
            SecurityLevel::Disarmed,
            "security level"
        );
        assert_eq!(site.alarm.status, Status::None, "alarm status");
    }

    #[tokio::test]
    async fn can_list_devices() {
        let container = start_somfy_mock().await;
        let client = client_for(&container).await;

        let devices = client
            .list_devices("Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string())
            .await
            .unwrap();
        let device_ids: Vec<String> = devices
            .iter()
            .map(|d| format!("{} {}", d.device_id, d.label.as_ref().unwrap()))
            .collect();
        assert_eq!(
            device_ids,
            vec![
                "c5441e32bc6aa634a8cf9b7fd1dd4ab3 Link Advanced",
                "DztQPcqPWAXfxq2BoR1VF8BtXI5F7zlu Window Room A",
                "HDkKd0vS7yeFSfwPIJFLc2HWNiv9muRu Window Room B",
                "2Xd3hHicDKb9NcfJpAJlqBb0QmbDIKG9 Joakim Key Fob",
                "GKZsZfvCZRj4KVEQ0rQrLK3Qq4hS2ubS Garagen Bewegungsmelder",
                "BayzpyoOyvZYMQwqsnmmu3FBHHz9cuLX Input Keypad",
                "MCzYWIDNuWNn5qfdUK5U2HiM0iDXQzMA Innensirene",
                "BStcq4XsKYBOdwspLS9TG8B9cZzEmCFO Außensirene"
            ],
            "expect devices"
        );
    }

    #[tokio::test]
    async fn should_refresh_access_token() {
        let container = start_somfy_mock().await;
        let client = client_for(&container).await;

        // should execute exchange password
        let sites = client.list_sites().await.unwrap();
        let site_ids: Vec<&String> = sites.iter().map(|s| &s.site_id).collect();
        assert_eq!(
            site_ids,
            vec!["Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT"],
            "expect 1 site id"
        );
        assert_mock_invocation_count(&container, "exchange-password-count", 1).await;
        assert_mock_invocation_count(&container, "refresh-token-count", 0).await;

        // should execute refresh token
        let sites = client.list_sites().await.unwrap();
        let site_ids: Vec<&String> = sites.iter().map(|s| &s.site_id).collect();
        assert_eq!(
            site_ids,
            vec!["Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT"],
            "expect 1 site id"
        );
        assert_mock_invocation_count(&container, "exchange-password-count", 1).await;
        assert_mock_invocation_count(&container, "refresh-token-count", 1).await;

        // should execute refresh token
        let sites = client.list_sites().await.unwrap();
        let site_ids: Vec<&String> = sites.iter().map(|s| &s.site_id).collect();
        assert_eq!(
            site_ids,
            vec!["Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT"],
            "expect 1 site id"
        );
        assert_mock_invocation_count(&container, "exchange-password-count", 1).await;
        assert_mock_invocation_count(&container, "refresh-token-count", 2).await;
    }
}

mod websocket {
    use std::collections::HashMap;

    use anyhow::Error;

    use futures_util::StreamExt;
    use oauth2::AccessToken;
    use somfy_protect_client::websocket::{
        AlarmDetails, Diagnosis, EventData, EventMessage, SecurityLevel, SiteDiagnosis,
    };
    use somfy_protect_openapi::models::site_diagnosis::MainStatus;

    use crate::{
        assert_server_received_acks, given_websocket_server_will_send, start_somfy_mock,
        websocket_for,
    };

    async fn assert_event_sequence(
        stream: impl StreamExt<Item = Result<EventMessage, Error>>,
        ordered_expected_events: &[EventMessage],
    ) {
        let expected_sequence = ordered_expected_events
            .iter()
            .map(|event| format!("{event:?}"))
            .collect::<Vec<String>>()
            .join("\n");
        let mut received_events = Vec::new();

        let mut event_stream = Box::pin(stream);
        for expected_event in ordered_expected_events {
            let actual_event = event_stream.next().await.unwrap().unwrap();
            received_events.push(actual_event);
            let actual_sequence = received_events
                .iter()
                .map(|event| format!("{event:?}"))
                .collect::<Vec<String>>()
                .join("\n");
            assert_eq!(
                expected_event,
                received_events.last().unwrap(),
                r#"
                expecting event sequence:
{expected_sequence}

                but the actual sequence starts with:
{actual_sequence}
                "#
            );
        }
    }

    #[tokio::test]
    async fn can_open_and_close_connection() {
        let container = start_somfy_mock().await;
        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();
        let mut event_stream = Box::pin(event_stream);
        let first = event_stream.next().await.unwrap().unwrap();
        assert_eq!(
            first.data,
            EventData::ConnectionReady,
            "receive a 'connection ready' event"
        );
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn can_receive_device_status_events() {
        let container = start_somfy_mock().await;
        given_websocket_server_will_send(
            &container,
            vec![
                r#"
            {
              "profiles": [
                "admin",
                "owner",
                "installer_write"
              ],
              "site_id":                "site_id_s1",
              "type":                   "testing",
              "key":                    "device.status",
              "device_id":              "device_id_d1",
              "device_lost":            false,
              "rlink_quality":          -43,
              "rlink_quality_percent":  60,
              "battery_level":          89,
              "recalibration_required": false,
              "cover_present":          true,
              "last_status_at":        "2023-05-02T19:43:21.000000Z",
              "diagnosis": {
                "is_everything_ok":     true,
                "problems":             []
              },
              "message_id":             "message_id_d1"
            }
            "#,
                r#"
            {
              "profiles": [
                "admin",
                "owner",
                "installer_write"
              ],
              "site_id":                "site_id_s1",
              "type":                   "testing",
              "key":                    "device.status",
              "device_id":              "device_id_d2",
              "device_lost":            false,
              "rlink_quality":          -54,
              "rlink_quality_percent":  68,
              "battery_level":          90,
              "recalibration_required": false,
              "cover_present":          true,
              "last_status_at":        "2023-05-02T19:43:21.000000Z",
              "diagnosis": {
                "is_everything_ok":     true,
                "problems":             []
              },
              "message_id":             "message_id_d2"
            }
            "#,
            ],
        )
        .await;

        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();

        assert_event_sequence(
            event_stream,
            &[
                EventMessage {
                    message_id: None,
                    data: EventData::ConnectionReady,
                },
                EventMessage {
                    message_id: Some("message_id_d1".into()),
                    data: EventData::DeviceStatus {
                        profiles: Some(vec![
                            "admin".into(),
                            "owner".into(),
                            "installer_write".into(),
                        ]),
                        site_id: "site_id_s1".into(),
                        status_type: Some("testing".into()),
                        device_id: "device_id_d1".into(),
                        device_lost: Some(false),
                        rlink_quality: Some(-43),
                        rlink_quality_percent: Some(60),
                        battery_level: Some(89),
                        recalibration_required: Some(false),
                        cover_present: Some(true),
                        last_status_at: Some("2023-05-02T19:43:21.000000Z".into()),
                        diagnosis: Some(Diagnosis {
                            is_everything_ok: Some(true),
                            problems: Some(Vec::new()),
                        }),
                    },
                },
                EventMessage {
                    message_id: Some("message_id_d2".into()),
                    data: EventData::DeviceStatus {
                        profiles: Some(vec![
                            "admin".into(),
                            "owner".into(),
                            "installer_write".into(),
                        ]),
                        site_id: "site_id_s1".into(),
                        status_type: Some("testing".into()),
                        device_id: "device_id_d2".into(),
                        device_lost: Some(false),
                        rlink_quality: Some(-54),
                        rlink_quality_percent: Some(68),
                        battery_level: Some(90),
                        recalibration_required: Some(false),
                        cover_present: Some(true),
                        last_status_at: Some("2023-05-02T19:43:21.000000Z".into()),
                        diagnosis: Some(Diagnosis {
                            is_everything_ok: Some(true),
                            problems: Some(Vec::new()),
                        }),
                    },
                },
            ],
        )
        .await;

        assert_server_received_acks(&container, &["message_id_d1", "message_id_d2"]).await;

        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn can_receive_site_device_testing_status_events() {
        let container = start_somfy_mock().await;
        given_websocket_server_will_send(
            &container,
            vec![
                r#"
            {
              "profiles": [
                "admin",
                "owner"
              ],
              "site_id":                      "site_id_s1",
              "type":                         "testing",
              "key":                          "site.device.testing.status",
              "diagnosis": {
                "main_status":                "ok",
                "main_message":               "diagnosis.ok",
                "main_message_vars": {
                },
                "device_diagnosis_available": true,
                "device_diagnosis_expired":   false,
                "items":                      []
              },
              "message_id":                   "message_id_dts1"
            }
            "#,
            ],
        )
        .await;

        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();

        assert_event_sequence(
            event_stream,
            &[
                EventMessage {
                    message_id: None,
                    data: EventData::ConnectionReady,
                },
                EventMessage {
                    message_id: Some("message_id_dts1".into()),
                    data: EventData::SiteDeviceTestingStatus {
                        profiles: Some(vec!["admin".into(), "owner".into()]),
                        site_id: "site_id_s1".into(),
                        status_type: Some("testing".into()),
                        diagnosis: Some(SiteDiagnosis {
                            main_status: Some(MainStatus::Ok),
                            main_message: Some("diagnosis.ok".into()),
                            main_message_vars: Some(HashMap::new()),
                            device_diagnosis_available: Some(true),
                            device_diagnosis_expired: Some(false),
                            items: Some(Vec::new()),
                        }),
                    },
                },
            ],
        )
        .await;

        assert_server_received_acks(&container, &["message_id_dts1"]).await;

        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn can_receive_can_receive_alarm_security_level_events() {
        let container = start_somfy_mock().await;
        given_websocket_server_will_send(
            &container,
            vec![
                r#"
            {
              "profiles": [
                  "owner",
                  "admin",
                  "guest",
                  "kid"
              ],
              "site_id":        "site_id_1",
              "type":           "config",
              "key":            "security.level.change",
              "security_level": "armed",
              "message_id":     "message_id_armed"
            }
            "#,
                r#"
            {
              "profiles": [
                  "owner",
                  "admin",
                  "guest",
                  "kid"
              ],
              "site_id":        "site_id_1",
              "type":           "config",
              "key":            "security.level.change",
              "security_level": "disarmed",
              "message_id":     "message_id_disarmed"
            }
            "#,
                r#"
            {
              "profiles": [
                  "owner",
                  "admin",
                  "guest",
                  "kid"
              ],
              "site_id":        "site_id_1",
              "type":           "config",
              "key":            "security.level.change",
              "security_level": "partial",
              "message_id":     "message_id_partial"
            }
            "#,
            ],
        )
        .await;

        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();

        assert_event_sequence(
            event_stream,
            &[
                EventMessage {
                    message_id: None,
                    data: EventData::ConnectionReady,
                },
                EventMessage {
                    message_id: Some("message_id_armed".into()),
                    data: EventData::SecurityLevelChange {
                        site_id: "site_id_1".into(),
                        security_level: SecurityLevel::Armed,
                    },
                },
                EventMessage {
                    message_id: Some("message_id_disarmed".into()),
                    data: EventData::SecurityLevelChange {
                        site_id: "site_id_1".into(),
                        security_level: SecurityLevel::Disarmed,
                    },
                },
                EventMessage {
                    message_id: Some("message_id_partial".into()),
                    data: EventData::SecurityLevelChange {
                        site_id: "site_id_1".into(),
                        security_level: SecurityLevel::Partial,
                    },
                },
            ],
        )
        .await;

        assert_server_received_acks(
            &container,
            &[
                "message_id_armed",
                "message_id_disarmed",
                "message_id_partial",
            ],
        )
        .await;

        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn can_receive_presence_events() {
        let container = start_somfy_mock().await;
        given_websocket_server_will_send(
            &container,
            vec![
                r#"
            {
              "profiles": [
                "owner",
                "admin"
              ],
              "site_id":    "site_id_a",
              "type":       "event",
              "key":        "presence_in",
              "user_id":    "user_id_a",
              "device_id":  "device_id_a",
              "device_type  ":"fob",
              "message_id": "message_id_a"
            }
            "#,
                r#"
            {
              "profiles": [
                "owner",
                "admin"
              ],
              "site_id":    "site_id_b",
              "type":       "event",
              "key":        "presence_out",
              "user_id":    "user_id_b",
              "device_id":  "device_id_ba",
              "device_type  ":"fob",
              "message_id": "message_id_b"
            }
            "#,
            ],
        )
        .await;

        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();

        assert_event_sequence(
            event_stream,
            &[
                EventMessage {
                    message_id: None,
                    data: EventData::ConnectionReady,
                },
                EventMessage {
                    message_id: Some("message_id_a".into()),
                    data: EventData::PresenceIn,
                },
                EventMessage {
                    message_id: Some("message_id_b".into()),
                    data: EventData::PresenceOut,
                },
            ],
        )
        .await;

        assert_server_received_acks(&container, &["message_id_a", "message_id_b"]).await;

        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn can_receive_alarm_trespass_events() {
        let container = start_somfy_mock().await;
        given_websocket_server_will_send(
            &container,
            vec![
                r#"
            {
              "profiles": [
                "owner",
                "admin",
                "custom",
                "family",
                "neighbor"
              ],
              "site_id":        "site_id_trespassed",
              "type":           "alarm",
              "key":            "alarm.trespass",
              "device_id":      "device_id_t",
              "device_type":    "pir",
              "start_at":       "2023-01-06T42:54:23.000000Z",
              "start_siren_at": "2023-01-06T42:54:53.000000Z",
              "end_at":         "2023-01-06T42:56:11.000000Z",
              "end_siren_at":   "2023-01-06T42:56:31.000000Z",
              "manual_alarm":   false,
              "message_id":     "message_id_trespass"
            }
            "#,
            ],
        )
        .await;

        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();

        assert_event_sequence(
            event_stream,
            &[
                EventMessage {
                    message_id: None,
                    data: EventData::ConnectionReady,
                },
                EventMessage {
                    message_id: Some("message_id_trespass".into()),
                    data: EventData::AlarmTrespass(AlarmDetails {
                        site_id: "site_id_trespassed".into(),
                        device_id: Some("device_id_t".into()),
                        device_type: Some("pir".into()),
                        start_at: Some("2023-01-06T42:54:23.000000Z".into()),
                        start_siren_at: Some("2023-01-06T42:54:53.000000Z".into()),
                        end_at: Some("2023-01-06T42:56:11.000000Z".into()),
                        end_siren_at: Some("2023-01-06T42:56:31.000000Z".into()),
                        manual_alarm: Some(false),
                    }),
                },
            ],
        )
        .await;

        assert_server_received_acks(&container, &["message_id_trespass"]).await;

        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn can_receive_alarm_panic_events() {
        let container = start_somfy_mock().await;
        given_websocket_server_will_send(
            &container,
            vec![
                r#"
            {
              "profiles": [
                "owner",
                "admin",
                "custom",
                "family",
                "neighbor"
              ],
              "site_id":        "site_id_panic",
              "type":           "alarm",
              "key":            "alarm.panic",
              "device_id":      null,
              "device_type":    null,
              "start_at":       "2023-01-06T42:54:23.000000Z",
              "start_siren_at": "2023-01-06T42:54:53.000000Z",
              "end_at":         "2023-01-06T42:56:11.000000Z",
              "end_siren_at":   "2023-01-06T42:56:31.000000Z",
              "manual_alarm":   false,
              "message_id":     "message_id_panic"
            }
            "#,
            ],
        )
        .await;

        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();

        assert_event_sequence(
            event_stream,
            &[
                EventMessage {
                    message_id: None,
                    data: EventData::ConnectionReady,
                },
                EventMessage {
                    message_id: Some("message_id_panic".into()),
                    data: EventData::AlarmPanic(AlarmDetails {
                        site_id: "site_id_panic".into(),
                        device_id: None,
                        device_type: None,
                        start_at: Some("2023-01-06T42:54:23.000000Z".into()),
                        start_siren_at: Some("2023-01-06T42:54:53.000000Z".into()),
                        end_at: Some("2023-01-06T42:56:11.000000Z".into()),
                        end_siren_at: Some("2023-01-06T42:56:31.000000Z".into()),
                        manual_alarm: Some(false),
                    }),
                },
            ],
        )
        .await;

        assert_server_received_acks(&container, &["message_id_panic"]).await;

        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn can_receive_alarm_end_events() {
        let container = start_somfy_mock().await;
        given_websocket_server_will_send(
            &container,
            vec![
                r#"
            {
              "profiles": [
                "owner",
                "admin",
                "custom",
                "family",
                "neighbor"
              ],
              "site_id":            "site_id_end",
              "type":               "alarm",
              "key":                "alarm.end",
              "device_id":          null,
              "device_type":        null,
              "end_at":             "2023-01-06T42:56:31.000000Z",
              "end_siren_at":       null,
              "stopped_by_user_id": "user_id_43",
              "message_id":         "message_id_end"
            }
            "#,
            ],
        )
        .await;

        let (connection, event_stream) = websocket_for(&container)
            .await
            .connect(AccessToken::new("valid-token".into()))
            .await
            .unwrap();

        assert_event_sequence(
            event_stream,
            &[
                EventMessage {
                    message_id: None,
                    data: EventData::ConnectionReady,
                },
                EventMessage {
                    message_id: Some("message_id_end".into()),
                    data: EventData::AlarmEnd {
                        site_id: "site_id_end".into(),
                        device_id: None,
                        device_type: None,
                        end_at: Some("2023-01-06T42:56:31.000000Z".into()),
                        end_siren_at: None,
                        stopped_by_user_id: Some("user_id_43".into()),
                    },
                },
            ],
        )
        .await;

        assert_server_received_acks(&container, &["message_id_end"]).await;

        connection.close().await.unwrap();
    }
}
