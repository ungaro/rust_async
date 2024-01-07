use reqwest::{Error,IntoUrl, Method, Request, RequestBuilder, Response};
use std::fmt;
use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
//use backoff::Error::{Transient,Permanent};
pub struct Client {
    http_client: reqwest::Client,
    backoff: ExponentialBackoff,
}

/*
// Define a custom error type that can encapsulate different kinds of errors.
#[derive(Debug)]
pub enum ClientError {
    HttpError(ReqwestError),
    BackoffError(BackoffError<E>),
    // You can add more error types as needed.
}

impl From<ReqwestError> for ClientError {
    fn from(error: ReqwestError) -> Self {
        ClientError::HttpError(error)
    }
}

impl From<BackoffError<E>> for ClientError {
    fn from(error: BackoffError<E>) -> Self {
        ClientError::BackoffError(error)
    }
}

// Implement Display for ClientError for better error messages.
impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::HttpError(e) => write!(f, "HTTP error: {}", e),
            ClientError::BackoffError(e) => write!(f, "Backoff error: {:?}", e),
        }
    }
}
*/
impl Client {
    pub fn new() -> Result<Self, Error> {
        let http_client = reqwest::Client::builder()
            .user_agent("horo bot/1.0")
            .build()?;
        let backoff = ExponentialBackoff::default();
        Ok(Self { http_client, backoff })
    }

    pub fn with_backoff(mut self, backoff: ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.http_client.request(method, url)
    }

    pub async fn execute(&self, req: Request) -> Result<Response, Error> {
        let exec = || async {
            self.http_client
                .execute(req.try_clone().expect("Failed to clone the request"))
                .await
                .and_then(|r| r.error_for_status())
                .map_err(backoff::Error::Transient)
        };

        let backoff = ExponentialBackoffBuilder::new()
            .with_max_elapsed_time(Some(std::time::Duration::from_secs(60)))
            .build();

        backoff::future::retry(backoff, exec).await.map_err(|e| match e {
            backoff::Error::Permanent(e) | backoff::Error::Transient(e,_) => e,
        });
    }
}