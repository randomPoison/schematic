use crate::{describe::*, schema::*, TypeName};
use std::borrow::Cow;

pub struct SchemaDescriber;

impl<'a> Describer for &'a mut SchemaDescriber {
    type Ok = Schema;
    type Error = ();

    type DescribeStruct = StructDescriber;
    type DescribeTupleStruct = TupleStructDescriber;
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

    fn describe_isize(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::ISize)
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
        Ok(Schema::U128)
    }

    fn describe_usize(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::USize)
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

    fn describe_str(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Str)
    }

    fn describe_string(self, name: TypeName) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::String(name))
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

    fn describe_tuple(self) -> Result<Self::DescribeTuple, Self::Error> {
        Ok(Default::default())
    }

    fn describe_tuple_struct(
        self,
        _name: TypeName,
    ) -> Result<Self::DescribeTupleStruct, Self::Error> {
        Ok(TupleStructDescriber {
            type_name: _name,
            elements: Vec::new(),
        })
    }

    fn describe_array<T>(self, len: usize) -> Result<Self::Ok, Self::Error>
    where
        T: Describe,
    {
        Ok(Schema::Array(Box::new(Array {
            element: T::describe(self)?,
            len,
        })))
    }

    fn describe_slice<T>(self) -> Result<Self::Ok, Self::Error>
    where
        T: Describe,
    {
        Ok(Schema::Slice(Box::new(T::describe(self)?)))
    }

    fn describe_seq<T>(self, name: TypeName, len: Option<usize>) -> Result<Self::Ok, Self::Error>
    where
        T: Describe,
    {
        let element = T::describe(self)?;
        Ok(Schema::Seq(Box::new(Sequence { name, element, len })))
    }

    fn describe_map<K, V>(self, name: TypeName) -> Result<Self::Ok, Self::Error>
    where
        K: Describe,
        V: Describe,
    {
        Ok(Schema::Map(Box::new(Map {
            name,
            key: K::describe(&mut *self)?,
            value: V::describe(&mut *self)?,
        })))
    }

    fn describe_struct(self, type_name: TypeName) -> Result<Self::DescribeStruct, Self::Error> {
        Ok(StructDescriber {
            type_name,
            fields: Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TupleStructDescriber {
    type_name: TypeName,
    elements: Vec<Schema>,
}

impl DescribeTupleStruct for TupleStructDescriber {
    type Ok = Schema;
    type Error = ();

    fn describe_element<T: Describe>(&mut self) -> Result<(), Self::Error> {
        self.elements.push(crate::describe::<T>()?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::TupleStruct(TupleStruct {
            name: self.type_name,
            elements: self.elements,
        }))
    }
}

#[derive(Debug, Clone, Default)]
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

    type DescribeStructVariant = StructVariantDescriber;
    type DescribeTupleVariant = TupleVariantDescriber;

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
        name: &'static str,
    ) -> Result<Self::DescribeTupleVariant, Self::Error> {
        Ok(TupleVariantDescriber {
            name,
            elements: Default::default(),
        })
    }

    fn end_tuple_variant(
        &mut self,
        variant: Self::DescribeTupleVariant,
    ) -> Result<(), Self::Error> {
        self.variants.push(Variant::Tuple {
            name: variant.name.into(),
            elements: variant.elements,
        });

        Ok(())
    }

    fn start_struct_variant(
        &mut self,
        name: &'static str,
    ) -> Result<Self::DescribeStructVariant, Self::Error> {
        Ok(StructVariantDescriber {
            name,
            fields: Default::default(),
        })
    }

    fn end_struct_variant(
        &mut self,
        variant: Self::DescribeStructVariant,
    ) -> Result<(), Self::Error> {
        self.variants.push(Variant::Struct {
            name: variant.name.into(),
            fields: variant.fields,
        });

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Schema::Enum(Enum {
            name: self.type_name,
            repr: None,
            variants: self.variants,
        }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TupleVariantDescriber {
    name: &'static str,
    elements: Vec<Schema>,
}

impl DescribeTupleVariant for TupleVariantDescriber {
    type Error = ();

    fn describe_element<T: Describe>(&mut self) -> Result<(), Self::Error> {
        self.elements.push(crate::describe::<T>()?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StructVariantDescriber {
    name: &'static str,
    fields: Vec<(Cow<'static, str>, Schema)>,
}

impl DescribeStructVariant for StructVariantDescriber {
    type Error = ();

    fn describe_field<T: Describe>(&mut self, name: &'static str) -> Result<(), Self::Error> {
        let ty = crate::describe::<T>()?;
        self.fields.push((name.into(), ty));
        Ok(())
    }
}
