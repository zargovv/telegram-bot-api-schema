# Telegram Bot API schema

All definitions are located in [`schema.json`](schema.json).

```ts
type Schema = Entry[];

type Entry = ObjectEntry | MethodEntry | UnionEntry;

interface ObjectEntry {
    kind: "object";
    name: string;
    fields: EntryField[];
}

interface MethodEntry {
    kind: "method";
    name: string;
    fields: EntryField[];
}

interface UnionEntry {
    kind: "union";
    variants: Type[];
}

interface EntryField {
    name: string;
    type: Type;
    description: string;
}

type Type = PrimitiveType | ArrayType | ObjectType | UnionType | OptionalType;

type PrimitiveType = "True" | "Boolean" | "Integer" | "Float" | "String";

type ArrayType = { Array: Type };
type ObjectType = { Object: Type };
type UnionType = { Union: Type[] };
type OptionalType = { Optional: Type };
```
