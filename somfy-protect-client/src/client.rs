use reqwest::{redirect::Policy, Client};
use reqwest_middleware::{ClientBuilder, Middleware};
use somfy_protect_openapi::{
    apis::{
        configuration::Configuration,
        device_api::{
            self, GetCompatibleDevicesError, GetCompatibleDevicesParams, GetInstalledDevicesError,
            GetInstalledDevicesParams,
        },
        site_api::{self, SiteGetListError},
        Error,
    },
    models::{DeviceCompatibility, DeviceOutput, SiteOutput},
};
use std::sync::Arc;

use crate::auth::SomfyAuthMiddlewareBuilder;

const API_BASE_URL: &str = "https://api.myfox.io/v3";
const AUTH_BASE_URL: &str = "https://accounts.somfy.com/oauth/oauth/v2";
const USER_AGENT: &str = "Somfy Protect";

#[derive(Default)]
pub struct SomfyProtectClientBuilder {
    auth_base_url: Option<String>,
    api_base_url: Option<String>,
    client_credentials: Option<(String, String)>,
    user_credentials: Option<(String, String)>,
    reqwest_middleware: Option<Arc<dyn Middleware>>,
}

impl SomfyProtectClientBuilder {
    pub fn with_auth_base_url(mut self, base_url: String) -> Self {
        self.auth_base_url = Some(base_url);
        self
    }

    pub fn with_api_base_url(mut self, base_url: String) -> Self {
        self.api_base_url = Some(base_url);
        self
    }

    pub fn with_client_credentials(mut self, client_id: String, client_secret: String) -> Self {
        self.client_credentials = Some((client_id, client_secret));
        self
    }

    pub fn with_user_credentials(mut self, username: String, password: String) -> Self {
        self.user_credentials = Some((username, password));
        self
    }

    pub fn enable_metrics(mut self, reqwest_middleware: Arc<dyn Middleware>) -> Self {
        self.reqwest_middleware = Some(reqwest_middleware);
        self
    }

    pub fn build(&self) -> SomfyProtectClient {
        let auth_base_url = self
            .auth_base_url
            .clone()
            .unwrap_or(AUTH_BASE_URL.to_string());
        let api_base_url = self
            .api_base_url
            .clone()
            .unwrap_or(API_BASE_URL.to_string());
        let (client_id, client_secret) = self.client_credentials.clone().expect(
            "ClientId and ClientSecret are required to authenticate with Somfy Protect API",
        );
        let (username, password) = self
            .user_credentials
            .clone()
            .expect("Username and password is required to authenticate with Somfy Protect API");

        let inner_client = Client::builder()
            .redirect(Policy::none())
            .build()
            .expect("an http client");
        let auth_middleware = SomfyAuthMiddlewareBuilder::default()
            .with_base_url(auth_base_url)
            .with_client_credentials(client_id, client_secret)
            .with_user_credentials(username, password)
            .build();
        let mut client = ClientBuilder::new(inner_client).with(auth_middleware);
        if let Some(reqwest_middleware) = self.reqwest_middleware.as_ref() {
            client = client.with_arc(reqwest_middleware.clone());
        }

        SomfyProtectClient {
            configuration: Configuration {
                client: client.build(),
                base_path: api_base_url,
                user_agent: Some(USER_AGENT.to_string()),
                ..Default::default()
            },
        }
    }
}
pub struct SomfyProtectClient {
    configuration: Configuration,
}

impl SomfyProtectClient {
    pub async fn list_sites(&self) -> Result<Vec<SiteOutput>, Error<SiteGetListError>> {
        let sites_response = site_api::site_get_list(&self.configuration).await?;
        Ok(sites_response.items)
    }

    pub async fn list_devices(
        &self,
        site_id: String,
    ) -> Result<Vec<DeviceOutput>, Error<GetInstalledDevicesError>> {
        let devices_response = device_api::get_installed_devices(
            &self.configuration,
            GetInstalledDevicesParams { site_id },
        )
        .await?;
        Ok(devices_response.items)
    }

    pub async fn list_compatible_devices(
        &self,
        site_id: String,
    ) -> Result<Vec<DeviceCompatibility>, Error<GetCompatibleDevicesError>> {
        let compatible_devices_response = device_api::get_compatible_devices(
            &self.configuration,
            GetCompatibleDevicesParams { site_id },
        )
        .await?;
        Ok(compatible_devices_response.items)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::{Method::GET, MockServer};
    use somfy_protect_openapi::apis::configuration::Configuration;

    use super::SomfyProtectClient;

    fn somfy_protect_client(server: &MockServer) -> SomfyProtectClient {
        SomfyProtectClient {
            configuration: Configuration {
                base_path: format!("{}/api", server.base_url()),
                ..Default::default()
            },
        }
    }

    #[tokio::test]
    async fn can_list_sites() {
        let server = MockServer::start();
        let list_sites_mock = server.mock(|when, then| {
            when.method(GET).path("/api/site");
            then.status(200)
                .body_from_file("tests/resources/list_sites.json");
        });

        let client = somfy_protect_client(&server);

        let sites = client
            .list_sites()
            .await
            .expect("a successful site listing");

        list_sites_mock.assert();
        assert_eq!(sites.len(), 1, "one site is fetched");
        assert_eq!(
            sites.get(0).unwrap().site_id,
            "Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string()
        );
        assert_eq!(sites.get(0).unwrap().name, Some("Hausalarm".to_string()));
    }

    #[tokio::test]
    async fn can_list_devices() {
        let server = MockServer::start();
        let list_devices_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api/site/Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT/device");
            then.status(200)
                .body_from_file("tests/resources/list_devices.json");
        });

        let client = somfy_protect_client(&server);

        let devices = client
            .list_devices("Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string())
            .await
            .expect("a successful device listing");

        list_devices_mock.assert();
        assert_eq!(devices.len(), 8, "expect 8 devices");
        for device in devices {
            assert_eq!(
                device.site_id,
                "Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string(),
                "expect consistent site_id"
            );
        }
    }

    #[tokio::test]
    async fn can_list_compatible_devices() {
        let server = MockServer::start();
        let list_compatible_devices_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api/site/Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT/device-compatible");
            then.status(200)
                .body_from_file("tests/resources/list_compatible_devices.json");
        });

        let client = somfy_protect_client(&server);

        let devices = client
            .list_compatible_devices("Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT".to_string())
            .await
            .expect("a successful compatible device listing");

        list_compatible_devices_mock.assert();
        assert_eq!(devices.len(), 15, "expect 15 device types");
    }
}
