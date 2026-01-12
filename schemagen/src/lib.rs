#![feature(c_size_t)]

mod libc;

pub use serde;
use serde::{Deserialize, Serialize};
pub use serde_json;

use crate::libc::MemchrExt;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Type {
    True,
    Boolean,
    Integer,
    Float,
    String,
    Array(Box<Type>),
    Object(String),
    Union(Vec<Type>),
    Optional(Box<Type>),
}

impl TryFrom<&[u8]> for Type {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (mut ty, rest) = if let Some(rest) = value.strip_prefix(b"True") {
            (Self::True, rest)
        } else if let Some(rest) = value.strip_prefix(b"Boolean") {
            (Self::Boolean, rest)
        } else if let Some(rest) = value
            .strip_prefix(b"Integer")
            .or_else(|| value.strip_prefix(b"Int"))
        {
            (Self::Integer, rest)
        } else if let Some(rest) = value.strip_prefix(b"Float") {
            (Self::Float, rest)
        } else if let Some(rest) = value.strip_prefix(b"String") {
            (Self::String, rest)
        } else if let Some(ty) = value.strip_prefix(b"Array of ") {
            return Ok(Self::Array(Box::new(Type::try_from(ty)?)));
        } else {
            let (object, rest) = value.find_between(b'>', b"</a>").ok_or(())?;
            (
                Self::Object(unsafe { String::from_utf8_unchecked(object.to_owned()) }),
                rest,
            )
        };

        if let Some(extra) = rest
            .strip_prefix(b" or ")
            .or_else(|| rest.find_after(b"otherwise <em>"))
        {
            ty = Type::Union(match Type::try_from(extra)? {
                Type::Union(mut items) => {
                    items.insert(0, ty);
                    items
                }
                extra => vec![ty, extra],
            });
        }

        Ok(ty)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccentColor {
    // field names are preserved to keep the API backward-compatible once the enum type definition
    // becomes more generic
    #[serde(rename = "Color identifier")]
    pub id: String,
    #[serde(rename = "Light colors")]
    pub light_colors: String,
    #[serde(rename = "Dark colors")]
    pub dark_colors: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SchemaEntry {
    Object {
        name: String,
        description: String,
        fields: Vec<EntryField>,
    },
    Method {
        name: String,
        description: String,
        fields: Vec<EntryField>,
        returns: Type,
    },
    Union {
        name: String,
        description: String,
        variants: Vec<Type>,
    },
    Enum {
        name: String,
        description: String,
        // currently enum type is hard-coded because `Accent colors` and `Profile accent colors` are
        // the only enumerations present in the documentation
        variants: Vec<AccentColor>,
    },
}
