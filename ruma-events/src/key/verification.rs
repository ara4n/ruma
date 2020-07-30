//! Modules for events in the *m.key.verification* namespace.
//!
//! This module also contains types shared by events in its child namespaces.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

pub mod accept;
pub mod cancel;
pub mod key;
pub mod mac;
pub mod request;
pub mod start;

/// A hash algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum HashAlgorithm {
    /// The SHA256 hash algorithm.
    Sha256,
}

/// A key agreement protocol.
#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum KeyAgreementProtocol {
    /// The [Curve25519](https://cr.yp.to/ecdh.html) key agreement protocol.
    Curve25519,
    /// The Curve25519 key agreement protocol with check for public keys.
    Curve25519HkdfSha256,
}

/// A message authentication code algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum MessageAuthenticationCode {
    /// The HKDF-HMAC-SHA256 MAC.
    HkdfHmacSha256,
    /// The HMAC-SHA256 MAC.
    HmacSha256,
}

/// A Short Authentication String method.
#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ShortAuthenticationString {
    /// The decimal method.
    Decimal,

    /// The emoji method.
    Emoji,
}

/// A Short Authentication String (SAS) verification method.
#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString, Deserialize, Serialize)]
#[non_exhaustive]
pub enum VerificationMethod {
    /// The *m.sas.v1* verification method.
    #[serde(rename = "m.sas.v1")]
    #[strum(serialize = "m.sas.v1")]
    MSasV1,
}

#[cfg(test)]
mod test {
    use super::{KeyAgreementProtocol, MessageAuthenticationCode};

    use serde_json::{from_value as from_json_value, json};

    #[test]
    fn serialize_key_agreement() {
        let serialized =
            serde_json::to_string(&KeyAgreementProtocol::Curve25519HkdfSha256).unwrap();
        assert_eq!(serialized, "\"curve25519-hkdf-sha256\"");

        let deserialized: KeyAgreementProtocol = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, KeyAgreementProtocol::Curve25519HkdfSha256);
    }

    #[test]
    fn deserialize_mac_method() {
        let json = json!(["hkdf-hmac-sha256", "hmac-sha256"]);

        let deserialized: Vec<MessageAuthenticationCode> = from_json_value(json).unwrap();
        assert!(deserialized.contains(&MessageAuthenticationCode::HkdfHmacSha256));
    }

    #[test]
    fn serialize_mac_method() {
        let serialized = serde_json::to_string(&MessageAuthenticationCode::HkdfHmacSha256).unwrap();
        let deserialized: MessageAuthenticationCode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized, "\"hkdf-hmac-sha256\"");
        assert_eq!(deserialized, MessageAuthenticationCode::HkdfHmacSha256);

        let serialized = serde_json::to_string(&MessageAuthenticationCode::HmacSha256).unwrap();
        let deserialized: MessageAuthenticationCode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serialized, "\"hmac-sha256\"");
        assert_eq!(deserialized, MessageAuthenticationCode::HmacSha256);
    }
}
