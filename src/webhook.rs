use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub webhook: Webhook,
    pub data: WebhookData,
    pub meta: WebhookMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(rename = "type")]
    pub webhook_type: String,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum WebhookType {
    #[strum(to_string = "approved")]
    #[serde(rename = "approved")]
    Approved,
    #[strum(to_string = "declined")]
    #[serde(rename = "declined")]
    Declined,
    #[strum(to_string = "approved_on_hold")]
    #[serde(rename = "approved_on_hold")]
    ApprovedOnHold,
    #[strum(to_string = "Refunds")]
    #[serde(rename = "Refunds")]
    Refunds,
    #[strum(to_string = "PreAuth")]
    #[serde(rename = "PreAuth")]
    PreAuth,
    #[strum(to_string = "Capture")]
    #[serde(rename = "Capture")]
    Capture,
    #[strum(to_string = "Void")]
    #[serde(rename = "Void")]
    Void,
    #[strum(to_string = "Payout")]
    #[serde(rename = "Payout")]
    Payout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookData {
    pub order_id: String,
    pub psp_name: String,
    pub charge: Charge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Charge {
    #[serde(rename = "type")]
    pub charge_type: String,
    pub id: String,
    pub uuid: String,
    pub psp_order_id: String,
    pub attributes: Attributes,
    pub is_refundable: bool,
    pub refund_id: String,
    pub operation_type: String,
    pub deposit_source: String,
    pub is_recurring: bool,
    pub mid_type: String,
    pub cft_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    pub is3_d: bool,
    pub live_mode: bool,
    pub amount: f64,
    pub status: String,
    pub card_number: Option<String>,
    pub currency: String,
    pub payment_method: String,
    pub description: Option<String>,
    pub decline_code: Option<String>,
    pub decline_reason: Option<String>,
    pub reference_id: Option<String>,
    pub pos_terminal_id: Option<String>,
    pub cash_register_id: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub source: Source,
    pub card_masked_number: Option<String>,
    pub card_expiration: Option<String>,
    pub card_brand: Option<String>,
    pub card_holder_name: Option<String>,
    pub customer: Customer,
    pub credit_card_token: Option<String>,
    pub mid_alias: String,
    pub installment_details: String,
    pub is_declined_due_to_funds: bool,
    pub is_hard_decline: bool,
    pub wire_transfer_details: Option<String>,
    pub verifications: Verifications,
    pub crypto_currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub email: String,
    pub ip_address: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub zip_code: Option<String>,
    pub phone: Option<String>,
    pub extra_data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verifications {
    pub cavv: Option<String>,
    pub cavv_message: Option<String>,
    pub avs: Avs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Avs {
    pub result: Option<String>,
    pub zip_match: Option<String>,
    pub address_match: Option<String>,
    pub name_match: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookMeta {
    pub server_time: u64,
    pub server_timezone: String,
    pub api_version: String,
    pub payload: Option<String>,
    pub cashier_session_id: String,
    pub platform_id: Option<String>,
    pub tracking_id: Option<String>,
    pub affiliate_id: Option<String>,
}
