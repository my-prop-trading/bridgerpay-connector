use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub response: ResponseModel,
    pub result: LoginResultModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResultModel {
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
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCashierSessionResponse {
    pub response: ResponseModel,
    pub result: CreateCashierSessionResultModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCashierSessionResultModel {
    pub cashier_token: String,
}
