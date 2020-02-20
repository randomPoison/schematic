use crate::TypeName;

// A *data structure* that can be described by schematic.
pub trait Describe: Sized {
    fn describe<D>(describer: D) -> Result<D::Ok, D::Error>
    where
        D: Describer;
}

/// A *schema format* that can describe any data structure supported by schematic.
pub trait Describer: Sized {
    type Ok;
    type Error;

    type DescribeStruct: DescribeStruct<Ok = Self::Ok, Error = Self::Error>;

    fn describe_bool(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i8(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i16(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i32(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i64(self) -> Result<Self::Ok, Self::Error>;
    fn describe_i128(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u8(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u16(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u32(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u64(self) -> Result<Self::Ok, Self::Error>;
    fn describe_u128(self) -> Result<Self::Ok, Self::Error>;
    fn describe_f32(self) -> Result<Self::Ok, Self::Error>;
    fn describe_f64(self) -> Result<Self::Ok, Self::Error>;
    fn describe_char(self) -> Result<Self::Ok, Self::Error>;
    fn describe_string(self) -> Result<Self::Ok, Self::Error>;
    fn describe_unit(self) -> Result<Self::Ok, Self::Error>;

    fn describe_option<T>(self) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    fn describe_unit_struct(self, name: TypeName) -> Result<Self::Ok, Self::Error>;

    fn describe_enum<T>(self, name: TypeName) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    fn describe_newtype_struct<T>(self, name: TypeName) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    fn describe_seq<T>(self) -> Result<Self::Ok, Self::Error>
    where
        T: Describe;

    fn describe_tuple(self) -> Result<Self::Ok, Self::Error>;

    fn describe_tuple_struct(self, name: TypeName) -> Result<Self::Ok, Self::Error>;

    fn describe_map<K, V>(self) -> Result<Self::Ok, Self::Error>
    where
        K: Describe,
        V: Describe;

    fn describe_struct(self, name: TypeName) -> Result<Self::DescribeStruct, Self::Error>;
}

pub trait DescribeStruct {
    type Ok;
    type Error;

    fn describe_field<T: Describe>(&mut self, name: &'static str) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
