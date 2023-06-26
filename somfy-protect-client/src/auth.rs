use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use log::{debug, info, warn};
use oauth2::{
    AccessToken,  ClientId, ClientSecret,  RefreshToken,
    ResourceOwnerPassword, ResourceOwnerUsername,  TokenResponse, RequestTokenError,
};

use self::somfy_oauth2::{OAuth2TokenResponse, SomfyOAuth2Client};

const AUTH_BASE_URL: &str = "https://accounts.somfy.com/oauth/oauth/v2";
const AUTH_TIME_DRIFT: Duration = Duration::from_secs(30);

pub(crate) struct SomfyAuthManagerBuilder {
    base_url: Option<String>,
    client_credentials: Option<(ClientId, ClientSecret)>,
    user_credentials: Option<(ResourceOwnerUsername, ResourceOwnerPassword)>,
}

impl Default for SomfyAuthManagerBuilder {
    fn default() -> Self {
        Self {
            base_url: Some(AUTH_BASE_URL.to_string()),
            client_credentials: None,
            user_credentials: None,
        }
    }
}

impl SomfyAuthManagerBuilder {
    fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    fn with_client_credentials(mut self, client_id: String, client_secret: String) -> Self {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        self.client_credentials = Some((client_id, client_secret));
        self
    }

    fn with_user_credentials(mut self, username: String, password: String) -> Self {
        let username = ResourceOwnerUsername::new(username);
        let password = ResourceOwnerPassword::new(password);
        self.user_credentials = Some((username, password));
        self
    }

    fn build(self) -> SomfyAuthManager {
        let (client_id, client_secret) = self
            .client_credentials
            .expect("Client credentials are missing");
        let base_url = self.base_url.expect("Authentication base URL is missing");
        let oauth2_client = SomfyOAuth2Client::new(base_url, client_id, client_secret);
        let (username, password) = self.user_credentials.expect("User credentials are missing");
        let somfy_auth = SomfyAuth::new(oauth2_client.clone(), username, password);
        SomfyAuthManager::new(oauth2_client, somfy_auth)
    }
}

pub(crate) struct SomfyAuthManager {
    oauth2_client: SomfyOAuth2Client,
    auth: Arc<RwLock<SomfyAuth>>,
}

impl SomfyAuthManager {
    fn new(oauth2_client: SomfyOAuth2Client, somfy_auth: SomfyAuth) -> Self {
        let auth = Arc::new(RwLock::new(somfy_auth));
        SomfyAuthManager {
            oauth2_client,
            auth,
        }
    }

    async fn get_token(&self) -> Option<AccessToken> {
        let status = {
            self.auth.read().unwrap().status()
        };
         match status {
            TokenStatus::FullAutenticationRequired => self.exchange_password().await,
            TokenStatus::Active(access_token) => Some(access_token),
            TokenStatus::RefreshTokenRequired(refresh_token) => {
                match self.refresh_token(&refresh_token).await {
                    None => self.exchange_password().await,
                    some_token => some_token,
                }
            }
        }
    }

    async fn exchange_password(&self) -> Option<AccessToken> {
        debug!("exchange password");
        let mut auth_rw = self.auth.write().unwrap();
        let token_response = self
            .oauth2_client
            .exchange_password(&auth_rw.username, &auth_rw.password)
            .await;
        match token_response {
            Ok(token_response) => {
                auth_rw.tokens = Some(token_response.clone());
                auth_rw.expiration_instant = token_response
                    .expires_in()
                    .map(|exp| Instant::now() + exp - AUTH_TIME_DRIFT);
                Some(token_response.access_token().clone())
            }
            Err(error) => {
                warn!("Unable to exchange password: {error}");
                None
            }
        }
    }

