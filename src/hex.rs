use std::fmt;

use serde::{
    de::{Unexpected, Visitor},
    Deserializer, Serializer,
};

const EXPECTING: &str = "an RGBA hex code string";

struct HexVisitor;

impl<'de> Visitor<'de> for HexVisitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(EXPECTING)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let without_hash = v.trim_start_matches('#');
        let parsed = u32::from_str_radix(without_hash, 16)
            .map_err(|_| E::invalid_value(Unexpected::Str(&v), &EXPECTING))?;

        Ok(parsed)
    }
}

pub(super) fn serialize<S: Serializer>(v: &u32, serializer: S) -> Result<S::Ok, S::Error> {
    let with_0x = format!("{v:#010X}");
    let with_hash = format!("#{}", with_0x.trim_start_matches("0x"));

    serializer.serialize_str(&with_hash)
}

pub(super) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    deserializer.deserialize_str(HexVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_correctly() {
        let mut writer = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut writer);

        serialize(&0x001414FF, &mut serializer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), "\"#001414FF\"");
    }

    #[test]
    fn deserializes_correctly() {
        let mut deserializer = serde_json::Deserializer::from_str("\"#288ED2FF\"");

        assert_eq!(deserialize(&mut deserializer).unwrap(), 0x288ED2FF);
    }
}
