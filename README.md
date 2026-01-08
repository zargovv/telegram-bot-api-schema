# Telegram Bot API schema

All definitions are located in [`schema.json`](schema.json)

> Some section are skipped: `Determining list of commands`, `Sending files`, `Inline mode objects`,
> `Formatting options`, `Paid Broadcasts`, `Inline mode methods`

```ts
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
```
