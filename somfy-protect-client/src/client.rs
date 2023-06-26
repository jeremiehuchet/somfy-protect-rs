use std::{
    io::ErrorKind,
    sync::{Arc, RwLock},
};
use reqwest::ClientBuilder;
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

const API_BASE_URL: &str = "https://api.myfox.io/v3";
const AUTH_BASE_URL: &str = "https://accounts.somfy.com/oauth/oauth/v2";
const USER_AGENT: &str = "Somfy Protect";

pub struct SomfyProtectClient {
    configuration: Arc<RwLock<Configuration>>,
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
        }
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
