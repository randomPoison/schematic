use schematic::*;

struct ManualDescribeImpl {
    pub field: String,
    pub another: u32,
}

impl Describe for ManualDescribeImpl {
    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        let mut describer = describer.describe_struct(schematic::type_name!(ManualDescribeImpl))?;
        describer.describe_field::<String>("field")?;
        describer.describe_field::<u32>("another")?;
        describer.end()
    }
}

#[test]
fn describe_struct() {
    let actual = schematic::describe::<ManualDescribeImpl>()
        .expect("Failed to describe `ManualDescribeImpl`");

    let expected = Schema::Struct(Struct {
        name: schematic::type_name!(ManualDescribeImpl),
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

#[test]
fn describe_tuple_struct() {
    let actual =
        schematic::describe::<ManualTupleStruct>().expect("Failed to describe `ManualTupleStruct`");

    let expected = Schema::TupleStruct(TupleStruct {
        name: schematic::type_name!(ManualTupleStruct),
        elements: vec![Schema::String, Schema::U32],
    });

    assert_eq!(expected, actual);
}
