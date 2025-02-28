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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<CheckoutTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// his parameter serves as an supplementary security measure. It will be subsequently returned
    /// as an integral component of the transaction notification process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_lock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_lock: Option<bool>,
    /// The platform ID refers to the unique ID from the merchant's CRM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_languages_dropdown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum CheckoutTheme {
    #[strum(to_string = "dark")]
    #[serde(rename = "dark")]
    Dark,
    #[strum(to_string = "light")]
    #[serde(rename = "light")]
    Light,
    #[strum(to_string = "bright")]
    #[serde(rename = "bright")]
    Bright,
    #[strum(to_string = "transparent")]
    #[serde(rename = "transparent")]
    Transparent,
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
