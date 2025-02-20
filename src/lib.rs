use std::collections::HashMap;
use crate::cipher::MessageCipher;

pub mod cipher;
pub mod rest;
pub mod webhook;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckoutPayloadModel {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(map = "string, string", tag = "2")]
    pub key_values: HashMap<String, String>,
}

impl CheckoutPayloadModel {
    pub fn encrypt(&self, key: &str) -> String {
        MessageCipher::encrypt(self, key)
    }

    pub fn try_decrypt(str: &str, key: &str) -> Result<CheckoutPayloadModel, String> {
        MessageCipher::decrypt(str, key)
    }
}
