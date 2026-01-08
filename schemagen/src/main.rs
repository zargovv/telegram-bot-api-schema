#![feature(c_size_t)]

pub mod html_encoding;
pub mod libc;

use serde::Serialize;

use crate::libc::{MemchrExt, memrchr};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EntryKind {
    Object,
    Method,
    Union,
    Enum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Required {
    Required,
    Optional,
}

impl TryFrom<&[u8]> for Required {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(match value {
            b"Yes" => Self::Required,
            b"Optional" => Self::Optional,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
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
        } else if let Some(rest) = value.strip_prefix(b"Integer") {
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

        if let Some(extra) = rest.strip_prefix(b" or ") {
            ty = Type::Union(match Type::try_from(extra)? {
                Type::Union(mut items) => {
                    items.push(ty);
                    items
                }
                extra => vec![extra, ty],
            });
        }
        Ok(ty)
    }
}

#[derive(Debug, Serialize)]
pub struct EntryField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: Type,
    pub description: String,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SchemaEntry {
    Object {
        name: String,
        fields: Vec<EntryField>,
    },
    Method {
        name: String,
        fields: Vec<EntryField>,
    },
    Union {
        name: String,
        variants: Vec<Type>,
    },
    Enum {
        name: String,
        // currently enum type is hard-coded because `Accent colors` and `Profile accent colors` are
        // the only enumerations present in the documentation
        variants: Vec<AccentColor>,
    },
}

fn collect_fields(entry_kind: EntryKind, mut table: &[u8]) -> Vec<EntryField> {
    let mut fields = Vec::<EntryField>::new();

    while let Some((field, rest)) = table.find_between(b"<td>", b"</td>") {
        let (ty, mut rest) = rest
            .find_between(b"<td>", b"</td>")
            .expect("column `type` expected");
        let mut ty = Type::try_from(ty).expect("failed to identify type");
        let required = if entry_kind == EntryKind::Method {
            let (required, rest_) = rest
                .find_between(b"<td>", b"</td>")
                .expect("column `required` expected");
            rest = rest_;
            Some(Required::try_from(required).expect("invalid `required` column value"))
        } else {
            None
        };
        let (mut description, rest) = rest
            .find_between(b"<td>", b"</td>")
            .expect("column `description` expected");
        table = rest;

        let optional = if let Some(desc) = description.strip_prefix(b"<em>Optional</em>. ") {
            description = desc;
            true
        } else {
            required == Some(Required::Optional)
        };
        if optional {
            ty = Type::Optional(Box::new(ty));
        }

        // println!("field = {} {ty:?}", unsafe {
        //     str::from_utf8_unchecked(field)
        // });

        fields.push(EntryField {
            name: str::from_utf8(field)
                .expect("invalid field name")
                .to_owned(),
            ty,
            description: String::from_utf8(html_encoding::decode(description))
                .expect("invalid description"),
        });
    }

    fields
}

fn collect_union_variants(mut table: &[u8]) -> Vec<Type> {
    let mut variants = Vec::<Type>::new();
    while let Some((variant, rest)) = table.find_between(b"<li>", b"</li>") {
        table = rest;
        variants.push(Type::try_from(variant).expect("failed to identify union type"));
    }
    variants
}

fn collect_enum_variants(mut table: &[u8]) -> Vec<AccentColor> {
    let mut variants = Vec::<AccentColor>::new();
    while let Some((id, rest)) = table.find_between(b"<td>", b"</td>") {
        let (light_colors, rest) = rest
            .find_between(b"<td>", b"</td>")
            .expect("column `Light colors` is expected");
        let (dark_colors, rest) = rest
            .find_between(b"<td>", b"</td>")
            .expect("column `Light colors` is expected");
        table = rest;

        variants.push(AccentColor {
            id: unsafe { str::from_utf8_unchecked(id) }.to_owned(),
            light_colors: unsafe { str::from_utf8_unchecked(light_colors) }.to_owned(),
            dark_colors: unsafe { str::from_utf8_unchecked(dark_colors) }.to_owned(),
        });
    }
    variants
}

fn main() {
    let client = reqwest::blocking::Client::builder()
        .build()
        .expect("failed to build http client");
    let body = client
        .get("https://core.telegram.org/bots/api")
        .send()
        .expect("failed to get api docs")
        .bytes()
        .expect("failed to read api docs body");

    // std::fs::write("body.html", body).expect("failed to write body to file");

    // let body = std::fs::read("body.html").expect("failed to read api docs");

    let mut body = body
        .find_after(br#"name="getting-updates""#)
        .expect("failed to find 'getting-updates'");

    let mut entries = Vec::<SchemaEntry>::new();
    while let Some((mut heading, rest)) = body.find_between(b"<h4>", b"</h4>") {
        body = rest;

        heading = memrchr(heading, b'>').map_or(heading, |pos| &heading[pos + 1..]);

        if matches!(
            heading,
            b"Determining list of commands"
                | b"Sending files"
                | b"Inline mode objects"
                | b"Formatting options"
                | b"Paid Broadcasts"
                | b"Inline mode methods"
        ) {
            eprintln!("skip: {}", unsafe { str::from_utf8_unchecked(heading) });
            continue;
        }

        let (mut table, rest) = body.find_until(b"<h4>").unwrap_or((body, &[]));
        body = rest;
        if let Some((t, _)) = table.find_until(b"</table>") {
            table = t;
        }

        let entry_kind = if table.find_needle(b"<li>").is_some() {
            EntryKind::Union
        } else if table.find_needle(b"table-bordered").is_some() {
            EntryKind::Enum
        } else if heading.first().expect("empty heading").is_ascii_uppercase() {
            EntryKind::Object
        } else {
            EntryKind::Method
        };

        let name = str::from_utf8(heading)
            .expect("invalid entry name")
            .to_owned();
        entries.push(match entry_kind {
            kind @ EntryKind::Object => SchemaEntry::Object {
                name,
                fields: collect_fields(kind, table),
            },
            kind @ EntryKind::Method => SchemaEntry::Method {
                name,
                fields: collect_fields(kind, table),
            },
            EntryKind::Union => SchemaEntry::Union {
                name,
                variants: collect_union_variants(table),
            },
            EntryKind::Enum => SchemaEntry::Enum {
                name,
                variants: collect_enum_variants(table),
            },
        });
    }

    std::fs::write(
        "schema.json",
        serde_json::to_string(&entries).expect("failed to serialize entries"),
    )
    .expect("failed to write schema");
}
