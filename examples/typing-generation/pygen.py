import json
import re
import sys
from pathlib import Path
from typing import Literal, TypedDict

DOCS_BASE_URI = "https://core.telegram.org/bots/api"

type Type = PrimitiveType | ArrayType | ObjectType | UnionType | OptionalType

type PrimitiveType = Literal["True", "Boolean", "Integer", "Float", "String"]

type ArrayType = dict[Literal["Array"], Type]
type ObjectType = dict[Literal["Object"], str]
type UnionType = dict[Literal["Union"], list[Type]]
type OptionalType = dict[Literal["Optional"], Type]


class EntryField(TypedDict):
    name: str
    type: Type
    description: str


class MethodEntry(TypedDict):
    kind: Literal["method"]
    name: str
    description: str
    fields: list[EntryField]


class UnionEntry(TypedDict):
    kind: Literal["union"]
    name: str
    description: str
    variants: list[Type]


class EnumEntry(TypedDict):
    kind: Literal["enum"]
    name: str
    description: str
    variants: list[dict[str, str]]


type AccentColor = dict[
    Literal["Color identifier", "Light colors", "Dark colors"],
    str,
]


type SchemaEntry = ObjectEntry | MethodEntry | UnionEntry | EnumEntry


class ObjectEntry(TypedDict):
    kind: Literal["object"]
    name: str
    description: str
    fields: list[EntryField]


schema: list[SchemaEntry] = json.loads(Path("schema.json").read_bytes())


def convert_type(ty: Type) -> str:
    match ty:
        case "True":
            return "Literal[True]"
        case "Boolean":
            return "bool"
        case "Integer":
            return "int"
        case "Float":
            return "float"
        case "String":
            return "str"
        case _ if "Array" in ty:
            return f"list[{convert_type(ty['Array'])}]"
        case _ if "Object" in ty:
            return ty["Object"]
        case _ if "Union" in ty:
            return " | ".join([convert_type(t) for t in ty["Union"]])
        case _ if "Optional" in ty:
            return f"{convert_type(ty['Optional'])} | None"
        case ty:
            print(f"unknown type {ty}", file=sys.stderr)
            sys.exit(1)


def format_description(s: str) -> str:
    def replace_anchor(m: re.Match) -> str:
        href: str = m.group(1)
        label: str = m.group(2)
        url = f"{DOCS_BASE_URI}{href}" if href.startswith(("#", "/")) else href
        return f"[{label}]({url})"

    s = s.replace(r"<br>", "\n")
    s = re.sub(r"</?code>", "`", s)
    s = re.sub(r"</?em>", "*", s)
    s = re.sub(r"</?strong>", "**", s)
    s = re.sub(r"<img [^>]*alt=\"([^\"]*)\"[^>]*/>", "$1", s)
    return re.sub(r"<a href=\"([^\"]*)\">([^<]*)</a>", replace_anchor, s)


def to_camelcase(s: str) -> str:
    r = ""

    uppercase = False
    for i in range(len(s)):
        c = s[i]
        if c == " ":
            uppercase = True
            continue

        r += c.upper() if uppercase else c
        uppercase = False

    return r


def titlecase(s: str) -> str:
    return f"{s[0].upper()}{s[1:]}"


def safename(s: str) -> str:
    match s:
        case "from":
            return f"{s}_"
        case _:
            return s


with Path("examples/typing-generation/typing.pyi").open("w") as file:
    file.write(
        "# this file is auto-generated\n\n"
        "from enum import Enum\nfrom typing import Literal, TypedDict\n\n",
    )

    for entry in schema:
        match entry["kind"]:
            case "object" | "method":
                fields = [
                    f"    {safename(f['name'])}: {convert_type(f['type'])}" for f in entry["fields"]
                ]
                if len(fields) < 1:
                    fields.append("    ...")
                name = (
                    f"{titlecase(entry['name'])}Request"
                    if entry["kind"] == "method"
                    else entry["name"]
                )
                file.write(f"class {name}(TypedDict):\n{'\n'.join(fields)}\n")
            case "union":
                file.write(
                    f"type {entry['name']} = "
                    f"{' | '.join([convert_type(t) for t in entry['variants']])}\n",
                )
            case "enum":
                file.write(
                    f"class {to_camelcase(entry['name'])}(Enum):\n"
                    f"{
                        '\n'.join(
                            [
                                f'    _{v['Color identifier']} = {v['Color identifier']}'
                                for v in entry['variants']
                            ]
                        )
                    }\n",
                )
            case kind:
                print(f"unknown entry kind {kind}", file=sys.stderr)
                sys.exit(1)
