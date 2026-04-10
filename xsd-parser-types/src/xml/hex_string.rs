use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper,
    WithDeserializerFromBytes, WithSerializeToBytes,
};

/// A string representation of hex binary data (`xs:hexBinary`).
///
/// This type stores the hex-encoded string directly without decoding.
/// The `len()` method returns the byte length of the decoded data,
/// not the length of the hex string.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HexString(pub String);

impl HexString {
    /// Creates a new `HexString` from a string.
    #[must_use]
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Returns the byte length of the decoded data, not the length of the string.
    #[must_use]
    pub fn len(&self) -> usize {
        // Each pair of hex digits represents one byte
        self.0.len() / 2
    }

    /// Returns `true` if the hex string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the hex string as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Decodes the hex string into bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the hex string is invalid.
    #[cfg(feature = "hex")]
    pub fn decode(&self) -> Result<Vec<u8>, hex::FromHexError> {
        hex::decode(&self.0)
    }
}

impl Debug for HexString {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl Display for HexString {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<String> for HexString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<HexString> for String {
    fn from(h: HexString) -> Self {
        h.0
    }
}

impl AsRef<str> for HexString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "quick-xml")]
impl DeserializeBytes for HexString {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let _helper = helper;
        let s = std::str::from_utf8(bytes).map_err(Error::from)?.to_owned();
        Ok(HexString(s))
    }
}

#[cfg(feature = "quick-xml")]
impl WithDeserializerFromBytes for HexString {}

#[cfg(feature = "quick-xml")]
impl SerializeBytes for HexString {
    fn serialize_bytes(
        &self,
        helper: &mut SerializeHelper,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, Error> {
        let _helper = helper;
        Ok(Some(std::borrow::Cow::Borrowed(&self.0)))
    }
}

#[cfg(feature = "quick-xml")]
impl WithSerializeToBytes for HexString {}
