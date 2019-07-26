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
            line_comment() /
            component_type() /
            struct()
        ) ** wn() wn() ![_] { exprs }

    //
    // Low level helper tokens
    //

    // Whitespace without including newlines
    rule w() = ['\t' | ' ']*

    // Whitespace including newlines
    rule wn() = ['\t' | ' ' | '\n']*

    // The name of a type
    rule type_name() = ['A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9']*

    // A valid identifier
    rule identifier() = ['a'..='z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*

    // A primitive type
    // TODO: More types
    rule primitive() -> Kind =
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
            Kind::Primitive(Primitive::from_str(type_name).unwrap())
        }

    //
    // Expression parsers
    //

    // Parse a line comment. Each line will be a separate comment `Expr`.
    rule line_comment() -> Expr =
        w() "//" " "? comment:$([x if x != '\n']*) ("\n" / ![_]) { Expr::Comment(comment.into()) }

    // The component type definition
    rule component_type() -> Expr =
        w() "type" w() "Component" w() "=" w() name:$(type_name()) ";" {
            Expr::ComponentName(name.into())
        }

    // Parse a struct definition
    rule struct() -> Expr = 
        w() "struct" w() name:$(type_name()) w() "{" wn()
            fields:(struct_field()) ** ("," wn()) ","? wn()
        "}" {
            Expr::Struct(Struct {
                name: name.into(),
                fields
            })
        }

    // Parse a struct field
    rule struct_field() -> Field =
        name:$(identifier()) w() ":" w() field_kind:(field_type() / primitive()) { 
            Field {
                name: name.into(),
                field_kind,
            }
        }

    // Parse a field "Type" kind, i.e. any field with a non-primitive type
    rule field_type() -> Kind =
        type_name:$(type_name()) {
            Kind::Type(Type {
                type_name: type_name.into()
            })
        }

}}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_parser::schema(include_str!("./example-schema.rs"))?;

    println!("{:#?}", schema);

    Ok(())
}