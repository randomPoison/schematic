use schematic::{type_name, Enum, Schema, Variant};

#[test]
fn test_empty_enum() {
    let schema = Enum {
        name: type_name!(MyEnum),
        repr: None,
        variants: vec![
            Variant::Unit {
                name: "Foo".into(),
                discriminant: None,
            },
            Variant::Struct {
                name: "Bar".into(),
                fields: Vec::new(),
            },
            Variant::Tuple {
                name: "Baz".into(),
                elements: Vec::new(),
            },
        ],
    };

    assert!(!schema.has_data());

    for variant in &schema.variants {
        assert!(variant.is_empty());
    }
}

#[test]
fn test_non_empty_enum() {
    let schema = Enum {
        name: type_name!(MyEnum),
        repr: None,
        variants: vec![
            Variant::Unit {
                name: "Foo".into(),
                discriminant: None,
            },
            Variant::Struct {
                name: "Bar".into(),
                fields: Vec::new(),
            },
            Variant::Tuple {
                name: "Baz".into(),
                elements: vec![Schema::I8],
            },
        ],
    };

    assert!(schema.has_data());
    assert!(!schema.variants[2].is_empty());
}
