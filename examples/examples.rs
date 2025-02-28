use bridgerpay_connector::rest::api_client::{CheckoutWidgetType, RestApiClient, RestApiConfig};
use bridgerpay_connector::rest::CreateCashierSessionRequest;
use bridgerpay_connector::{generate_sign, CheckoutPayloadModel, CheckoutSign};
use std::collections::HashMap;
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

pub fn create_cashier_session_req() -> CreateCashierSessionRequest {
    let order_id = Uuid::new_v4().to_string();
    let amount = 10.0;
    let currency = "USD".to_string();

    CreateCashierSessionRequest {
        cashier_key: None,
        order_id: order_id.clone(),
        currency: currency.clone(),
        country: "US".to_string(),
        amount: Some(amount),
        theme: None,
        first_name: Some(String::from("John Smith")),
        last_name: Some(String::from("Doe")),
        phone: Some("38506466464".to_string()),
        email: Some("test1234@mailinator.com".to_string()),
        zip_code: Some(String::from("14900")),
        payload: Some(
            CheckoutPayloadModel {
                timestamp: 123,
                client_id: "test-client-id".to_string(),
                sign: generate_sign(
                    &CheckoutSign {
                        amount,
                        order_id,
                        currency,
                    },
                    &std::env::var("API_KEY").unwrap(),
                )
                .unwrap(),
                metadata: HashMap::from([("test".to_string(), "test".to_string())]),
            }
            .encrypt(&std::env::var("API_KEY").unwrap()),
        ),
        currency_lock: Some(true),
        amount_lock: Some(true),
        platform_id: None,
        tracking_id: None,
        affiliate_id: None,
        city: Some("Sofia".to_string()),
        address: Some("Address".to_string()),
        state: Some("Alabama".to_string()),
    }
}

pub async fn login(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client.login().await;

    println!("{:?}", resp)
}

pub async fn create_cashier_session(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client
        .create_cashier_session(create_cashier_session_req())
        .await;

    println!("{:?}", resp)
}

pub async fn generate_checkout_widget(rest_client: &RestApiClient<ExampleApiConfig>) {
    let resp = rest_client
        .generate_checkout_widget(create_cashier_session_req(), CheckoutWidgetType::Wrapped)
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
