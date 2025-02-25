use async_trait::async_trait;
use serde::Serialize;
use std::marker::PhantomData;

use crate::errors::CLIError;
use crate::ResponseBody;

/// Empty State controlling the requester
pub struct Empty;
/// WithConfig ensures requester is populated
pub struct WithConfig;

#[async_trait]
pub trait SendRequest {
    type Response;

    async fn send_request(
        &self,
        body: impl Serialize + Send + 'static,
    ) -> Result<Self::Response, CLIError>;
}

pub struct Requester<T> {
    api_key: String,
    url: String,
    phantom_data: PhantomData<T>,
}

impl<T> Requester<T> {
    pub fn new(api_key: String, url: String) -> Requester<WithConfig> {
        Requester {
            api_key,
            url,
            phantom_data: PhantomData::<WithConfig>,
        }
    }
}

#[async_trait]
impl SendRequest for Requester<WithConfig> {
    type Response = ResponseBody;

    async fn send_request(
        &self,
        body: impl Serialize + Send + 'static,
    ) -> Result<Self::Response, CLIError> {
        // Send Request
        let client = reqwest::Client::new();
        let res = client
            .post(&self.url)
            .header("X-API-TOKEN", &self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|_| CLIError::HTTP)?;

        println!("The status is {}", res.status());
        if res.status().is_client_error() || res.status().is_server_error() {
            return Err(CLIError::HTTP);
        }

        let res_body: ResponseBody = res.json().await.map_err(|_| CLIError::HTTP)?;
        Ok(res_body)
    }
}
