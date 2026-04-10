use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::Deref;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
    WithDeserializerFromBytes, WithSerializeToBytes,
};

/// A binary representation of base64 binary data (`xs:base64Binary`).
///
/// This type stores the decoded binary data directly as `Vec<u8>`.
/// It implements `Deref` to allow transparent access to the underlying `Vec<u8>`.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Base64Binary(pub Vec<u8>);

impl Base64Binary {
    /// Creates a new `Base64Binary` from a vector of bytes.
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Creates a new `Base64Binary` from a base64-encoded string.
    ///
    /// # Errors
    ///
    /// Returns an error if the base64 string is invalid.
    #[cfg(feature = "base64")]
    pub fn from_base64(s: &str) -> Result<Self, base64::DecodeError> {
        use base64::Engine;
        Ok(Self(base64::engine::general_purpose::STANDARD.decode(s)?))
    }

    /// Encodes the binary data as a base64 string.
    #[cfg(feature = "base64")]
    #[must_use]
    pub fn to_base64(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(&self.0)
    }
}

impl Deref for Base64Binary {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Base64Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Base64Binary({} bytes)", self.0.len())
    }
}

impl From<Vec<u8>> for Base64Binary {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl From<Base64Binary> for Vec<u8> {
    fn from(b: Base64Binary) -> Self {
        b.0
    }
}

impl AsRef<[u8]> for Base64Binary {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "quick-xml")]
impl DeserializeBytes for Base64Binary {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let _helper = helper;
        let s = std::str::from_utf8(bytes).map_err(Error::from)?;

        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(s)
            .map(Base64Binary)
            .map_err(Error::custom)
    }
}

#[cfg(feature = "quick-xml")]
impl WithDeserializerFromBytes for Base64Binary {}

#[cfg(feature = "quick-xml")]
impl SerializeBytes for Base64Binary {
    fn serialize_bytes(
        &self,
        helper: &mut SerializeHelper,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, Error> {
        let _helper = helper;
        use base64::Engine;
        Ok(Some(std::borrow::Cow::Owned(
            base64::engine::general_purpose::STANDARD.encode(&self.0),
        )))
    }
}

#[cfg(feature = "quick-xml")]
impl WithSerializeToBytes for Base64Binary {}
