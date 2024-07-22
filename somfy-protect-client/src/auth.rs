use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use async_trait::async_trait;
use http::Extensions;
use log::{debug, warn};
use oauth2::{
    ClientId, ClientSecret, RefreshToken, RequestTokenError, ResourceOwnerPassword,
    ResourceOwnerUsername, TokenResponse,
};
use reqwest::{header::AUTHORIZATION, Client, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next, Result};

use self::somfy_oauth2::{OAuth2TokenResponse, SomfyOAuth2Client};

const AUTH_BASE_URL: &str = "https://accounts.somfy.com/oauth/oauth/v2";
const AUTH_TIME_DRIFT: Duration = Duration::from_secs(30);

pub(crate) struct SomfyAuthMiddlewareBuilder {
    http_client: Option<ClientWithMiddleware>,
    base_url: Option<String>,
    client_credentials: Option<(ClientId, ClientSecret)>,
    user_credentials: Option<(ResourceOwnerUsername, ResourceOwnerPassword)>,
}

impl Default for SomfyAuthMiddlewareBuilder {
    fn default() -> Self {
        Self {
            http_client: None,
            base_url: Some(AUTH_BASE_URL.to_string()),
            client_credentials: None,
            user_credentials: None,
        }
    }
}

impl SomfyAuthMiddlewareBuilder {
    pub(crate) fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub(crate) fn with_client_credentials(
        mut self,
        client_id: String,
        client_secret: String,
    ) -> Self {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        self.client_credentials = Some((client_id, client_secret));
        self
    }

    pub(crate) fn with_user_credentials(mut self, username: String, password: String) -> Self {
        let username = ResourceOwnerUsername::new(username);
        let password = ResourceOwnerPassword::new(password);
        self.user_credentials = Some((username, password));
        self
    }

    pub(crate) fn build(self) -> SomfyAuthMiddleware {
        let http_client = self
            .http_client
            .unwrap_or_else(|| ClientBuilder::new(Client::default()).build());
        let (client_id, client_secret) = self
            .client_credentials
            .expect("Client credentials are missing");
        let base_url = self.base_url.expect("Authentication base URL is missing");
        let oauth2_client = SomfyOAuth2Client::new(http_client, base_url, client_id, client_secret);
        let (username, password) = self.user_credentials.expect("User credentials are missing");
        let somfy_auth = SomfyAuth::new(username, password);
        SomfyAuthMiddleware {
            oauth2_client,
            auth: Arc::new(RwLock::new(somfy_auth)),
        }
    }
}

#[derive(Clone)]
pub(crate) struct SomfyAuthMiddleware {
    oauth2_client: SomfyOAuth2Client,
    auth: Arc<RwLock<SomfyAuth>>,
}

#[async_trait]
impl Middleware for SomfyAuthMiddleware {
    async fn handle(
        &self,
        request: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let status = { self.auth.read().unwrap().status() };

        let token = match status {
            TokenStatus::Active(access_token) => Ok(access_token),
            TokenStatus::FullAutenticationRequired(username, password) => {
                debug!("Execute exchange password");
                self.oauth2_client
                    .exchange_password(&username, &password)
                    .await
            }
            TokenStatus::RefreshTokenRequired(refresh_token) => {
                debug!("Execute refresh token");
                let refresh_respone = self.oauth2_client.refresh_token(&refresh_token).await;
                match refresh_respone {
                    Ok(token_response) => Ok(token_response),
                    Err(err) => match err {
                        RequestTokenError::ServerResponse(details) => {
                            debug!("Refresh token request rejected ({details}), attempt to execute an exchange password");
                            let (username, password) = {
                                let auth = self.auth.read().unwrap();
                                (auth.username.clone(), auth.password.clone())
                            };
                            self.oauth2_client
                                .exchange_password(&username, &password)
                                .await
                        }
                        _ => Err(err),
                    },
                }
            }
        };
        match token {
            Ok(token) => {
                {
                    let mut auth_rw = self.auth.write().unwrap();
                    auth_rw.update(&token);
                }
                let access_token = token.access_token();
                let mut request = request
                    .try_clone()
                    .expect("Request body should be clonable (it should not be a stream)");
                request.headers_mut().insert(
                    AUTHORIZATION,
                    format!("Bearer {}", access_token.secret()).parse().unwrap(),
                );
                next.run(request, extensions).await
            }
            Err(error) => {
                warn!("Unable to login to Somfy Protect API: {error}");
                next.run(request, extensions).await
            }
        }
    }
}

