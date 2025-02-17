use bridgerpay_connector::rest::api_client::{RestApiClient, RestApiConfig};
use bridgerpay_connector::rest::LoginRequest;
use std::time::Duration;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("API_KEY").unwrap();
    let config = ExampleApiConfig {
        api_url: "https://api.bridgerpay.com".to_string(),
        api_key,
    };
    let brand_api = RestApiClient::new(config);
    let instant = Instant::now();
    login(&brand_api).await;

    println!("elapsed time: {:?}", instant.elapsed());
}

pub async fn login(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client
        .login(&LoginRequest {
            user_name: std::env::var("USER_NAME").unwrap(),
            password: std::env::var("PASSWORD").unwrap(),
        })
        .await;

    println!("{:?}", resp)
}

pub struct ExampleApiConfig {
    pub api_url: String,
    pub api_key: String,
}

#[async_trait::async_trait]
impl RestApiConfig for ExampleApiConfig {
    async fn get_api_url(&self) -> String {
        self.api_url.clone()
    }

    async fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    async fn get_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }
}
