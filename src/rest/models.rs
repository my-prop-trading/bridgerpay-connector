use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub response: ResponseModel,
    pub result: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub response: ResponseModel,
    pub result: LoginModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginModel {
    pub refresh_token: String,
    pub access_token: AccessTokenModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenModel {
    pub token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseModel {
    pub status: String,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCashierSessionRequest {
    /// The Cashier key refers to software-level credentials utilized for the purpose of identifying a merchant.
    pub cashier_key: Option<String>,
    /// The Order ID denotes the unique transaction identifier within the merchant's system.
    pub order_id: String,
    /// The currency of the payment transaction will be initiated is determined by adhering to ISO 4217 - Currency Codes (e.g., "USD," "CNY," or "EUR").
    pub currency: String,
    /// The transaction will be created in the country specified, following ISO 3166-1 - Country Codes (e.g., "US," "CN," or "BE").
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCashierSessionResponse {
    pub response: ResponseModel,
    pub result: CashierSessionModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashierSessionModel {
    pub cashier_token: String,
}
