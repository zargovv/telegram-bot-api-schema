type Schema = Entry[];

type Entry = ObjectEntry | MethodEntry | UnionEntry | EnumEntry;

interface ObjectEntry {
    kind: "object";
    name: string;
    description: string;
    fields: EntryField[];
}

interface MethodEntry {
    kind: "method";
    name: string;
    description: string;
    fields: EntryField[];
    returns: Type;
}

interface UnionEntry {
    kind: "union";
    name: string;
    description: string;
    variants: Type[];
}

interface EnumEntry {
    kind: "enum";
    name: string;
    description: string;
    variants: AccentColor[];
}

interface AccentColor {
    "Color identifier": string;
    "Light colors": string;
    "Dark colors": string;
}

interface EntryField {
    name: string;
    type: Type;
    description: string;
}

type Type = PrimitiveType | ArrayType | ObjectType | UnionType | OptionalType;

type PrimitiveType = "True" | "Boolean" | "Integer" | "Float" | "String";

type ArrayType = { Array: Type };
type ObjectType = { Object: string };
type UnionType = { Union: Type[] };
type OptionalType = { Optional: Type };

// deno-lint-ignore no-unused-vars
function todo(message?: string): never {
    console.error("TODO" + (message ? `: ${message}` : ""));
    Deno.exit(1);
}

function unreachable(message?: string): never {
    console.error("unreachable" + (message ? `: ${message}` : ""));
    Deno.exit(1);
}

const DOCS_BASE_URI = "https://core.telegram.org/bots/api";

const te = new TextEncoder();

const schema: Schema = JSON.parse(await Deno.readTextFile("schema.json"));

const outputFile = await Deno.create("examples/typing-generation/typing.d.ts");
outputFile.write(te.encode("// this file is auto-generated\n\n"));

function convertType(ty: Type): string;
function convertType(ty: Type[]): string[];
function convertType(ty: Type | Type[]): string | string[] {
    if (Array.isArray(ty)) return ty.map(t => convertType(t));

    if (typeof ty === "string") {
        // PrimitiveType
        switch (ty) {
            case "True":
                return "true";
            case "Boolean":
                return "boolean";
            case "Integer":
                return "number";
            case "Float":
                return "number";
            case "String":
                return "string";
            default:
                unreachable(`unknown type ${ty}`);
        }
    }

    if ("Array" in ty) {
        return `${convertType(ty.Array)}[]`;
    } else if ("Object" in ty) {
        return ty.Object;
    } else if ("Union" in ty) {
        return convertType(ty.Union).join(" | ");
    } else if ("Optional" in ty) {
        return `${convertType(ty.Optional)} | null`;
    } else {
        const kind = Object.keys(ty)[0];
        unreachable(`unknown type kind: ${kind}`);
    }
}

function formatDescription(s: string): string {
    return s
        .replace(/<br>/g, "\n")
        .replace(/<\/?code>/g, "`")
        .replace(/<\/?em>/g, "*")
        .replace(/<\/?strong>/g, "**")
        .replace(/<img [^>]*alt="([^"]*)"[^>]*\/>/g, "$1")
        .replace(/<a href="([^"]*)">([^<]*)<\/a>/g, (_, href, label) => {
            const url =
                href.startsWith("#") || href.startsWith("/") ? `${DOCS_BASE_URI}${href}` : href;
            return `[${label}](${url})`;
        });
}

function toCamelcase(s: string): string {
    let r = "";

    let uppercase = false;
    for (let i = 0; i < s.length; ++i) {
        const c = s[i];
        if (c === " ") {
            uppercase = true;
            continue;
        }

        r += uppercase ? c.toUpperCase() : c;
        uppercase = false;
    }

    return r;
}

function titlecase(s: string): string {
    return `${s[0].toUpperCase()}${s.substring(1)}`;
}

for (const entry of schema) {
    const kind = entry.kind;
    const desc = `/**
${formatDescription(entry.description)
    .split("\n")
    .map(s => ` * ${s}`.trimEnd())
    .join("\n")}
 */\n`;
    switch (kind) {
        case "method":
        case "object": {
            const fields = entry.fields.map(
                f => `    /**
${formatDescription(f.description)
    .split("\n")
    .map(s => `     * ${s}`.trimEnd())
    .join("\n")}
     */
    ${f.name}: ${convertType(f.type)};`,
            );

            let lintIgnore = "";
            if (fields.length < 1) lintIgnore += "// deno-lint-ignore no-empty-interface\n";

            const name = kind === "method" ? titlecase(entry.name) + "Request" : entry.name;
            const def = `${desc}${lintIgnore}export interface ${name}`;
            const sep = fields.length < 1 ? "" : "\n";
            await outputFile.write(te.encode(`${def} {${sep}${fields.join("\n")}${sep}}\n`));

            if (kind === "method") {
                await outputFile.write(
                    te.encode(
                        `export type ${titlecase(entry.name)}Response = ${convertType(
                            entry.returns,
                        )};\n`,
                    ),
                );
            }

            break;
        }
        case "union":
            await outputFile.write(
                te.encode(
                    `${desc}export type ${entry.name} = ${convertType(entry.variants).join(
                        " | ",
                    )};\n`,
                ),
            );
            break;
        case "enum": {
            const variants = entry.variants.map(
                v => `    /**
     * - Light colors: ${v["Light colors"]}
     * - Dark colors: ${v["Dark colors"]}
     */
    _${v["Color identifier"]} = ${v["Color identifier"]},`,
            );
            await outputFile.write(
                te.encode(`${desc}export enum ${toCamelcase(entry.name)} {
${variants.join("\n")}
}\n`),
            );
            break;
        }
        default:
            unreachable(`unknown entry kind ${kind}`);
    }
}

outputFile.close();
