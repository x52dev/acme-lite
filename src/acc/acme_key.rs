use eyre::WrapErr as _;
use pkcs8::{DecodePrivateKey as _, EncodePrivateKey as _};
use zeroize::Zeroizing;

#[derive(Debug, Clone)]
pub(crate) struct AcmeKey {
    /// Signing key for ACME API interactions.
    signing_key: p256::ecdsa::SigningKey,

    /// Key ID that is set once an ACME account is created.
    key_id: Option<String>,
}

impl AcmeKey {
    /// Constructs new ACME key with random signing key.
    pub(crate) fn new() -> AcmeKey {
        Self::from_key(crate::create_p256_key())
    }

    /// Constructs new ACME key from PEM-encoded signing key.
    ///
    /// No key ID is set.
    pub(crate) fn from_pem(pem: &str) -> eyre::Result<AcmeKey> {
        let pri_key = ecdsa::SigningKey::<p256::NistP256>::from_pkcs8_pem(pem)
            .context("Failed to read PEM")?;
        Ok(Self::from_key(pri_key))
    }

    /// Constructs new ACME key from signing key.
    ///
    /// No key ID is set.
    fn from_key(signing_key: p256::ecdsa::SigningKey) -> AcmeKey {
        AcmeKey {
            signing_key,
            key_id: None,
        }
    }

    /// Returns PEM-encoded signing key.
    pub(crate) fn to_pem(&self) -> eyre::Result<Zeroizing<String>> {
        self.signing_key
            .to_pkcs8_pem(pem::LineEnding::LF)
            .context("private_key_to_pem")
    }

    /// Returns signing key.
    pub(crate) fn signing_key(&self) -> &p256::ecdsa::SigningKey {
        &self.signing_key
    }

    /// Return key ID.
    ///
    /// # Panics
    ///
    /// Panics if key ID is not set.
    pub(crate) fn key_id(&self) -> &str {
        self.key_id.as_ref().unwrap()
    }

    /// Sets key ID.
    ///
    /// Overwrites any previously set value.
    pub(crate) fn set_key_id(&mut self, kid: String) {
        self.key_id = Some(kid)
    }
}
