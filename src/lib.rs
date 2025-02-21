use crate::cipher::MessageCipher;
use base64::engine::general_purpose;
use base64::Engine;
use prost::Message;
use ring::hmac;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod cipher;
pub mod rest;
pub mod webhook;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckoutPayloadModel {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(string, tag = "2")]
    pub client_id: String,
    #[prost(string, tag = "3")]
    pub sign: String,
    #[prost(map = "string, string", tag = "4")]
    pub metadata: HashMap<String, String>,
}

impl CheckoutPayloadModel {
    pub fn encrypt(&self, key: &str) -> String {
        MessageCipher::encrypt(self, key)
    }

    pub fn try_decrypt(str: &str, key: &str) -> Result<CheckoutPayloadModel, String> {
        MessageCipher::decrypt(str, key)
    }
}

#[derive(Clone, Serialize)]
pub struct CheckoutSign {
    pub amount: f64,
    pub order_id: String,
    pub currency: String,
}

pub fn generate_sign<T: Serialize>(data: &T, key: &str) -> Result<String, String> {
    let data = serde_json::to_string(data);

    let Ok(data) = data else {
        return Err(format!("{}", data.unwrap_err()));
    };

    Ok(sign_str(&data, key))
}

fn sign_str(str: &str, key: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA512, key.as_bytes());
    let signature = hmac::sign(&key, str.as_bytes());

    general_purpose::STANDARD.encode(signature)
}
