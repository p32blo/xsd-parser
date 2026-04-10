//! The [`xml`](self) module contains different types to store unstructured XML
//! data. This is useful to represents `xs:any` and `xs:anyAttribute` information
//! from the XML schema.

mod any_simple_type;
mod attributes;
mod base64_binary;
mod base64_string;
mod element;
mod hex_binary;
mod hex_string;
mod mixed;
mod namespace_scope;
mod namespaces;
mod nillable;
mod qname;
mod text;
mod value;

pub use self::any_simple_type::{AnySimpleType, Base64Binary, Decimal, Integer, Unsigned};
pub use self::attributes::{
    AnyAttributes, Attributes, Key as AttributeKey, Value as AttributeValue,
};
pub use self::base64_binary::Base64Binary as Base64BinaryBytes;
pub use self::base64_string::Base64String;
pub use self::element::{AnyElement, AnyElements, Element, Elements};
pub use self::hex_binary::HexBinary;
pub use self::hex_string::HexString;
pub use self::mixed::{Mixed, MixedDeserializer, MixedSerializer};
pub use self::namespace_scope::NamespaceScope;
pub use self::namespaces::{
    Key as NamespaceKey, Namespaces, NamespacesShared, Value as NamespaceValue,
};
pub use self::nillable::{Nillable, NillableDeserializer, NillableSerializer};
pub use self::qname::QName;
pub use self::text::{Text, TextDeserializer, TextSerializer};
pub use self::value::Value;

#[cfg(feature = "quick-xml")]
pub use self::any_simple_type::{AnySimpleTypeDeserializer, AnySimpleTypeSerializer};