    async fn refresh_token(&self, refresh_token: &RefreshToken) -> Option<AccessToken> {
        debug!("refresh token");
        let mut auth_rw = self.auth.write().unwrap();
        let token_response = self.oauth2_client.refresh_token(refresh_token).await;
        match token_response {
            Ok(token_response) => {
                auth_rw.tokens = Some(token_response.clone());
                auth_rw.expiration_instant = token_response
                    .expires_in()
                    .map(|exp| Instant::now() + exp - AUTH_TIME_DRIFT);
                Some(token_response.access_token().clone())
            }
            Err(error) => {
                warn!("Unable to refresh token: {error}");
                None
            }
        }
    }
}

#[derive(Debug)]
struct SomfyAuth {
    oauth2_client: SomfyOAuth2Client,
    username: ResourceOwnerUsername,
    password: ResourceOwnerPassword,
    tokens: Option<OAuth2TokenResponse>,
    expiration_instant: Option<Instant>,
}

#[derive(Debug)]
enum TokenStatus {
    FullAutenticationRequired,
    Active(AccessToken),
    RefreshTokenRequired(RefreshToken),
}

impl SomfyAuth {
    fn new(
        oauth2_client: SomfyOAuth2Client,
        username: ResourceOwnerUsername,
        password: ResourceOwnerPassword,
    ) -> Self {
        Self {
            oauth2_client,
            username,
            password,
            tokens: None,
            expiration_instant: None,
        }
    }

