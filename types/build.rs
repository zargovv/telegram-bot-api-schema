#![feature(trim_prefix_suffix)]

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter, Write as _},
};

use schemagen::{EntryField, SchemaEntry, Type, serde_json};

fn format_description(s: &str) -> String {
    s.replace(
        "<a href=\"#",
        "<a href=\"https://core.telegram.org/bots/api#",
    )
    // TODO: `<img>` tag
}

fn type_name(ty: &Type) -> Cow<'_, str> {
    match ty {
        Type::True => Cow::Borrowed("True"),
        Type::Boolean => Cow::Borrowed("Boolean"),
        Type::Integer => Cow::Borrowed("Integer"),
        Type::Float => Cow::Borrowed("Float"),
        Type::String => Cow::Borrowed("String"),
        Type::Array(ty) => Cow::Owned(format!("ArrayOf{}", type_name(ty))),
        Type::Object(name) => Cow::Borrowed(name),
        Type::Union(_items) => unreachable!(),
        Type::Optional(ty) => Cow::Owned(format!("Optional{}", type_name(ty))),
    }
}

fn convert_type<'a>(
    ty: &'a Type,
    backref: Option<(&HashMap<String, HashSet<String>>, &str)>,
) -> Cow<'a, str> {
    match ty {
        Type::True => Cow::Borrowed("crate::True"),
        Type::Boolean => Cow::Borrowed("bool"),
        Type::Integer => Cow::Borrowed("i64"),
        Type::Float => Cow::Borrowed("f64"),
        Type::String => Cow::Borrowed("String"),
        Type::Array(ty) => Cow::Owned(format!("Vec<{}>", convert_type(ty, None))),
        Type::Object(name) => {
            if backref.is_some_and(|(refs, super_name)| {
                refs.get(name).is_some_and(|set| set.contains(super_name))
            }) {
                Cow::Owned(format!("Box<{name}>"))
            } else {
                Cow::Borrowed(name)
            }
        }
        Type::Union(items) => {
            match items
                .iter()
                .map(type_name)
                .fold(String::new(), |mut acc, name| {
                    if !acc.is_empty() {
                        acc.push_str("Or");
                    }
                    acc.push_str(&name);
                    acc
                }) {
                s if s == "IntegerOrString" => Cow::Borrowed("ChatId"),
                s if s == "InputFileOrString" => Cow::Borrowed("Attachment"),
                s if s
                    == "InlineKeyboardMarkupOrReplyKeyboardMarkupOrReplyKeyboardRemoveOrForceReply" =>
                {
                    Cow::Borrowed("ReplyMarkup")
                }
                s => Cow::Owned(s),
            }
        }
        Type::Optional(ty) => Cow::Owned(format!("Option<{}>", convert_type(ty, backref))),
    }
}

struct UnionVariant {
    name: String,
    ty: String,
}

struct Ctx {
    schema: Vec<SchemaEntry>,
    unions: HashMap<String, Vec<UnionVariant>>,
    refs: HashMap<String, HashSet<String>>,
}

impl Ctx {
    pub fn propagate_object_refs(&mut self) {
        fn propagate(refs: &mut HashSet<String>, ty: &Type) {
            match ty {
                Type::True
                | Type::Boolean
                | Type::Integer
                | Type::Float
                | Type::String
                | Type::Array(_) => {}
                Type::Union(items) => {
                    for ty in items {
                        propagate(refs, ty);
                    }
                }
                Type::Object(name) => {
                    refs.insert(name.clone());
                }
                Type::Optional(ty) => propagate(refs, ty),
            }
        }

        for entry in &self.schema {
            let mut set = HashSet::<String>::new();
            entry.iter_types().for_each(|ty| propagate(&mut set, ty));
            self.refs.insert(entry.name().to_owned(), set);
        }
    }

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
                    unions
                        .entry(convert_type(ty, None).to_string())
                        .or_insert_with(|| {
                            items
                                .iter()
                                .map(|ty| UnionVariant {
                                    name: type_name(ty).to_string(),
                                    ty: convert_type(ty, None).to_string(),
                                })
                                .collect()
                        });
                }
            }
        }

        for entry in &self.schema {
            entry
                .iter_types()
                .for_each(|ty| process_ty(&mut self.unions, ty));
        }
    }
}

fn escape_name(s: &str) -> &str {
    match s {
        "type" => "r#type",
        s => s,
    }
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    println!("cargo::rerun-if-changed=../schema.json");

    let schema = serde_json::from_reader::<_, Vec<SchemaEntry>>(BufReader::new(File::open(
        "../schema.json",
    )?))?;
    let mut output = BufWriter::new(File::create("src/gen.rs")?);
    output.write_all(b"// this file is auto-generated\n\n")?;

    let mut ctx = Ctx {
        schema,
        unions: HashMap::new(),
        refs: HashMap::new(),
    };
    ctx.collect_unions();
    ctx.propagate_object_refs();

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
        fn write_fields(
            output: &mut BufWriter<File>,
            fields: &[EntryField],
            backref: (&HashMap<String, HashSet<String>>, &str),
        ) -> std::io::Result<()> {
            if fields.is_empty() {
                output.write_all(b";\n")?;
            } else {
                output.write_all(b" {\n")?;
                for f in fields {
                    output.write_fmt(format_args!(
                        "    pub {}: {},\n",
                        escape_name(&f.name),
                        convert_type(&f.ty, Some(backref)),
                    ))?;
                }
                output.write_all(b"}\n")?;
            }
            Ok(())
        }

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
                        "pub struct {name}",
                    ],
                    name = name,
                    description = format_description(description),
                ))?;
                write_fields(&mut output, fields, (&ctx.refs, name))?;
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
                        "#[derive(macros::Method)]\n",
                        "#[method(name = {route:?}, response({return_type}))]\n",
                        "pub struct {name}Request"
                    ],
                    name = name,
                    description = format_description(description),
                    route = route,
                    return_type = convert_type(returns, None),
                ))?;
                write_fields(&mut output, fields, (&ctx.refs, &name))?;
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
                        convert_type(ty, None),
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
