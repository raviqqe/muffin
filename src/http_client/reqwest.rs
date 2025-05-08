use super::{BareHttpClient, BareRequest, BareResponse, HttpClientError};
use async_trait::async_trait;
use reqwest::{Client, ClientBuilder, redirect::Policy};

/// An HTTP client based on [`reqwest`].
#[derive(Debug, Default)]
pub struct ReqwestHttpClient {
    client: Client,
}

impl ReqwestHttpClient {
    /// Creates an HTTP client.
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: ClientBuilder::new()
                .tcp_keepalive(None)
                .redirect(Policy::none())
                .build()?,
        })
    }
}

#[async_trait]
impl BareHttpClient for ReqwestHttpClient {
    async fn get(&self, request: &BareRequest) -> Result<BareResponse, HttpClientError> {
        let response = self
            .client
            .execute(
                self.client
                    .get(request.url.clone())
                    .headers(request.headers.clone())
                    .build()?,
            )
            .await?;

        Ok(BareResponse {
            url: response.url().clone(),
            status: response.status(),
            headers: response.headers().clone(),
            body: response.bytes().await?.to_vec(),
        })
    }
}

impl From<reqwest::Error> for HttpClientError {
    fn from(error: reqwest::Error) -> Self {
        Self::new(error.to_string())
    }
}