    fn status(&self) -> TokenStatus {
        match (self.tokens.as_ref(), self.expiration_instant) {
            (None, _) => TokenStatus::FullAutenticationRequired,
            (Some(_), None) => TokenStatus::FullAutenticationRequired,
            (Some(tokens), Some(expiration_instant)) => {

                if Instant::now() < expiration_instant {
                    TokenStatus::Active(tokens.access_token().clone())
                } else if let Some(refresh_token) = tokens.refresh_token() {
                    TokenStatus::RefreshTokenRequired(refresh_token.clone())
                } else {
                    TokenStatus::FullAutenticationRequired
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use httpmock::{Mock, MockServer};

    use crate::auth::SomfyAuthManagerBuilder;

    fn login_mock(server: &MockServer) -> Mock {
        server.mock(|when, then| {
            when.path("/token")
                .header("Authorization", "Basic Y2xpZW50X2lkOmNsaWVudF9zZWNyZXQ=")
                .body("grant_type=password&username=username&password=password");
            then.status(200).body(
                r#"
                {
                    "access_token": "access_token_from_login",
                    "token_type": "Bearer",
                    "expires_in": 1,
                    "refresh_token": "refresh_token_from_login"
                }
                "#,
            );
        })
    }

    fn refresh_mock(server: &MockServer) -> Mock {
        server.mock(|when, then| {
            when.path("/token")
                .header("Authorization", "Basic Y2xpZW50X2lkOmNsaWVudF9zZWNyZXQ=")
                .body("grant_type=refresh_token&refresh_token=refresh_token_from_login");
            then.status(200).body(
                r#"
                {
                    "access_token": "access_token_from_refresh",
                    "token_type": "Bearer",
                    "expires_in": 1,
                    "refresh_token": "refresh_token_from_refresh"
                }
                "#,
            );
        })
    }

    fn refresh_mock_forbidden(server: &MockServer) -> Mock {
        server.mock(|when, then| {
            when.path("/token")
                .header("Authorization", "Basic Y2xpZW50X2lkOmNsaWVudF9zZWNyZXQ=")
                .body("grant_type=refresh_token&refresh_token=refresh_token_from_login");
            then.status(401).body(
                r#"
                {
                    "error": "invalid_token",
                    "error_description": "The access token expired"
                }
                "#,
            );
        })
    }

    #[tokio::test]
    async fn can_exchange_password() {
        let server = MockServer::start();
        let login_endpoint = login_mock(&server);
        let auth_manager = SomfyAuthManagerBuilder::default()
            .with_base_url(server.base_url())
            .with_client_credentials("client_id".to_string(), "client_secret".to_string())
            .with_user_credentials("username".to_string(), "password".to_string())
            .build();

        let token = auth_manager.get_token().await.unwrap();

        assert_eq!(token.secret(), "access_token_from_login");
        login_endpoint.assert();
    }

    #[tokio::test]
    async fn can_refresh_token() {
        let server = MockServer::start();
        let login_endpoint = login_mock(&server);
        let refresh_endpoint = refresh_mock(&server);
        let auth_manager = SomfyAuthManagerBuilder::default()
            .with_base_url(server.base_url())
            .with_client_credentials("client_id".to_string(), "client_secret".to_string())
            .with_user_credentials("username".to_string(), "password".to_string())
            .build();

        auth_manager.get_token().await;
        // mocked token is valid 1s and implementation assumes clocks may drift by 30s,
        // so there is no need to wait, token will be considered expired.
        let token = auth_manager.get_token().await.unwrap();

        assert_eq!(token.secret(), "access_token_from_refresh");
        login_endpoint.assert();
        refresh_endpoint.assert();
    }

    #[tokio::test]
    async fn should_login_after_refresh_failure() {
        let server = MockServer::start();
        let login_endpoint = login_mock(&server);
        let refresh_endpoint = refresh_mock_forbidden(&server);
        let auth_manager = SomfyAuthManagerBuilder::default()
            .with_base_url(server.base_url())
            .with_client_credentials("client_id".to_string(), "client_secret".to_string())
            .with_user_credentials("username".to_string(), "password".to_string())
            .build();

        auth_manager.get_token().await;
        // mocked token is valid 1s and implementation assumes clocks may drift by 30s,
        // so there is no need to wait, token will be considered expired.
        let token = auth_manager.get_token().await.unwrap();

        assert_eq!(token.secret(), "access_token_from_login");
        login_endpoint.assert_hits(2);
        refresh_endpoint.assert_hits(1);
    }

    // TODO
    //#[tokio::test]
    //async fn should_not_retry_login_requests_with_invalid_credentials() {}
}

mod somfy_oauth2 {
    use oauth2::{
        basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
        AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, RefreshToken, RequestTokenError,
        ResourceOwnerPassword, ResourceOwnerUsername, StandardErrorResponse, StandardTokenResponse,
        TokenUrl,
    };

    pub(super) type OAuth2TokenResponse =
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
    pub(super) type OAuth2TokenError = RequestTokenError<
        oauth2::reqwest::Error<reqwest::Error>,
        StandardErrorResponse<BasicErrorResponseType>,
    >;

    #[derive(Clone,Debug)]
    pub(super) struct SomfyOAuth2Client {
        oauth2_client: BasicClient,
    }

    impl SomfyOAuth2Client {
        pub(super) fn new(
            base_url: String,
            client_id: ClientId,
            client_secret: ClientSecret,
        ) -> Self {
            let oauth2_client = BasicClient::new(
                client_id,
                Some(client_secret),
                AuthUrl::new(format!("{base_url}/auth"))
                    .expect("Invalid authorization endpoint URL"),
                Some(
                    TokenUrl::new(format!("{base_url}/token")).expect("Invalid token endpoint URL"),
                ),
            );
            Self { oauth2_client }
        }

        pub(super) async fn exchange_password(
            &self,
            username: &ResourceOwnerUsername,
            password: &ResourceOwnerPassword,
        ) -> Result<OAuth2TokenResponse, OAuth2TokenError> {
            self.oauth2_client
                .exchange_password(username, password)
                .request_async(oauth2::reqwest::async_http_client)
                .await
        }

        pub(super) async fn refresh_token(
            &self,
            refresh_token: &RefreshToken,
        ) -> Result<OAuth2TokenResponse, OAuth2TokenError> {
            self.oauth2_client
                .exchange_refresh_token(refresh_token)
                .request_async(oauth2::reqwest::async_http_client)
                .await
        }
    }
}
