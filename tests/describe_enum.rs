use pretty_assertions::assert_eq;
use schematic::*;

#[allow(dead_code)]
enum Simple {
    Foo,
    Bar,
}

impl Describe for Simple {
    fn describe<D: Describer>(describer: D) -> std::result::Result<D::Ok, D::Error> {
        let mut describer = describer.describe_enum(schematic::type_name!(Simple))?;
        describer.describe_unit_variant("Foo", None)?;
        describer.describe_unit_variant("Bar", None)?;
        describer.end()
    }
}

#[test]
fn describe_simple_enum() {
    let actual = schematic::describe::<Simple>().expect("Failed to describe `Simple`");

    let expected = Schema::Enum(Enum {
        name: type_name!(Simple),
        repr: None,
        variants: vec![
            Variant::Unit {
                name: "Foo".into(),
                discriminant: None,
            },
            Variant::Unit {
                name: "Bar".into(),
                discriminant: None,
            },
        ],
    });

    assert_eq!(expected, actual);
}

#[allow(dead_code)]
enum WithData {
    Foo,
    Bar(usize, u32),
    Baz { first: bool, second: i8 },
}

impl Describe for WithData {
    fn describe<D: Describer>(describer: D) -> std::result::Result<D::Ok, D::Error> {
        let mut describer = describer.describe_enum(schematic::type_name!(WithData))?;

        describer.describe_unit_variant("Foo", None)?;

        {
            let mut variant_describer = describer.start_tuple_variant("Bar")?;
            variant_describer.describe_element::<usize>()?;
            variant_describer.describe_element::<u32>()?;
            describer.end_tuple_variant(variant_describer)?;
        }

        {
            let mut variant_describer = describer.start_struct_variant("Baz")?;
            variant_describer.describe_field::<bool>("first")?;
            variant_describer.describe_field::<i8>("second")?;
            describer.end_struct_variant(variant_describer)?;
        }

        describer.end()
    }
}

#[test]
fn describe_data_enum() {
    let actual = schematic::describe::<WithData>().expect("Failed to describe `WithData`");

    let expected = Schema::Enum(Enum {
        name: type_name!(WithData),
        repr: None,
        variants: vec![
            Variant::Unit {
                name: "Foo".into(),
                discriminant: None,
            },
            Variant::Tuple {
                name: "Bar".into(),
                elements: vec![Schema::USize, Schema::U32],
            },
            Variant::Struct {
                name: "Baz".into(),
                fields: vec![
                    ("first".into(), Schema::Bool),
                    ("second".into(), Schema::I8),
                ],
            },
        ],
    });

    assert_eq!(expected, actual);
}
