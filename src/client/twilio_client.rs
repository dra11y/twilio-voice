use std::sync::Arc;

use http::{Method, StatusCode, header::CONTENT_TYPE};
use reqwest::Body;

#[derive(Debug, Clone)]
pub struct TwilioClient {
    account_sid: Arc<str>,
    auth_token: Arc<str>,
    http_client: reqwest::Client,
}

fn url_encode(params: &[(&str, &str)]) -> String {
    let mut url = form_urlencoded::Serializer::new(String::new());
    for (k, v) in params {
        url.append_pair(k, v);
    }

    url.finish()
}

pub type TwilioClientResult<T> = Result<T, TwilioClientError>;

#[derive(Debug, thiserror::Error)]
pub enum TwilioClientError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("http error: {0}")]
    HTTPError(StatusCode),
    #[error("parsing error: {0}")]
    ParsingError(String),
    #[error("auth error")]
    AuthError,
    #[error("bad request")]
    BadRequest,
}

impl TwilioClient {
    pub fn new(account_sid: &str, auth_token: &str) -> TwilioClient {
        TwilioClient::new_with_client(
            account_sid,
            auth_token,
            reqwest::Client::builder()
                .http1_only()
                .https_only(true)
                .build()
                .expect("Client::new()"),
        )
    }

    pub fn new_with_client(
        account_sid: &str,
        auth_token: &str,
        http_client: reqwest::Client,
    ) -> TwilioClient {
        TwilioClient {
            account_sid: Arc::from(account_sid),
            auth_token: Arc::from(auth_token),
            http_client,
        }
    }

    pub async fn send_request<T>(
        &self,
        method: Method,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T, TwilioClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/{}.json",
            self.account_sid, endpoint
        );

        let resp = self
            .http_client
            .request(method, url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(url_encode(params)))
            .send()
            .await?;

        match resp.status() {
            StatusCode::CREATED | StatusCode::OK => {}
            status => return Err(TwilioClientError::HTTPError(status)),
        };

        Ok(resp.json().await?)
    }
}
