use schematic::*;

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
