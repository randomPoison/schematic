use crate::TypeName;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// In-memory representation of a type tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Schema {
    Struct(Struct),
    UnitStruct(UnitStruct),
    NewtypeStruct(Box<NewtypeStruct>),
    TupleStruct(TupleStruct),
    Enum(Enum),
    Option(Box<Schema>),
    Seq(Box<Schema>),
    Tuple(Vec<Schema>),
    Map {
        key: Box<Schema>,
        value: Box<Schema>,
    },
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Bool,
    Char,
    String,
    Unit,
}

impl Schema {
    pub fn as_struct(&self) -> Option<&Struct> {
        match self {
            Schema::Struct(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_enum(&self) -> Option<&Enum> {
        match self {
            Schema::Enum(inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnitStruct {
    pub name: TypeName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NewtypeStruct {
    pub name: TypeName,
    pub inner: Schema,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(Cow<'static, str>, Schema)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Enum {
    pub name: TypeName,

    /// The explicit representation of the enum, as specified by the `#[repr(...)]`
    /// attribute.
    ///
    /// `None` if the
    pub repr: Option<Primitive>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Variant {
    Unit {
        name: Cow<'static, str>,
        discriminant: Option<PrimitiveValue>,
    },

    Struct {
        name: Cow<'static, str>,
        fields: Vec<(Cow<'static, str>, Schema)>,
    },

    Tuple {
        name: Cow<'static, str>,
        elements: Vec<Schema>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Primitive {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, Serialize, Deserialize)]
pub enum PrimitiveValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
}

impl fmt::Display for PrimitiveValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveValue::U8(value) => write!(f, "{}", value),
            PrimitiveValue::U16(value) => write!(f, "{}", value),
            PrimitiveValue::U32(value) => write!(f, "{}", value),
            PrimitiveValue::U64(value) => write!(f, "{}", value),
            PrimitiveValue::U128(value) => write!(f, "{}", value),
            PrimitiveValue::Usize(value) => write!(f, "{}", value),
            PrimitiveValue::I8(value) => write!(f, "{}", value),
            PrimitiveValue::I16(value) => write!(f, "{}", value),
            PrimitiveValue::I32(value) => write!(f, "{}", value),
            PrimitiveValue::I64(value) => write!(f, "{}", value),
            PrimitiveValue::I128(value) => write!(f, "{}", value),
            PrimitiveValue::Isize(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TupleStruct {
    pub name: TypeName,
}

/// Expands to a [`TypeName`] for the specified type.
///
/// When invoking this macro, a couple of things should be kept in mind in order to
/// get the correct result:
///
/// * This macro should be invoked in the same module that declares `$ty`, otherwise
///   the module path will not be correct.
/// * The given name should be unqualified, e.g. instead of `type_name!(foo::bar::Baz)`,
///   you should invoke it as `type_name!(Baz)`. This restriction may be lifted in
///   the future.
///
/// [`TypeName`]: struct.TypeName.html
#[macro_export]
macro_rules! type_name {
    ($ty:ty) => {
        $crate::TypeName {
            // TODO: Support stripping off
            name: std::borrow::Cow::Borrowed(stringify!($ty)),
            module: std::borrow::Cow::Borrowed(module_path!()),
        }
    };
}
