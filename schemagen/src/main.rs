#![feature(c_size_t)]

pub mod html_encoding;
pub mod libc;

use schemagen::{AccentColor, EntryField, SchemaEntry, Type};

use crate::libc::{MemchrExt, memrchr, memrmem};

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

fn resolve_return_type(description: &str) -> Type {
    let rest = if let Some(rest) = description.as_bytes().find_after(b"On success") {
        rest
    } else {
        let Some(pos) = memrmem(description.as_bytes(), b"Returns ") else {
            unreachable!("failed to resolve return definition");
        };
        &description.as_bytes()[pos..]
    };

    let ty = rest
        .find_needle(b"Array")
        .or_else(|| rest.find_needle(b"<a"))
        .map(|pos| &rest[pos..])
        .or_else(|| rest.find_between(b"<em>", b"</em>").map(|v| v.0))
        .expect("failed to resolve return type");
    // println!("ty = {:?}", unsafe { str::from_utf8_unchecked(ty) });
    Type::try_from(ty).expect("unreachable")
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
        heading = memrchr(heading, b'>').map_or(heading, |pos| &heading[pos + 1..]);
        let (description, rest) = rest
            .find_between(b"<p>", b"</p>")
            .expect("description expected");
        body = rest;

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
        let mut description =
            String::from_utf8(html_encoding::decode(description)).expect("invalid description");
        entries.push(match entry_kind {
            kind @ EntryKind::Object => SchemaEntry::Object {
                name,
                description,
                fields: collect_fields(kind, table),
            },
            kind @ EntryKind::Method => {
                let returns = resolve_return_type(&description);
                SchemaEntry::Method {
                    name,
                    description,
                    fields: collect_fields(kind, table),
                    returns,
                }
            }
            EntryKind::Union => {
                let pos = description.rfind('.').expect("invalid union description");
                description.truncate(pos + 1);
                SchemaEntry::Union {
                    name,
                    description,
                    variants: collect_union_variants(table),
                }
            }
            EntryKind::Enum => SchemaEntry::Enum {
                name,
                description,
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
