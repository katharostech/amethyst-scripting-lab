use std::str::FromStr;

use peg::parser;

mod ast;
use ast::*;

parser! {
/// Parser grammar for the component schema definition
grammar schema_parser() for str {
    /// Parse the schema into a set of `Expr`'s.
    pub rule schema() -> Vec<Expr> =
        wn() exprs:(
            component_type() /
            struct() /
            enum()
        ) ** wn() wn() ![_] { exprs }

    // Whitespace character
    rule whitespace_char() = ['\t' | ' ']

    // Line comment
    rule line_comment() = "//" (!"\n" [_])* ("\n" / ![_])

    // Inline comment
    rule inline_comment() = "/*" (!"*/" [_])* "*/"

    // Whitespace including comments
    rule w() = (whitespace_char() / inline_comment())*

    // Whitespace including newlines and line comments
    rule wn() = (whitespace_char() / "\n" / inline_comment() / line_comment())*

    // The name of a type
    rule type_name() = ['A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9']*

    // A valid identifier
    rule identifier() = ['a'..='z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*

    // A primitive type
    rule primitive() -> Primitive =
        type_name:$(
            "bool" /
            "u8" /
            "u16" /
            "u32" /
            "u64" /
            "u128" /
            "i8" /
            "i16" /
            "i32" /
            "i64" /
            "i128" /
            "f32" /
            "f64" /
            "char"
        ) {
            Primitive::from_str(type_name).unwrap()
        }

    // The component type definition
    rule component_type() -> Expr =
        w() "type" w() "Component" w() "=" w() name:$type_name() ";" {
            Expr::ComponentName(name.into())
        }

    // A struct definition
    rule struct() -> Expr = 
        w() def:$("struct" / "component") w() struct_type:struct_type() w() "{" wn()
            fields:struct_field() ** ("," wn()) ","? wn()
        "}" {
            Expr::Struct(Struct {
                struct_type,
                fields,
                is_component: if def == "component" { true } else { false }
            })
        }

    // A non-primitive struct or field type
    rule struct_type() -> Type =
        type_name:$type_name() params:type_parameters() {
            Type {
                type_name: type_name.into(),
                type_parameters: params
            }
        }

    // A struct type parameter
    rule type_parameters() -> Option<Vec<Kind>> =
        ("<" params:field_kind() ** ("," w()) ">" { params })?

    // A struct field
    rule struct_field() -> Field =
        name:$identifier() w() ":" w() field_kind:field_kind() { 
            Field {
                name: name.into(),
                field_kind,
            }
        }

    // A field kind
    rule field_kind() -> Kind =
        field_type:struct_type() { Kind::Type(field_type) } /
        primitive_type:primitive() { Kind::Primitive(primitive_type) }

    // An Enum
    rule enum() -> Expr =
        w() "enum" w() name:$type_name() w() "{" wn()
            variants:$type_name() ** ("," wn()) ","? wn()
        "}" {
            Expr::Enum(Enum {
                name: name.into(),
                variants: variants.iter().map(|&s| s.into()).collect(),
            })
        }
}}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_parser::schema(include_str!("../example-schema.rs"))?;

    println!("{:#?}", schema);

    Ok(())
}
