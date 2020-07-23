//! De/Serialization of hexadecimal encoded bytes
//!
//! This modules is only available when using the `hex` feature of the crate.

use crate::{
    de::DeserializeAs,
    formats::{Format, Lowercase, Uppercase},
    ser::SerializeAs,
};
use serde::{de::Error, Deserialize, Deserializer, Serializer};
use std::{borrow::Cow, marker::PhantomData};

/// Serialize bytes as a hex string
///
/// The type serializes a sequence of bytes as a hexadecimal string.
/// It works on any type implementing `AsRef<[u8]>` for serialization and `From<Vec<u8>>` for deserialization.
///
/// The format type parameter specifies if the hex string should use lower- or uppercase characters.
/// Valid options are the types [`Lowercase`] and [`Uppercase`].
/// Deserialization always supports lower- and uppercase characters, even mixed in one string.
// TODO example
#[derive(Copy, Clone, Debug, Default)]
pub struct Hex<FORMAT: Format = Lowercase>(PhantomData<FORMAT>);

impl<T> SerializeAs<T> for Hex<Lowercase>
where
    T: AsRef<[u8]>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(source))
    }
}

impl<T> SerializeAs<T> for Hex<Uppercase>
where
    T: AsRef<[u8]>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode_upper(source))
    }
}

impl<'de, T, FORMAT> DeserializeAs<'de, T> for Hex<FORMAT>
where
    T: From<Vec<u8>>,
    FORMAT: Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Cow<'de, str> as Deserialize<'de>>::deserialize(deserializer)
            .and_then(|s| hex::decode(&*s).map_err(Error::custom))
            .map(Into::into)
    }
}
