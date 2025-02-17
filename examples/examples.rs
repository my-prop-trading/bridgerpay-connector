use bridgerpay_connector::rest::api_client::{RestApiClient, RestApiConfig};
use bridgerpay_connector::rest::{CreateCashierSessionRequest};
use std::time::Duration;
use tokio::time::Instant;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let config = ExampleApiConfig;
    let brand_api = RestApiClient::new(config);
    let instant = Instant::now();
    login(&brand_api).await;
    create_cashier_session(&brand_api).await;
    generate_checkout_widget(&brand_api).await;

    println!("elapsed time: {:?}", instant.elapsed());
}

pub async fn login(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client.login().await;

    println!("{:?}", resp)
}

pub async fn create_cashier_session(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client
        .create_cashier_session(CreateCashierSessionRequest {
            cashier_key: None,
            order_id: Uuid::new_v4().to_string(),
            currency: "USD".to_string(),
            country: "US".to_string(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn generate_checkout_widget(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client
        .generate_checkout_widget(CreateCashierSessionRequest {
            cashier_key: None,
            order_id: Uuid::new_v4().to_string(),
            currency: "USD".to_string(),
            country: "US".to_string(),
        })
        .await;

    println!("{:?}", resp)
}

pub struct ExampleApiConfig;

#[async_trait::async_trait]
impl RestApiConfig for ExampleApiConfig {
    async fn get_api_url(&self) -> String {
        "https://api.bridgerpay.com".to_string()
    }

    async fn get_api_key(&self) -> String {
        std::env::var("API_KEY").unwrap()
    }

    async fn get_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }

    async fn get_user_name(&self) -> String {
        std::env::var("USER_NAME").unwrap()
    }

    async fn get_password(&self) -> String {
        std::env::var("PASSWORD").unwrap()
    }

    async fn get_cashier_key(&self) -> String {
        std::env::var("CASHIER_KEY").unwrap()
    }
}
