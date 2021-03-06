use pretty_assertions::assert_eq;
use schematic::*;

pub struct ManualStruct {
    pub field: bool,
    pub another: u32,
}

impl Describe for ManualStruct {
    fn type_name() -> TypeName {
        schematic::type_name!(ManualStruct)
    }

    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        let mut describer = describer.describe_struct(Self::type_name())?;
        describer.describe_field::<bool>("field")?;
        describer.describe_field::<u32>("another")?;
        describer.end()
    }
}

#[test]
fn describe_struct() {
    let actual = schematic::describe::<ManualStruct>();

    let expected = Schema::Struct(Struct {
        name: schematic::type_name!(ManualStruct),
        fields: vec![
            ("field".into(), Schema::Bool),
            ("another".into(), Schema::U32),
        ],
    });

    assert_eq!(expected, actual);
}

pub struct ManualTupleStruct(bool, u32);

impl Describe for ManualTupleStruct {
    fn type_name() -> TypeName {
        schematic::type_name!(ManualTupleStruct)
    }

    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        let mut describer = describer.describe_tuple_struct(Self::type_name())?;
        describer.describe_element::<bool>()?;
        describer.describe_element::<u32>()?;
        describer.end()
    }
}

// Test that nested structs can be described fully. The original design for
// describing structs was broken, such that struct fields that were user-defined
// types wouldn't be fully described. This test case verifies that the current
// design is setup to correctly handle this case.
//
// TODO: This test doesn't need to be done with a manual `Describe` impl, update it
// to use the derive once it's implemented.

#[test]
fn describe_tuple_struct() {
    let actual = schematic::describe::<ManualTupleStruct>();

    let expected = Schema::TupleStruct(TupleStruct {
        name: schematic::type_name!(ManualTupleStruct),
        elements: vec![Schema::Bool, Schema::U32],
    });

    assert_eq!(expected, actual);
}

pub struct NestedStruct {
    pub manual_struct: ManualStruct,
    pub tuple_struct: ManualTupleStruct,
}

impl Describe for NestedStruct {
    fn type_name() -> TypeName {
        schematic::type_name!(NestedStruct)
    }

    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        let mut describer = describer.describe_struct(Self::type_name())?;
        describer.describe_field::<ManualStruct>("manual_struct")?;
        describer.describe_field::<ManualTupleStruct>("tuple_struct")?;
        describer.end()
    }
}

#[test]
fn test_nested_struct() {
    let actual = schematic::describe::<NestedStruct>();

    let expected = Schema::Struct(Struct {
        name: schematic::type_name!(NestedStruct),
        fields: vec![
            (
                "manual_struct".into(),
                Schema::Struct(Struct {
                    name: schematic::type_name!(ManualStruct),
                    fields: vec![
                        ("field".into(), Schema::Bool),
                        ("another".into(), Schema::U32),
                    ],
                }),
            ),
            (
                "tuple_struct".into(),
                Schema::TupleStruct(TupleStruct {
                    name: schematic::type_name!(ManualTupleStruct),
                    elements: vec![Schema::Bool, Schema::U32],
                }),
            ),
        ],
    });

    assert_eq!(expected, actual);
}
