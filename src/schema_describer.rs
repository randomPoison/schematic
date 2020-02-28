use crate::{
    describe::{Describe, Describer},
    schema::*,
    DescribeEnum, DescribeStruct, DescribeTuple, TypeName,
};
use std::borrow::Cow;

pub struct SchemaDescriber;

impl<'a> Describer for &'a mut SchemaDescriber {
    type Ok = Schema;
    type Error = ();

    type DescribeStruct = StructDescriber;
    type DescribeEnum = EnumDescriber;
    type DescribeTuple = TupleDescriber;

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

    fn describe_enum(self, type_name: TypeName) -> Result<Self::DescribeEnum, Self::Error> {
        Ok(EnumDescriber {
            type_name,
            variants: Vec::new(),
        })
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

    fn describe_struct(self, type_name: TypeName) -> Result<Self::DescribeStruct, Self::Error> {
        Ok(StructDescriber {
            type_name,
            fields: Vec::new(),
        })
    }
}

pub struct StructDescriber {
    type_name: TypeName,
    fields: Vec<(Cow<'static, str>, Schema)>,
}

impl DescribeStruct for StructDescriber {
    type Ok = Schema;
    type Error = ();

    fn describe_field<T: Describe>(&mut self, name: &'static str) -> Result<(), Self::Error> {
        let ty = crate::describe::<T>()?;
        self.fields.push((name.into(), ty));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Struct(Struct {
            name: self.type_name,
            fields: self.fields,
        }))
    }
}

pub struct TupleDescriber {
    elements: Vec<Schema>,
}

impl DescribeTuple for TupleDescriber {
    type Ok = Schema;
    type Error = ();

    fn describe_element<T: Describe>(&mut self) -> Result<(), Self::Error> {
        self.elements.push(crate::describe::<T>()?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Tuple(self.elements))
    }
}

pub struct EnumDescriber {
    type_name: TypeName,
    variants: Vec<Variant>,
}

impl DescribeEnum for EnumDescriber {
    type Ok = Schema;
    type Error = ();

    type DescribeStruct = StructDescriber;
    type DescribeTuple = TupleDescriber;

    fn describe_unit_variant(
        &mut self,
        name: &'static str,
        discriminant: Option<PrimitiveValue>,
    ) -> Result<(), Self::Error> {
        self.variants.push(Variant::Unit {
            name: name.into(),
            discriminant,
        });

        Ok(())
    }

    fn start_tuple_variant(
        &mut self,
        _name: &'static str,
    ) -> Result<Self::DescribeTuple, Self::Error> {
        unimplemented!()
    }

    fn end_tuple_variant(&mut self, _variant: Self::DescribeTuple) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn start_struct_variant(
        &mut self,
        _name: &'static str,
    ) -> Result<Self::DescribeStruct, Self::Error> {
        unimplemented!()
    }

    fn end_struct_variant(&mut self, _variant: Self::DescribeStruct) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Enum(Enum {
            name: self.type_name,
            repr: None,
            variants: self.variants,
        }))
    }
}
