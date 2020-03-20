use schematic::{type_name, Enum, Schema, Variant, VariantField};

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

#[test]
fn unit_variant_fields() {
    let variant = Variant::Unit {
        name: "Foo".into(),
        discriminant: None,
    };

    let mut fields = variant.fields();
    assert_eq!(None, fields.next());
}

#[test]
fn named_variant_fields() {
    let variant = Variant::Struct {
        name: "Bar".into(),
        fields: vec![("foo".into(), Schema::I8), ("bar".into(), Schema::String)],
    };

    let mut fields = variant.fields();
    assert_eq!(
        Some(VariantField {
            name: Some("foo"),
            schema: &Schema::I8
        }),
        fields.next()
    );
    assert_eq!(
        Some(VariantField {
            name: Some("bar"),
            schema: &Schema::String,
        }),
        fields.next()
    );
    assert_eq!(None, fields.next());
}

#[test]
fn unnamed_variant_fields() {
    let variant = Variant::Tuple {
        name: "Bar".into(),
        elements: vec![Schema::I8, Schema::String],
    };

    let mut fields = variant.fields();
    assert_eq!(
        Some(VariantField {
            name: None,
            schema: &Schema::I8
        }),
        fields.next()
    );
    assert_eq!(
        Some(VariantField {
            name: None,
            schema: &Schema::String,
        }),
        fields.next()
    );
    assert_eq!(None, fields.next());
}
