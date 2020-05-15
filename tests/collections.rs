use pretty_assertions::assert_eq;
use schematic::{Array, Schema, Sequence, TypeName};

#[test]
fn describe_vec() {
    let expected = Schema::Seq(Box::new(Sequence {
        name: TypeName::new("Vec", "alloc::vec"),
        element: Schema::U32,
        len: None,
    }));
    let actual = schematic::describe::<Vec<u32>>();
    assert_eq!(expected, actual);
}

#[test]
fn describe_array() {
    let expected = Schema::Array(Box::new(Array {
        element: Schema::U32,
        len: 8,
    }));
    assert_eq!(expected, schematic::describe::<[u32; 8]>());
}

#[test]
fn describe_slice() {
    let expected = Schema::Slice(Box::new(Schema::U32));
    let actual = schematic::describe::<&[u32]>();
    assert_eq!(expected, actual);
}
