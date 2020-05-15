use pretty_assertions::assert_eq;
use schematic::Schema;

macro_rules! describe_primitives {
    ( $( $prim:ty => $schema:ident, )* ) => {
        $(
            assert_eq!(Schema::$schema, schematic::describe::<$prim>());
        )*
    };
}

#[test]
fn describe_primitives() {
    describe_primitives! {
        u8 => U8,
        u16 => U16,
        u32 => U32,
        u64 => U64,
        u128 => U128,
        usize => USize,

        i8 => I8,
        i16 => I16,
        i32 => I32,
        i64 => I64,
        i128 => I128,
        isize => ISize,

        f32 => F32,
        f64 => F64,

        bool => Bool,
        char => Char,
        () => Unit,

        &str => Str,
    }
}
