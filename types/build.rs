#![feature(trim_prefix_suffix)]

use core::iter::once;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Write as _},
};

use schemagen::{SchemaEntry, Type, serde_json};

fn format_description(s: &str) -> String {
    s.replace(
        "<a href=\"#",
        "<a href=\"https://core.telegram.org/bots/api#",
    )
    // TODO: `<img>` tag
}

fn type_name(ty: &Type) -> String {
    match ty {
        Type::True => "True".to_owned(),
        Type::Boolean => "Boolean".to_owned(),
        Type::Integer => "Integer".to_owned(),
        Type::Float => "Float".to_owned(),
        Type::String => "String".to_owned(),
        Type::Array(ty) => format!("ArrayOf{}", type_name(ty)),
        Type::Object(name) => name.clone(),
        Type::Union(_items) => unreachable!(),
        Type::Optional(ty) => format!("Optional{}", type_name(ty)),
    }
}

fn convert_type(ty: &Type) -> String {
    match ty {
        Type::True => "crate::True".to_owned(),
        Type::Boolean => "bool".to_owned(),
        Type::Integer => "i64".to_owned(),
        Type::Float => "f64".to_owned(),
        Type::String => "String".to_owned(),
        Type::Array(ty) => format!("Vec<{}>", convert_type(ty)),
        Type::Object(name) => name.clone(),
        Type::Union(items) => items
            .iter()
            .map(type_name)
            .fold(String::new(), |mut acc, name| {
                if !acc.is_empty() {
                    acc.push_str("Or");
                }
                acc.push_str(&name);
                acc
            }),
        Type::Optional(ty) => format!("Option<{}>", convert_type(ty)),
    }
}

struct UnionVariant {
    name: String,
    ty: String,
}

struct Ctx {
    schema: Vec<SchemaEntry>,
    unions: HashMap<String, Vec<UnionVariant>>,
}

impl Ctx {
    pub fn collect_unions(&mut self) {
        fn process_ty(unions: &mut HashMap<String, Vec<UnionVariant>>, ty: &Type) {
            match ty {
                Type::True
                | Type::Boolean
                | Type::Integer
                | Type::Float
                | Type::String
                | Type::Object(_) => {}
                Type::Array(ty) | Type::Optional(ty) => process_ty(unions, ty),
                ty @ Type::Union(items) => {
                    unions.entry(convert_type(ty)).or_insert_with(|| {
                        items
                            .iter()
                            .map(|ty| UnionVariant {
                                name: type_name(ty),
                                ty: convert_type(ty),
                            })
                            .collect()
                    });
                }
            }
        }

        for entry in &self.schema {
            match entry {
                SchemaEntry::Object { fields, .. } => fields
                    .iter()
                    .for_each(|f| process_ty(&mut self.unions, &f.ty)),
                SchemaEntry::Method {
                    fields, returns, ..
                } => fields
                    .iter()
                    .map(|f| &f.ty)
                    .chain(once(returns))
                    .for_each(|ty| process_ty(&mut self.unions, ty)),
                SchemaEntry::Union { variants, .. } => variants
                    .iter()
                    .for_each(|ty| process_ty(&mut self.unions, ty)),
                SchemaEntry::Enum { .. } => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let schema = serde_json::from_reader::<_, Vec<SchemaEntry>>(BufReader::new(File::open(
        "../schema.json",
    )?))?;
    let mut output = BufWriter::new(File::create("src/gen.rs")?);
    output.write_all(b"// this file is auto-generated\n\n")?;

    let mut ctx = Ctx {
        schema,
        unions: HashMap::new(),
    };
    ctx.collect_unions();

    for (name, variants) in &ctx.unions {
        output.write_fmt(format_args!(
            concat![
                "#[derive(serde::Serialize, serde::Deserialize)]\n",
                "#[serde(untagged)]\n",
                "pub enum {name} {{\n",
            ],
            name = name,
        ))?;
        for v in variants {
            output.write_fmt(format_args!("    {}({}),\n", v.name, v.ty))?;
        }
        output.write_all(b"}\n")?;
    }

    for entry in &ctx.schema {
        match &entry {
            SchemaEntry::Object {
                name,
                description,
                fields,
            } => {
                output.write_fmt(format_args!(
                    concat![
                        "/// {description}\n",
                        "#[derive(serde::Serialize, serde::Deserialize)]\n",
                        "pub struct {name} {{",
                    ],
                    name = name,
                    description = format_description(description),
                ))?;
                output.write_all(b"}\n")?;
            }
            SchemaEntry::Method {
                name,
                description,
                fields,
                returns,
            } => {
                let route = name;
                let mut name = route.clone();
                name[..1].make_ascii_uppercase();

                output.write_fmt(format_args!(
                    concat![
                        "/// {description}\n",
                        "#[derive(serde::Serialize)]\n",
                        "pub struct {name}Request {{"
                    ],
                    name = name,
                    description = format_description(description),
                ))?;
                output.write_all(b"}\n")?;

                output.write_fmt(format_args!(
                    concat![
                        "impl crate::Method<'_> for {name}Request {{\n",
                        "    const NAME: &'static str = {route:?};\n",
                        "    type Response = {return_type};\n",
                        "}}\n",
                    ],
                    name = name,
                    route = route,
                    return_type = convert_type(returns),
                ))?;
            }
            SchemaEntry::Union {
                name,
                description,
                variants,
            } => {
                output.write_fmt(format_args!(
                    concat![
                        "/// {description}\n",
                        "#[derive(serde::Serialize, serde::Deserialize)]\n",
                        "#[serde(untagged)]\n",
                        "pub enum {name} {{",
                    ],
                    name = name,
                    description = format_description(description),
                ))?;
                for (i, ty) in variants.iter().enumerate() {
                    if i == 0 {
                        output.write_all(b"\n")?;
                    }
                    output.write_fmt(format_args!(
                        "    {}({}),\n",
                        type_name(ty).trim_prefix(name),
                        convert_type(ty),
                    ))?;
                }
                output.write_all(b"}\n")?;
            }
            SchemaEntry::Enum {
                name,
                description,
                variants,
            } => {}
        }
    }
    Ok(())
}
