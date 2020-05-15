use crate::{PrimitiveValue, TypeName};

// A *data structure* that can be described by schematic.
pub trait Describe: Sized {
    // The unique name identifying the type.
    fn type_name() -> TypeName;

    fn describe<D>(describer: D) -> Result<D::Ok, D::Error>
    where
        D: Describer;
}

/// A *schema format* that can describe any data structure supported by schematic.
pub trait Describer: Sized {
    type Ok;
    type Error;

    type DescribeStruct: DescribeStruct<Ok = Self::Ok, Error = Self::Error>;
    type DescribeTupleStruct: DescribeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
    type DescribeEnum: DescribeEnum<Ok = Self::Ok, Error = Self::Error>;
    type DescribeTuple: DescribeTuple<Ok = Self::Ok, Error = Self::Error>;

    fn describe_bool(self) -> Result<Self::Ok, Self::Error>;

    fn describe_i8(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i16(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i32(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i64(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i128(self) -> Result<Self::Ok, Self::Error>;
    fn describe_isize(self) -> Result<Self::Ok, Self::Error>;

    fn describe_u8(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u16(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u32(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u64(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u128(self) -> Result<Self::Ok, Self::Error>;
    fn describe_usize(self) -> Result<Self::Ok, Self::Error>;

    fn describe_f32(self) -> Result<Self::Ok, Self::Error>;
    fn describe_f64(self) -> Result<Self::Ok, Self::Error>;

    fn describe_char(self) -> Result<Self::Ok, Self::Error>;

    fn describe_str(self) -> Result<Self::Ok, Self::Error>;
    fn describe_string(self, name: TypeName) -> Result<Self::Ok, Self::Error>;

    fn describe_unit(self) -> Result<Self::Ok, Self::Error>;

    fn describe_tuple(self) -> Result<Self::DescribeTuple, Self::Error>;

    fn describe_option<T>(self) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    /// Describes a fixed-size array of length `len` and elements of type `T`.
    fn describe_array<T>(self, len: usize) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    /// Describes a slice with elements of type `T`.
    fn describe_slice<T>(self) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    /// Describes a custom type that logically represents a sequence of elements.
    ///
    /// `len` is an optional length for the type. `len` should only be specified if
    /// the length of the sequence can be known statically.
    fn describe_seq<T>(self, name: TypeName, len: Option<usize>) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    fn describe_map<K, V>(self, name: TypeName) -> Result<Self::Ok, Self::Error>
    where
        K: Describe,
        V: Describe;

    fn describe_enum(self, name: TypeName) -> Result<Self::DescribeEnum, Self::Error>;

    fn describe_unit_struct(self, name: TypeName) -> Result<Self::Ok, Self::Error>;

    fn describe_struct(self, name: TypeName) -> Result<Self::DescribeStruct, Self::Error>;

    fn describe_tuple_struct(
        self,
        name: TypeName,
    ) -> Result<Self::DescribeTupleStruct, Self::Error>;

    fn describe_newtype_struct<T>(self, name: TypeName) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;
}

pub trait DescribeTuple {
    type Ok;
    type Error;

    fn describe_element<T: Describe>(&mut self) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait DescribeStruct {
    type Ok;
    type Error;

    fn describe_field<T: Describe>(&mut self, name: &'static str) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait DescribeTupleStruct {
    type Ok;
    type Error;

    fn describe_element<T: Describe>(&mut self) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait DescribeEnum {
    type Ok;
    type Error;

    type DescribeStructVariant: DescribeStructVariant<Error = Self::Error>;
    type DescribeTupleVariant: DescribeTupleVariant<Error = Self::Error>;

    fn describe_unit_variant(
        &mut self,
        name: &'static str,
        discriminant: Option<PrimitiveValue>,
    ) -> Result<(), Self::Error>;

    fn start_tuple_variant(
        &mut self,
        name: &'static str,
    ) -> Result<Self::DescribeTupleVariant, Self::Error>;

    fn end_tuple_variant(&mut self, variant: Self::DescribeTupleVariant)
        -> Result<(), Self::Error>;

    fn start_struct_variant(
        &mut self,
        name: &'static str,
    ) -> Result<Self::DescribeStructVariant, Self::Error>;

    fn end_struct_variant(
        &mut self,
        variant: Self::DescribeStructVariant,
    ) -> Result<(), Self::Error>;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait DescribeTupleVariant {
    type Error;

    fn describe_element<T: Describe>(&mut self) -> Result<(), Self::Error>;
}

pub trait DescribeStructVariant {
    type Error;

    fn describe_field<T: Describe>(&mut self, name: &'static str) -> Result<(), Self::Error>;
}
