use strum_macros::EnumString;

#[derive(Debug)]
pub enum Expr {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Debug)]
pub struct Struct {
    pub struct_type: Type,
    pub fields: Vec<Field>,
    pub is_component: bool,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub field_kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    Type(Type),
    Primitive(Primitive),
}

#[derive(Debug)]
pub struct Type {
    pub type_name: String,
    pub type_parameters: Option<Vec<Kind>>
}

#[derive(Debug)]
#[derive(EnumString)]
pub enum Primitive {
    #[strum(serialize = "bool")]
    Bool,
    #[strum(serialize = "u8")]
    U8,
    #[strum(serialize = "u16")]
    U16,
    #[strum(serialize = "u32")]
    U32,
    #[strum(serialize = "u64")]
    U64,
    #[strum(serialize = "u128")]
    U128,
    #[strum(serialize = "i8")]
    I8,
    #[strum(serialize = "i16")]
    I16,
    #[strum(serialize = "i32")]
    I32,
    #[strum(serialize = "i64")]
    I64,
    #[strum(serialize = "i128")]
    I128,
    #[strum(serialize = "f32")]
    F32,
    #[strum(serialize = "f64")]
    F64,
    #[strum(serialize = "char")]
    Char,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>
}
