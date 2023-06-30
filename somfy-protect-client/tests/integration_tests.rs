use reqwest::Client;
use somfy_protect_client::client::SomfyProtectClientBuilder;
use somfy_protect_openapi::models::site_alarm::Status;
use somfy_protect_openapi::models::site_output::SecurityLevel;
use testcontainers::{clients::Cli, core::WaitFor, Container, Image};

#[derive(Default)]
struct SomfyMock {}

impl Image for SomfyMock {
    type Args = Vec<String>;

    fn name(&self) -> String {
        String::from("somfy-protect-api-mock")
    }

    fn tag(&self) -> String {
        String::from("latest")
    }
    fn expose_ports(&self) -> Vec<u16> {
        vec![3000]
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::StdOutMessage {
            message: String::from("Somfy Protect API mock listening on port 3000"),
        }]
    }
}

fn client_for<'d>(container: &Container<'d, SomfyMock>) -> SomfyProtectClientBuilder {
    let listening_port = container.ports().map_to_host_port_ipv4(3000).unwrap();
    SomfyProtectClientBuilder::default()
        .with_auth_base_url(format!("http://127.0.0.1:{listening_port}/auth"))
        .with_api_base_url(format!("http://127.0.0.1:{listening_port}/api"))
        .with_client_credentials("somfy".to_string(), "somfy secret".to_string())
        .with_user_credentials("user@somfy.com".to_string(), "user password".to_string())
}

async fn assert_mock_count<'d>(
    feature: &str,
    expected_count: u32,
    container: &Container<'d, SomfyMock>,
) {
    let listening_port = container.ports().map_to_host_port_ipv4(3000).unwrap();
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
    assert_eq!(actual_count, expected_count, "expecting {expected_count} {feature}");
}

#[tokio::test]
async fn can_list_sites() {
    let docker = Cli::docker();
    let container = docker.run(SomfyMock::default());
    let client = client_for(&container).build();

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
    let docker = Cli::docker();
    let container = docker.run(SomfyMock::default());
    let client = client_for(&container).build();

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
    let docker = Cli::docker();
    let container = docker.run(SomfyMock::default());
    let client = client_for(&container).build();

    // should execute exchange password
    let sites = client.list_sites().await.unwrap();
    let site_ids: Vec<&String> = sites.iter().map(|s| &s.site_id).collect();
    assert_eq!(
        site_ids,
        vec!["Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT"],
        "expect 1 site id"
    );
    assert_mock_count("exchange-password-count", 1, &container).await;
    assert_mock_count("refresh-token-count", 0, &container).await;

    // should execute refresh token
    let sites = client.list_sites().await.unwrap();
    let site_ids: Vec<&String> = sites.iter().map(|s| &s.site_id).collect();
    assert_eq!(
        site_ids,
        vec!["Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT"],
        "expect 1 site id"
    );
    assert_mock_count("exchange-password-count", 1, &container).await;
    assert_mock_count("refresh-token-count", 1, &container).await;

    // should execute refresh token
    let sites = client.list_sites().await.unwrap();
    let site_ids: Vec<&String> = sites.iter().map(|s| &s.site_id).collect();
    assert_eq!(
        site_ids,
        vec!["Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT"],
        "expect 1 site id"
    );
    assert_mock_count("exchange-password-count", 1, &container).await;
    assert_mock_count("refresh-token-count", 2, &container).await;
}
