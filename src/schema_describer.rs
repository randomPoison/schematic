use crate::{
    describe::{Describe, Describer},
    schema::*,
    TypeName,
};

pub struct SchemaDescriber;

impl<'a> Describer for &'a mut SchemaDescriber {
    type Ok = Schema;
    type Error = ();

    fn describe_bool(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Bool)
    }

    fn describe_i8(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::I8)
    }

    fn describe_i16(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::I16)
    }

    fn describe_i32(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::I32)
    }

    fn describe_i64(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::I64)
    }

    fn describe_i128(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::I128)
    }

    fn describe_u8(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::U8)
    }

    fn describe_u16(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::U16)
    }

    fn describe_u32(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::U32)
    }

    fn describe_u64(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::U64)
    }

    fn describe_u128(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::I128)
    }

    fn describe_f32(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::F32)
    }

    fn describe_f64(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::F64)
    }

    fn describe_char(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Char)
    }

    fn describe_string(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::String)
    }

    fn describe_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Unit)
    }

    fn describe_option<T>(self) -> Result<Self::Ok, Self::Error>
    where
        T: Describe,
    {
        let inner = T::describe(self)?;
        Ok(Schema::Option(Box::new(inner)))
    }

    fn describe_unit_struct(self, name: TypeName) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::UnitStruct(UnitStruct { name }))
    }

    fn describe_newtype_struct<T>(self, name: TypeName) -> Result<Self::Ok, Self::Error>
    where
        T: Describe,
    {
        let inner = T::describe(self)?;
        Ok(Schema::NewtypeStruct(Box::new(NewtypeStruct {
            name: name.into(),
            inner,
        })))
    }

    fn describe_enum<T>(self, _name: TypeName) -> Result<Self::Ok, Self::Error>
    where
        T: Describe,
    {
        unimplemented!()
    }

    fn describe_tuple(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn describe_tuple_struct(self, _name: TypeName) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn describe_seq<T>(self) -> Result<Self::Ok, Self::Error>
    where
        T: Describe,
    {
        unimplemented!()
    }

    fn describe_map<K, V>(self) -> Result<Self::Ok, Self::Error>
    where
        K: Describe,
        V: Describe,
    {
        unimplemented!()
    }

    fn describe_struct(self, _name: TypeName) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}
