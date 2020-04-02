use pretty_assertions::assert_eq;
use schematic::*;

pub struct ManualStruct {
    pub field: String,
    pub another: u32,
}

impl Describe for ManualStruct {
    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        let mut describer = describer.describe_struct(schematic::type_name!(ManualStruct))?;
        describer.describe_field::<String>("field")?;
        describer.describe_field::<u32>("another")?;
        describer.end()
    }
}

#[test]
fn describe_struct() {
    let actual = schematic::describe::<ManualStruct>().unwrap();

    let expected = Schema::Struct(Struct {
        name: schematic::type_name!(ManualStruct),
        fields: vec![
            ("field".into(), Schema::String),
            ("another".into(), Schema::U32),
        ],
    });

    assert_eq!(expected, actual);
}

pub struct ManualTupleStruct(String, u32);

impl Describe for ManualTupleStruct {
    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        let mut describer =
            describer.describe_tuple_struct(schematic::type_name!(ManualTupleStruct))?;
        describer.describe_element::<String>()?;
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
    let actual = schematic::describe::<ManualTupleStruct>().unwrap();

    let expected = Schema::TupleStruct(TupleStruct {
        name: schematic::type_name!(ManualTupleStruct),
        elements: vec![Schema::String, Schema::U32],
    });

    assert_eq!(expected, actual);
}

pub struct NestedStruct {
    pub manual_struct: ManualStruct,
    pub tuple_struct: ManualTupleStruct,
}

impl Describe for NestedStruct {
    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        let mut describer = describer.describe_struct(schematic::type_name!(NestedStruct))?;
        describer.describe_field::<ManualStruct>("manual_struct")?;
        describer.describe_field::<ManualTupleStruct>("tuple_struct")?;
        describer.end()
    }
}

#[test]
fn test_nested_struct() {
    let actual = schematic::describe::<NestedStruct>().unwrap();

    let expected = Schema::Struct(Struct {
        name: schematic::type_name!(NestedStruct),
        fields: vec![
            (
                "manual_struct".into(),
                Schema::Struct(Struct {
                    name: schematic::type_name!(ManualStruct),
                    fields: vec![
                        ("field".into(), Schema::String),
                        ("another".into(), Schema::U32),
                    ],
                }),
            ),
            (
                "tuple_struct".into(),
                Schema::TupleStruct(TupleStruct {
                    name: schematic::type_name!(ManualTupleStruct),
                    elements: vec![Schema::String, Schema::U32],
                }),
            ),
        ],
    });

    assert_eq!(expected, actual);
}
