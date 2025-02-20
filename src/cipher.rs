use base64::{engine::general_purpose, Engine};
use prost::Message;
use libaes::Cipher;
use sha2::{Digest, Sha512};

pub struct MessageCipher;

impl MessageCipher {
    pub fn encrypt_with_iv<T: Message>(src: &T, key: &str, iv: &[u8]) -> String {
        let mut prost_encoded = Vec::new();
        Message::encode(src, &mut prost_encoded).expect("Failed to encode");
        let data = AesCipher::encrypt_with_iv(&prost_encoded, key, iv);
        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    pub fn decrypt_with_iv<T: Message + Default>(
        src: &str,
        key: &str,
        iv: &[u8],
    ) -> Result<T, String> {
        let base64_decoded = &general_purpose::STANDARD.decode(src);

        if let Err(err) = base64_decoded {
            return Err(format!("{}", err));
        }

        let base64_decoded = base64_decoded.as_ref().unwrap();
        let decrypted = AesCipher::decrypt_with_iv(base64_decoded, key, iv);

        let Ok(decrypted) = decrypted else {
            return Err(decrypted.unwrap_err());
        };

        let prost_decoded: Result<T, prost::DecodeError> = Message::decode(&decrypted[..]);

        match prost_decoded {
            Err(err) => Err(format!("{}", err)),
            Ok(data) => Ok(data),
        }
    }

    pub fn encrypt<T: Message>(src: &T, key: &str) -> String {
        let mut prost_encoded = Vec::new();
        Message::encode(src, &mut prost_encoded).expect("Failed to encode");
        let data = AesCipher::encrypt(&prost_encoded, key);
        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    pub fn decrypt<T: Message + Default>(src: &str, key: &str) -> Result<T, String> {
        let base64_decoded = &general_purpose::STANDARD.decode(src);

        if let Err(err) = base64_decoded {
            return Err(format!("{}", err));
        }

        let base64_decoded = base64_decoded.as_ref().unwrap();
        let decrypted = AesCipher::decrypt(base64_decoded, key);

        let Ok(decrypted) = decrypted else {
            return Err(decrypted.unwrap_err());
        };

        let prost_decoded: Result<T, prost::DecodeError> = Message::decode(&decrypted[..]);

        match prost_decoded {
            Err(err) => Err(format!("{}", err)),
            Ok(data) => Ok(data),
        }
    }
}

pub struct AesCipher;

impl AesCipher {
    pub fn encrypt_with_iv(src: &[u8], key: &str, iv: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);
        let cipher = Cipher::new_192(&aes_key);

        cipher.cbc_encrypt(iv, src)
    }

    pub fn decrypt_with_iv(src: &[u8], key: &str, iv: &[u8]) -> Result<Vec<u8>, String> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        const KEY_LEN: usize = 24;

        if key_hash.len() < KEY_LEN {
            return Err(format!("Key hash len can't be less than {}", KEY_LEN));
        }

        let mut aes_key = [0; KEY_LEN];
        aes_key.copy_from_slice(&key_hash[..KEY_LEN]);

        let cipher = Cipher::new_192(&aes_key);
        let decrypted = cipher.cbc_decrypt(iv, src);

        Ok(decrypted)
    }

    pub fn encrypt(src: &[u8], key: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        let mut aes_key = [0; 24];
        aes_key.copy_from_slice(&key_hash[..24]);

        let mut iv = vec![0u8; 16];
        iv.copy_from_slice(&src[..16]);

        let cipher = Cipher::new_192(&aes_key);
        let mut encrypted = cipher.cbc_encrypt(&iv, src);

        let mut data: Vec<u8> = Vec::with_capacity(iv.len() + encrypted.len());
        data.append(&mut iv);
        data.append(&mut encrypted);

        data
    }

    pub fn decrypt(src: &[u8], key: &str) -> Result<Vec<u8>, String> {
        let mut hasher = Sha512::new();
        hasher.update(key);
        let key_hash = hasher.finalize();
        const KEY_LEN: usize = 24;

        if key_hash.len() < KEY_LEN {
            return Err(format!("Key hash len can't be less than {}", KEY_LEN));
        }

        let mut aes_key = [0; KEY_LEN];
        aes_key.copy_from_slice(&key_hash[..KEY_LEN]);

        const IV_LEN: usize = 16;

        if src.len() < IV_LEN {
            return Err(format!("Src array len can't be less than {}", IV_LEN));
        }

        let mut iv = vec![0u8; IV_LEN];
        iv.copy_from_slice(&src[..IV_LEN]);

        let cipher = Cipher::new_192(&aes_key);
        let decrypted = cipher.cbc_decrypt(&iv, &src[16..]);

        Ok(decrypted)
    }
}

