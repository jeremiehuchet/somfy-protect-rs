use std::{
    io::ErrorKind,
    sync::{Arc, RwLock},
};

use oauth2::{
    basic::BasicClient, reqwest::Error as OAuth2Error, AuthUrl, ClientId, ClientSecret,
    HttpRequest, HttpResponse, ResourceOwnerPassword, ResourceOwnerUsername, TokenResponse,
    TokenUrl,
};
use reqwest::ClientBuilder;

use crate::{
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

const API_BASE_URL: &str = "https://api.myfox.io/v3";
const AUTH_BASE_URL: &str = "https://accounts.somfy.com/oauth/oauth/v2";
const USER_AGENT: &str = "Somfy Protect";

pub struct SomfyProtectClient {
    configuration: Arc<RwLock<Configuration>>,
    oauth2_client: BasicClient,
}

impl SomfyProtectClient {
    pub fn new(oauth_client_id: String, oauth_client_secret: String) -> Self {
        Self::new_with_base_url(
            API_BASE_URL.to_string(),
            AUTH_BASE_URL.to_string(),
            oauth_client_id,
            oauth_client_secret,
        )
    }

    pub fn new_with_base_url(
        api_base_url: String,
        auth_base_url: String,
        oauth_client_id: String,
        oauth_client_secret: String,
    ) -> Self {
        SomfyProtectClient {
            configuration: Arc::new(RwLock::new(Configuration {
                client: ClientBuilder::new()
                    .redirect(reqwest::redirect::Policy::none())
                    .build()
                    .expect("a valid http client configuration"),
                base_path: api_base_url,
                user_agent: Some(USER_AGENT.to_string()),
                ..Default::default()
            })),
            oauth2_client: BasicClient::new(
                ClientId::new(oauth_client_id),
                Some(ClientSecret::new(oauth_client_secret)),
                AuthUrl::new(format!("{auth_base_url}/auth"))
                    .expect("Invalid authorization endpoint URL"),
                Some(
                    TokenUrl::new(format!("{auth_base_url}/token"))
                        .expect("Invalid token endpoint URL"),
                ),
            ),
        }
    }

    pub async fn login(&self, username: String, password: String) -> Result<(), Error<LoginError>> {
        let token_response = self
            .oauth2_client
            .exchange_password(
                &ResourceOwnerUsername::new(username),
                &ResourceOwnerPassword::new(password),
            )
            .request_async(|req| self.execute_request_async(req))
            .await
            .map_err(|error| Error::Io(std::io::Error::new(ErrorKind::Other, error.to_string())))?;
        let mut configuration = self
            .configuration
            .write()
            .map_err(|error| Error::Io(std::io::Error::new(ErrorKind::Other, error.to_string())))?;
        configuration.oauth_access_token = Some(token_response.access_token().secret().to_string());
        Ok(())
    }

    async fn execute_request_async(
        &self,
        request: HttpRequest,
    ) -> Result<HttpResponse, OAuth2Error<reqwest::Error>> {
        let client = self
            .configuration
            .read()
            .map_err(|error| {
                OAuth2Error::Other(format!(
                    "Unable to acquire http configuration lock: {error}"
                ))
            })?
            .client
            .clone();
        let mut request_builder = client
            .request(request.method, request.url.as_str())
            .body(request.body);
        for (name, value) in &request.headers {
            request_builder = request_builder.header(name.as_str(), value.as_bytes());
        }
        let request = request_builder.build().map_err(OAuth2Error::Reqwest)?;

        let response = client
            .execute(request)
            .await
            .map_err(OAuth2Error::Reqwest)?;

        let status_code = response.status();
        let headers = response.headers().to_owned();
        let chunks = response.bytes().await.map_err(OAuth2Error::Reqwest)?;
        Ok(HttpResponse {
            status_code,
            headers,
            body: chunks.to_vec(),
        })
    }

    pub async fn list_sites(&self) -> Result<Vec<SiteOutput>, Error<SiteGetListError>> {
        let configuration = self
            .configuration
            .read()
            .map_err(|error| Error::Io(std::io::Error::new(ErrorKind::Other, error.to_string())))?;
        let sites_response = site_api::site_get_list(&configuration).await?;
        Ok(sites_response.items)
    }

    pub async fn list_devices(
        &self,
        site_id: String,
    ) -> Result<Vec<DeviceOutput>, Error<GetInstalledDevicesError>> {
        let configuration = self
            .configuration
            .read()
            .map_err(|error| Error::Io(std::io::Error::new(ErrorKind::Other, error.to_string())))?;
        let devices_response = device_api::get_installed_devices(
            &configuration,
            GetInstalledDevicesParams { site_id },
        )
        .await?;
        Ok(devices_response.items)
    }

    pub async fn list_compatible_devices(
        &self,
        site_id: String,
    ) -> Result<Vec<DeviceCompatibility>, Error<GetCompatibleDevicesError>> {
        let configuration = self
            .configuration
            .read()
            .map_err(|error| Error::Io(std::io::Error::new(ErrorKind::Other, error.to_string())))?;
        let compatible_devices_response = device_api::get_compatible_devices(
            &configuration,
            GetCompatibleDevicesParams { site_id },
        )
        .await?;
        Ok(compatible_devices_response.items)
    }
}

#[derive(Debug)]
pub struct LoginError {
    msg: String,
}

impl LoginError {
    fn new(msg: String) -> Self {
        LoginError { msg }
    }
}
