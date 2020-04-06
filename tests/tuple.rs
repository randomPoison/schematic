use schematic::*;

#[test]
fn describe_two_tuple() {
    let actual = schematic::describe::<(i32, bool)>().expect("Failed to describe tuple");

    let expected = Schema::Tuple(vec![Schema::I32, Schema::Bool]);

    assert_eq!(expected, actual);
}

#[test]
fn describe_nested_tuple() {
    let actual = schematic::describe::<(u8, (bool, bool), u8)>().expect("Failed to describe tuple");

    let expected = Schema::Tuple(vec![
        Schema::U8,
        Schema::Tuple(vec![Schema::Bool, Schema::Bool]),
        Schema::U8,
    ]);

    assert_eq!(expected, actual);
}

#[test]
fn describe_sixteen_tuple() {
    let actual = schematic::describe::<(
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
    )>()
    .expect("Failed to describe tuple");

    let expected = Schema::Tuple(vec![
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
        Schema::U8,
    ]);

    assert_eq!(expected, actual);
}
