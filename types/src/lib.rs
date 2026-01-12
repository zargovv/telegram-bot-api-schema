#![feature(bool_to_result)]

#[allow(clippy::doc_markdown)]
mod r#gen;

pub use r#gen::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Unexpected},
};

pub trait Method<'a> {
    const NAME: &'static str;
    type Response: Deserialize<'a>;
}

pub struct True;

impl From<True> for bool {
    fn from(_value: True) -> Self {
        true
    }
}

impl<'de> Deserialize<'de> for True {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        bool::deserialize(de)?
            .ok_or(de::Error::invalid_value(Unexpected::Bool(false), &"True"))
            .map(|()| Self)
    }
}

impl Serialize for True {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_bool(true)
    }
}
