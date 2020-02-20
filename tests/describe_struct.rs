use schematic::*;

struct ManualDescribeImpl {
    _field: String,
    _another: u32,
}

impl Describe for ManualDescribeImpl {
    fn describe<D: Describer>(describer: D) -> std::result::Result<D::Ok, D::Error> {
        let mut describer = describer.describe_struct(schematic::type_name!(ManualDescribeImpl))?;
        describer.describe_field::<String>("_field")?;
        describer.describe_field::<u32>("_another")?;
        describer.end()
    }
}

#[test]
fn describe_struct() {
    let actual = schematic::describe::<ManualDescribeImpl>()
        .expect("Failed to describe `ManualDescribeImpl`");

    let expected = Schema::Struct(Box::new(Struct {
        name: TypeName {
            name: "ManualDescribeImpl".into(),
            module: "describe_struct".into(),
        },
        fields: vec![
            ("_field".into(), Schema::String),
            ("_another".into(), Schema::U32),
        ],
    }));

    assert_eq!(expected, actual);
}
