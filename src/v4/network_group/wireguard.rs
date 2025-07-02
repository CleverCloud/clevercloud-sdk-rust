use core::{cmp, fmt, hash};
use std::{io, process::Command};

use base64::Engine;
use rand_core::{OsError, OsRng, TryCryptoRng};
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::Zeroizing;

static REDACTED: &str = "<REDACTED>";

// WIREGUARD PRIVATE KEY ERROR /////////////////////////////////////////////////

/// Error related to [`WireGuardPrivateKey`].
#[derive(Debug, thiserror::Error)]
pub enum WireGuardPrivateKeyError {
    #[error("failed to decode base64-encoded private key, {0}")]
    Base64(#[source] base64::DecodeError),
    #[error("private key is {0} bytes when it must be exactly 32 bytes")]
    InvalidLength(usize),
}

// WIREGUARD PRIVATE KEY ///////////////////////////////////////////////////////

/// WireGuard private key.
///
/// This is a thin wrapper around a Diffie-Hellman secret key as used by WireGuard.
///
/// * [`serde::Deserialize`] implementation based on the **base64-encoded string** representation
/// * redacted [`fmt::Debug`] and [`fmt::Display`] implementations to prevent
///   accidentally leaking the secret (e.g. in logs)
/// * [`Zeroize`](zeroize::Zeroize) on [`Drop`]
#[derive(Clone)]
pub struct WireGuardPrivateKey(Zeroizing<StaticSecret>);

impl WireGuardPrivateKey {
    /// Generates a new private key using the given random number generator.
    ///
    /// # Errors
    ///
    /// * If `rng` fails to produce 32 random bytes.
    pub fn new<R: ?Sized + TryCryptoRng>(rng: &mut R) -> Result<Self, R::Error> {
        let mut buf = [0; 32];
        match rng.try_fill_bytes(&mut buf) {
            Err(error) => Err(error),
            Ok(()) => Ok(Self(Zeroizing::new(StaticSecret::from(buf)))),
        }
    }

    /// Generates a new private key using operating system's random data source.
    ///
    /// # Errors
    ///
    /// * if [`OsRng`] fails to produce 32 random bytes.
    pub fn from_os_rng() -> Result<Self, OsError> {
        Self::new(&mut OsRng)
    }

    /// Creates a new private key from base64-encoded bytes.
    ///
    /// # Errors
    ///
    /// * If decoding the base64-encoded output fails
    /// * If decoded output is not 32 bytes long
    pub fn from_base64(
        input: &(impl ?Sized + AsRef<[u8]>),
    ) -> Result<Self, WireGuardPrivateKeyError> {
        match base64::engine::general_purpose::STANDARD.decode(input) {
            Err(error) => Err(WireGuardPrivateKeyError::Base64(error)),
            Ok(bytes) => match <[u8; 32]>::try_from(bytes) {
                Err(error) => Err(WireGuardPrivateKeyError::InvalidLength(error.len())),
                Ok(array) => Ok(Self(Zeroizing::new(StaticSecret::from(array)))),
            },
        }
    }

    /// Exposes the private key in a base64-encoded string.
    pub fn to_base64(&self) -> Zeroizing<String> {
        Zeroizing::new(base64::engine::general_purpose::STANDARD.encode(self.0.as_bytes()))
    }

    /// Generates a new private key using `wg genkey` command in a child process.
    ///
    /// Requires WireGuard command-line interface (`wg`).
    ///
    /// # Errors
    ///
    /// * If `wg` command fails
    /// * If decoding the base64-encoded output fails
    /// * If decoded output is not 32 bytes long
    pub fn from_wg_genkey() -> io::Result<Self> {
        match Command::new("wg").arg("genkey").output() {
            Err(error) => Err(error),
            Ok(output) => match Self::from_base64(output.stdout.trim_ascii()) {
                Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
                Ok(private_key) => Ok(private_key),
            },
        }
    }

    /// Returns the associated public key.
    pub fn public_key(&self) -> WireGuardPublicKey {
        WireGuardPublicKey(PublicKey::from(&*self.0))
    }
}

impl From<[u8; 32]> for WireGuardPrivateKey {
    fn from(value: [u8; 32]) -> Self {
        Self(Zeroizing::new(StaticSecret::from(value)))
    }
}

impl TryFrom<&str> for WireGuardPrivateKey {
    type Error = WireGuardPrivateKeyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_base64(value)
    }
}

impl<'de> Deserialize<'de> for WireGuardPrivateKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let base64_encoded = String::deserialize(deserializer)?;
        Self::from_base64(&base64_encoded).map_err(serde::de::Error::custom)
    }
}

// REDACTED implementations

impl fmt::Display for WireGuardPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(REDACTED, f)
    }
}

impl fmt::Debug for WireGuardPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(REDACTED, f)
    }
}

impl hash::Hash for WireGuardPrivateKey {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        REDACTED.hash(state);
    }
}

impl PartialEq for WireGuardPrivateKey {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for WireGuardPrivateKey {
    fn partial_cmp(&self, _other: &Self) -> Option<cmp::Ordering> {
        None
    }
}

impl Serialize for WireGuardPrivateKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        REDACTED.serialize(serializer)
    }
}

// WIREGUARD PUBLIC KEY ////////////////////////////////////////////////////////

/// WireGuard public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireGuardPublicKey(pub PublicKey);

impl Serialize for WireGuardPublicKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        base64::engine::general_purpose::STANDARD
            .encode(self.0)
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WireGuardPublicKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let base64_encoded = String::deserialize(deserializer)?;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&base64_encoded)
            .map_err(serde::de::Error::custom)?;
        let data = <[u8; 32]>::try_from(decoded).map_err(|error| {
            serde::de::Error::custom(format!(
                "invalid wireguard public key: length is {} bytes when it must be exactly 32 bytes",
                error.len()
            ))
        })?;
        Ok(Self(PublicKey::from(data)))
    }
}
