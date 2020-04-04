use pretty_assertions::assert_eq;
use schematic::Schema;

#[test]
fn describe_vec() {
    let expected = Schema::Seq(Box::new(Schema::U32));
    let actual = schematic::describe::<Vec<u32>>().unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn describe_array() {
    let expected = Schema::Seq(Box::new(Schema::U32));
    assert_eq!(expected, schematic::describe::<[u32; 0]>().unwrap());
    assert_eq!(expected, schematic::describe::<[u32; 1]>().unwrap());
    assert_eq!(expected, schematic::describe::<[u32; 2]>().unwrap());
    assert_eq!(expected, schematic::describe::<[u32; 4]>().unwrap());
    assert_eq!(expected, schematic::describe::<[u32; 8]>().unwrap());
    assert_eq!(expected, schematic::describe::<[u32; 16]>().unwrap());
    assert_eq!(expected, schematic::describe::<[u32; 32]>().unwrap());
}

#[test]
fn describe_slice() {
    let expected = Schema::Seq(Box::new(Schema::U32));
    let actual = schematic::describe::<&[u32]>().unwrap();
    assert_eq!(expected, actual);
}
