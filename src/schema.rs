use crate::TypeName;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt, iter};

/// In-memory representation of a type tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Schema {
    Unit,
    Bool,
    Char,

    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,

    U8,
    U16,
    U32,
    U64,
    U128,
    USize,

    F32,
    F64,

    Str,
    String(TypeName),

    Option(Box<Schema>),

    Tuple(Vec<Schema>),

    Array(Box<Array>),
    Slice(Box<Schema>),
    Seq(Box<Sequence>),

    Map(Box<Map>),

    UnitStruct(UnitStruct),
    Struct(Struct),
    TupleStruct(TupleStruct),
    NewtypeStruct(Box<NewtypeStruct>),

    Enum(Enum),
}

impl Schema {
    /// Returns the [`TypeName`] for user-defined types.
    ///
    /// For user-defined types (i.e. structs and enums) this function returns the type
    /// name identifying the type. For all other types it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use schematic::{Schema, Struct, TypeName};
    ///
    /// let schema = Schema::Struct(Struct {
    ///     name: TypeName::new("MyStruct", "my_crate::my_module"),
    ///     fields: vec![],
    /// });
    ///
    /// let type_name = schema.type_name().unwrap();
    /// assert_eq!("MyStruct", type_name.name);
    /// assert_eq!("my_crate::my_module", type_name.module);
    /// ```
    pub fn type_name(&self) -> Option<&TypeName> {
        Some(match self {
            Schema::Struct(schema) => &schema.name,
            Schema::UnitStruct(schema) => &schema.name,
            Schema::NewtypeStruct(schema) => &schema.name,
            Schema::TupleStruct(schema) => &schema.name,
            Schema::Enum(schema) => &schema.name,

            _ => return None,
        })
    }

    pub fn as_struct(&self) -> Option<&Struct> {
        match self {
            Schema::Struct(schema) => Some(schema),
            _ => None,
        }
    }

    pub fn as_tuple_struct(&self) -> Option<&TupleStruct> {
        match self {
            Schema::TupleStruct(schema) => Some(schema),
            _ => None,
        }
    }

    pub fn as_newtype_struct(&self) -> Option<&NewtypeStruct> {
        match self {
            Schema::NewtypeStruct(schema) => Some(schema),
            _ => None,
        }
    }

    pub fn as_unit_struct(&self) -> Option<&UnitStruct> {
        match self {
            Schema::UnitStruct(schema) => Some(schema),
            _ => None,
        }
    }

