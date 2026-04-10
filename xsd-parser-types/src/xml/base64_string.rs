use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
    WithDeserializerFromBytes, WithSerializeToBytes,
};

/// A string representation of base64 binary data (`xs:base64Binary`).
///
/// This type stores the base64-encoded string directly without decoding.
/// The `len()` method returns the byte length of the decoded data,
/// not the length of the base64 string.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Base64String(pub String);

impl Base64String {
    /// Creates a new `Base64String` from a string.
    #[must_use]
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Returns the byte length of the decoded data, not the length of the string.
    #[must_use]
    pub fn len(&self) -> usize {
        // Calculate the decoded length from base64 string
        // Base64 encodes 3 bytes into 4 characters
        let len = self.0.trim_end_matches('=').len();
        (len * 3) / 4
    }

    /// Returns `true` if the base64 string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the base64 string as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Decodes the base64 string into bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the base64 string is invalid.
    #[cfg(feature = "base64")]
    pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(&self.0)
    }
}

impl Debug for Base64String {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Base64String {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<String> for Base64String {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<Base64String> for String {
    fn from(b: Base64String) -> Self {
        b.0
    }
}

impl AsRef<str> for Base64String {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "quick-xml")]
impl DeserializeBytes for Base64String {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let _helper = helper;
        let s = std::str::from_utf8(bytes).map_err(Error::from)?.to_owned();
        Ok(Base64String(s))
    }
}

#[cfg(feature = "quick-xml")]
impl WithDeserializerFromBytes for Base64String {}

#[cfg(feature = "quick-xml")]
impl SerializeBytes for Base64String {
    fn serialize_bytes(
        &self,
        helper: &mut SerializeHelper,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, Error> {
        let _helper = helper;
        Ok(Some(std::borrow::Cow::Borrowed(&self.0)))
    }
}

#[cfg(feature = "quick-xml")]
impl WithSerializeToBytes for Base64String {}