#[derive(Debug)]
struct SomfyAuth {
    username: ResourceOwnerUsername,
    password: ResourceOwnerPassword,
    tokens: Option<OAuth2TokenResponse>,
    expiration_instant: Option<Instant>,
}

#[derive(Debug)]
enum TokenStatus {
    FullAutenticationRequired(ResourceOwnerUsername, ResourceOwnerPassword),
    Active(OAuth2TokenResponse),
    RefreshTokenRequired(RefreshToken),
}

impl SomfyAuth {
    fn new(username: ResourceOwnerUsername, password: ResourceOwnerPassword) -> Self {
        Self {
            username,
            password,
            tokens: None,
            expiration_instant: None,
        }
    }

    fn update(&mut self, token_response: &OAuth2TokenResponse) {
        self.expiration_instant = token_response
            .expires_in()
            .map(|exp| Instant::now() + exp - AUTH_TIME_DRIFT);
        self.tokens = Some(token_response.clone());
    }

    fn status(&self) -> TokenStatus {
        match (self.tokens.as_ref(), self.expiration_instant) {
            (None, _) => {
                TokenStatus::FullAutenticationRequired(self.username.clone(), self.password.clone())
            }
            (Some(_), None) => {
                TokenStatus::FullAutenticationRequired(self.username.clone(), self.password.clone())
            }
            (Some(tokens), Some(expiration_instant)) => {
                if Instant::now() < expiration_instant {
                    TokenStatus::Active(tokens.clone())
                } else if let Some(refresh_token) = tokens.refresh_token() {
                    TokenStatus::RefreshTokenRequired(refresh_token.clone())
                } else {
                    TokenStatus::FullAutenticationRequired(
                        self.username.clone(),
                        self.password.clone(),
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use httpmock::{Mock, MockServer};
    use reqwest::Client;
    use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

    use crate::auth::SomfyAuthMiddlewareBuilder;

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

    fn target_endpoint_with_bearer_token(server: &MockServer, token: String) -> Mock {
        server.mock(|when, then| {
            when.path("/target")
                .header("Authorization", format!("Bearer {token}"));
            then.status(200);
        })
    }

    fn http_client_with_auth_middleware(base_url: String) -> ClientWithMiddleware {
        let auth_manager = SomfyAuthMiddlewareBuilder::default()
            .with_base_url(base_url)
            .with_client_credentials("client_id".to_string(), "client_secret".to_string())
            .with_user_credentials("username".to_string(), "password".to_string())
            .build();
        ClientBuilder::new(Client::default())
            .with(auth_manager)
            .build()
    }

    #[tokio::test]
    async fn can_exchange_password() {
        let server = MockServer::start();
        let target_endpoint =
            target_endpoint_with_bearer_token(&server, "access_token_from_login".to_string());
        let login_endpoint = login_mock(&server);
        let client = http_client_with_auth_middleware(server.base_url());

        let req = client
            .get(format!("{}/target", server.base_url()))
            .build()
            .unwrap();
        client.execute(req).await.unwrap();

        login_endpoint.assert();
        target_endpoint.assert();
    }

    #[tokio::test]
    async fn can_refresh_token() {
        let server = MockServer::start();
        let target_endpoint_intial_login =
            target_endpoint_with_bearer_token(&server, "access_token_from_login".to_string());
        let target_endpoint_after_refresh =
            target_endpoint_with_bearer_token(&server, "access_token_from_refresh".to_string());
        let login_endpoint = login_mock(&server);
        let refresh_endpoint = refresh_mock(&server);
        let client = http_client_with_auth_middleware(server.base_url());

        // first request should trigger an exchange_password:
        let req = client
            .get(format!("{}/target", server.base_url()))
            .build()
            .unwrap();
        client.execute(req.try_clone().unwrap()).await.unwrap();
        // mocked token is valid 1s and implementation assumes clocks may drift by 30s,
        // so there is no need to wait, token will be considered expired.
        // second request should trigger a refresh_token:
        client.execute(req).await.unwrap();

        login_endpoint.assert();
        refresh_endpoint.assert();
        target_endpoint_intial_login.assert();
        target_endpoint_after_refresh.assert();
    }

    #[tokio::test]
    async fn should_login_after_refresh_failure() {
        let server = MockServer::start();
        let target_endpoint =
            target_endpoint_with_bearer_token(&server, "access_token_from_login".to_string());
        let login_endpoint = login_mock(&server);
        let refresh_forbidden_endpoint = refresh_mock_forbidden(&server);
        let client = http_client_with_auth_middleware(server.base_url());

        // first request should trigger an exchange_password:
        let req = client
            .get(format!("{}/target", server.base_url()))
            .build()
            .unwrap();
        client.execute(req.try_clone().unwrap()).await.unwrap();
        // mocked token is valid 1s and implementation assumes clocks may drift by 30s,
        // so there is no need to wait, token will be considered expired.
        // second request should trigger a refresh_token:
        client.execute(req).await.unwrap();

        login_endpoint.assert_hits(2);
        refresh_forbidden_endpoint.assert_hits(1);
        target_endpoint.assert_hits(2);
    }
}

mod somfy_oauth2 {

    use std::str::FromStr;

    use oauth2::{
        basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
        http::StatusCode,
        AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, HttpRequest, HttpResponse,
        RefreshToken, RequestTokenError, ResourceOwnerPassword, ResourceOwnerUsername,
        StandardErrorResponse, StandardTokenResponse, TokenUrl,
    };
    use reqwest_middleware::ClientWithMiddleware;

    pub(super) type OAuth2TokenResponse =
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
    pub(super) type OAuth2TokenError = RequestTokenError<
        oauth2::reqwest::Error<reqwest::Error>,
        StandardErrorResponse<BasicErrorResponseType>,
    >;

    #[derive(Clone, Debug)]
    pub(super) struct SomfyOAuth2Client {
        http_client: ClientWithMiddleware,
        oauth2_client: BasicClient,
    }

    impl SomfyOAuth2Client {
        pub(super) fn new(
            http_client: ClientWithMiddleware,
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
            Self {
                http_client,
                oauth2_client,
            }
        }

        pub(super) async fn exchange_password(
            &self,
            username: &ResourceOwnerUsername,
            password: &ResourceOwnerPassword,
        ) -> anyhow::Result<OAuth2TokenResponse, OAuth2TokenError> {
            self.oauth2_client
                .exchange_password(username, password)
                .request_async(|request| self.async_http_client(request))
                .await
        }

        pub(super) async fn refresh_token(
            &self,
            refresh_token: &RefreshToken,
        ) -> Result<OAuth2TokenResponse, OAuth2TokenError> {
            self.oauth2_client
                .exchange_refresh_token(refresh_token)
                .request_async(|request| self.async_http_client(request))
                .await
        }

        async fn async_http_client(
            &self,
            request: HttpRequest,
        ) -> Result<HttpResponse, oauth2::reqwest::Error<reqwest::Error>> {
            let client = self.http_client.clone();

            let method = http::Method::from_bytes(request.method.to_string().as_bytes())
                .expect("a valid HTTP verb");

            let mut request_builder = client
                .request(method, request.url.as_str())
                .body(request.body);
            for (name, value) in &request.headers {
                request_builder = request_builder.header(name.as_str(), value.as_bytes());
            }
            let request = request_builder
                .build()
                .map_err(oauth2::reqwest::Error::Reqwest)?;

            let response = client.execute(request).await.map_err(|error| match error {
                reqwest_middleware::Error::Middleware(error) => {
                    oauth2::reqwest::Error::Other(error.to_string())
                }
                reqwest_middleware::Error::Reqwest(error) => oauth2::reqwest::Error::Reqwest(error),
            })?;

            let status_code = response.status();
            let mut headers = oauth2::http::HeaderMap::new();
            for (name, value) in response.headers() {
                let name = oauth2::http::HeaderName::from_str(name.as_str())
                    .expect("a valid HTTP header name");
                let value = oauth2::http::HeaderValue::from_bytes(value.as_bytes())
                    .expect("a valid HTTP header value");
                headers.append(name, value);
            }
            let chunks = response
                .bytes()
                .await
                .map_err(oauth2::reqwest::Error::Reqwest)?;
            Ok(HttpResponse {
                status_code: StatusCode::from_u16(status_code.as_u16())
                    .expect("a valid HTTP status code"),
                headers,
                body: chunks.to_vec(),
            })
        }
    }
}