    pub fn as_struct_like(&self) -> Option<StructLike<'_>> {
        match self {
            Schema::Struct(schema) => Some(schema.into()),
            Schema::TupleStruct(schema) => Some(schema.into()),
            Schema::NewtypeStruct(schema) => Some((&**schema).into()),
            Schema::UnitStruct(schema) => Some(schema.into()),

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
pub struct Array {
    pub element: Schema,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sequence {
    pub name: TypeName,
    pub element: Schema,
    pub len: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Map {
    pub name: TypeName,
    pub key: Schema,
    pub value: Schema,
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

impl NewtypeStruct {
    pub fn fields(&self) -> impl Iterator<Item = Field<'_>> {
        iter::once(Field::unnamed(&self.inner))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(Cow<'static, str>, Schema)>,
}

impl Struct {
    pub fn fields(&self) -> impl Iterator<Item = Field<'_>> {
        self.fields
            .iter()
            .map(|(name, schema)| Field::named(name, schema))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TupleStruct {
    pub name: TypeName,
    pub elements: Vec<Schema>,
}

impl TupleStruct {
    pub fn fields(&self) -> impl Iterator<Item = Field<'_>> {
        self.elements.iter().map(Field::unnamed)
    }
}

/// Generic representation for all struct-like schema types.
///
/// The data model has several different struct-like types:
///
/// * `struct`
/// * `tuple_struct`
/// * `newtype_struct`
/// * `unit_struct`
///
/// While it is often useful to handle these differently (and they're treated as
/// different in the [Serde data model][sdm]), there are also times where you want
/// to operate over all struct-like types in a uniform way. `StructLike` provides a
/// way to do this, providing a generic representation for all of the above types
/// in the data model.
///
/// [sdm]: https://serde.rs/data-model.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructLike<'a> {
    pub name: &'a TypeName,
    pub fields: Vec<Field<'a>>,
}

impl<'a> From<&'a Struct> for StructLike<'a> {
    fn from(from: &'a Struct) -> Self {
        Self {
            name: &from.name,
            fields: from.fields().collect(),
        }
    }
}

impl<'a> From<&'a TupleStruct> for StructLike<'a> {
    fn from(from: &'a TupleStruct) -> Self {
        Self {
            name: &from.name,
            fields: from.fields().collect(),
        }
    }
}

impl<'a> From<&'a NewtypeStruct> for StructLike<'a> {
    fn from(from: &'a NewtypeStruct) -> Self {
        Self {
            name: &from.name,
            fields: from.fields().collect(),
        }
    }
}

impl<'a> From<&'a UnitStruct> for StructLike<'a> {
    fn from(from: &'a UnitStruct) -> Self {
        Self {
            name: &from.name,
            fields: Vec::new(),
        }
    }
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

impl Enum {
    /// Returns `true` if any variant in the enum contains field data.
    ///
    /// When working with enums, it's often useful to handle C-like enums differently
    /// than enums that carry additional data. This method makes it easy to quickly
    /// determine which kind of enum you're dealing with.
    ///
    /// Note that this will return `true` for enums with struct-like or tuple-like
    /// variants if those variants don't contain any fields:
    ///
    /// ```ignore
    /// #[derive(Describe)]
    /// pub enum MyEnum {
    ///     Foo,
    ///     Bar {},
    ///     Baz (),
    /// }
    ///
    /// let schema = schematic::describe::<MyEnum>()
    ///     .and_then(Schema::as_enum)
    ///     .unwrap();
    /// assert!(!schema.has_data());
    /// ```
    pub fn has_data(&self) -> bool {
        !self.variants.iter().all(Variant::is_empty)
    }
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

impl Variant {
    /// Returns the name of the variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use schematic::Variant;
    ///
    /// let variant = Variant::Unit {
    ///     name: "Foo".into(),
    ///     discriminant: None,
    /// };
    ///
    /// assert_eq!("Foo", variant.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Variant::Unit { name, .. } => name,
            Variant::Struct { name, .. } => name,
            Variant::Tuple { name, .. } => name,
        }
    }

    /// Returns `true` for unit-like variants and struct/tuple-like variants with no fields.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #[derive(Schematic)]
    /// pub enum MyEnum {
    ///     Foo,
    ///     Bar {},
    ///     Baz (),
    /// }
    ///
    /// let schema = schematic::describe::<MyEnum>()
    ///     .and_then(Schema::as_enum)
    ///     .unwrap();
    ///
    /// for variant in &schema.variants {
    ///     assert!(variant.is_empty());
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        match self {
            Variant::Unit { .. } => true,
            Variant::Struct { fields, .. } => fields.is_empty(),
            Variant::Tuple { elements, .. } => elements.is_empty(),
        }
    }

    /// Returns an iterator over the fields in the variant.
    ///
    /// Useful if you're iterating over the variants of an enum and want to work with
    /// the fields of each variant without needing to deal with the specific type of
    /// each variant. For each of the variant types, the returned iterator does the
    /// following:
    ///
    /// * For a unit variant, an empty iterator is returned.
    /// * For a struct-like variant, the yielded elements will have names.
    /// * For a tuple-like variant, the yielded elements will not have names.
    pub fn fields(&self) -> Box<dyn Iterator<Item = Field<'_>> + '_> {
        match self {
            Variant::Unit { .. } => Box::new(iter::empty()),

            Variant::Struct { fields, .. } => Box::new(fields.iter().map(|(name, schema)| Field {
                name: Some(name),
                schema,
            })),

            Variant::Tuple { elements, .. } => {
                Box::new(elements.iter().map(|schema| Field { name: None, schema }))
            }
        }
    }
}

/// Generic representation of a field in a struct, enum variant, or tuple.
///
/// It's often desirable to be able to operate over the fields of different kinds of
/// types in a uniform manner. Any schema value that contains fields (or can
/// possibly contain fields) provides a way to get an iterator of `Field` objects in
/// to simplify these cases.
///
/// Note that the [`name`] field is an [`Option`] because fields in tuple-like
/// objects don't have names, and so you can't assume that a field will have a name
/// when working with generic fields.
///
/// See [`Variant::fields`] for more.
///
/// [`Variant::fields`]: ./struct.Variant.html#method.fields
/// [`name`]: #structfield.name
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field<'a> {
    pub name: Option<&'a str>,
    pub schema: &'a Schema,
}

impl<'a> Field<'a> {
    pub fn new(name: Option<&'a str>, schema: &'a Schema) -> Self {
        Self { name, schema }
    }

    pub fn named(name: &'a str, schema: &'a Schema) -> Self {
        Self {
            name: Some(name),
            schema,
        }
    }

    pub fn unnamed(schema: &'a Schema) -> Self {
        Self { name: None, schema }
    }
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
        $crate::TypeName::new(stringify!($ty), module_path!())
    };
}
