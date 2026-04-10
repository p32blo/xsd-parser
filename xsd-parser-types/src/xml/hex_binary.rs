use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::Deref;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
    WithDeserializerFromBytes, WithSerializeToBytes,
};

/// A binary representation of hex binary data (`xs:hexBinary`).
///
/// This type stores the decoded binary data directly as `Vec<u8>`.
/// It implements `Deref` to allow transparent access to the underlying `Vec<u8>`.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexBinary(pub Vec<u8>);

impl HexBinary {
    /// Creates a new `HexBinary` from a vector of bytes.
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Creates a new `HexBinary` from a hex-encoded string.
    ///
    /// # Errors
    ///
    /// Returns an error if the hex string is invalid.
    #[cfg(feature = "hex")]
    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        Ok(Self(hex::decode(s)?))
    }

    /// Encodes the binary data as a hex string.
    #[cfg(feature = "hex")]
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

impl Deref for HexBinary {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for HexBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "HexBinary({} bytes)", self.0.len())
    }
}

impl From<Vec<u8>> for HexBinary {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl From<HexBinary> for Vec<u8> {
    fn from(h: HexBinary) -> Self {
        h.0
    }
}

impl AsRef<[u8]> for HexBinary {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "quick-xml")]
impl DeserializeBytes for HexBinary {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let _helper = helper;
        let s = std::str::from_utf8(bytes).map_err(Error::from)?;

        hex::decode(s).map(HexBinary).map_err(Error::custom)
    }
}

#[cfg(feature = "quick-xml")]
impl WithDeserializerFromBytes for HexBinary {}

#[cfg(feature = "quick-xml")]
impl SerializeBytes for HexBinary {
    fn serialize_bytes(
        &self,
        helper: &mut SerializeHelper,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, Error> {
        let _helper = helper;
        Ok(Some(std::borrow::Cow::Owned(hex::encode(&self.0))))
    }
}

#[cfg(feature = "quick-xml")]
impl WithSerializeToBytes for HexBinary {}
