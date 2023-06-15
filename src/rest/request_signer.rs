use hmac::{Hmac, Mac};
use sha2::Sha512;

#[derive(Clone)]
pub struct RequestSigner {
    private_key: String,
}

impl RequestSigner {
    pub fn new(private_key: String) -> Self {
        Self { private_key }
    }

    pub fn generate_sign(&self, request_json: &str) -> String {
        let mut signed_key = Hmac::<Sha512>::new_from_slice(self.private_key.as_bytes()).unwrap();
        signed_key.update(request_json.as_bytes());

        hex::encode(signed_key.finalize().into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sign() {
        let request = r#"{"currency":"BTC","foreign_id":"123456"}"#;
        let key = "AbCdEfG123456".to_string();
        let client = RequestSigner::new(key.clone());

        let sign = client.generate_sign(request);

        let source_sign = "03c25fcf7cd35e7d995e402cd5d51edd72d48e1471e865907967809a0c189ba55b90815f20e2bb10f82c7a9e9d865546fda58989c2ae9e8e2ff7bc29195fa1ec";
        assert_eq!(source_sign, sign);
    }
}
