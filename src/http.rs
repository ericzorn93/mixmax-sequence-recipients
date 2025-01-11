use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use std::marker::PhantomData;

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
    ) -> Result<Self::Response, Box<dyn std::error::Error + Send + Sync>>;
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
    type Response = Value;

    async fn send_request(
        &self,
        body: impl Serialize + Send + 'static,
    ) -> Result<Self::Response, Box<dyn std::error::Error + Send + Sync>> {
        // Send Request
        let client = reqwest::Client::new();
        let res = client
            .post(&self.url)
            .header("X-API-TOKEN", &self.api_key)
            .json(&body)
            .send()
            .await?;

        println!("The status is {}", res.status());

        let res_body: Value = res.json().await?;
        Ok(res_body)
    }
}
