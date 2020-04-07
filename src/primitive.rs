//! Implementations of `Describe` for primitives and types provided by the standard
//! library.

use crate::{describe::*, TypeName};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// Generates the `Describe` impl for primitives and collection types.
///
/// Forwards any specified generic parameters directly to the specified describe
/// function and automatically adds the `Describe` bound for generic parameters. For
/// example, invoking with `Collection<A, B> => describe_map` will expand to:
///
/// ```
/// impl<A, B> Describe for Collection<A, B> where A: Describe, B: Describe {
///     fn describe<E: Describer>(describer: D) -> Result<> {
///         describer.describe_map::<A, B>()
///     }
/// }
/// ```
macro_rules! impl_describe {
    ( $( $ty:ident $( < $( $generic:ident ),* > )? => $describe:ident, )* ) => {
        $(
            impl $( < $( $generic ),* > )? Describe for $ty $( < $( $generic ),* > where $( $generic: Describe ),* )? {
                fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
                    describer.$describe $( ::<$( $generic, )* >)?()
                }
            }
        )*
    }
}

impl_describe! {
    i8 => describe_i8,
    i16 => describe_i16,
    i32 => describe_i32,
    i64 => describe_i64,
    i128 => describe_i128,
    isize => describe_isize,
    u8 => describe_u8,
    u16 => describe_u16,
    u32 => describe_u32,
    u64 => describe_u64,
    u128 => describe_u128,
    usize => describe_usize,
    bool => describe_bool,
    char => describe_char,
    Option<T> => describe_option,
}

macro_rules! describe_seq {
    ( $( $ty:ident => $module:literal, )* ) => {
        $(
            impl<T> Describe for $ty<T> where T: Describe {
                fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
                    describer.describe_seq::<T>(TypeName::new(stringify!($ty), $module), None)
                }
            }
        )*
    };
}

describe_seq! {
    Vec => "alloc::vec",
    VecDeque => "alloc::collections::vec_deque",
    HashSet => "std::collections::hash_set::HashSet",
    BTreeSet => "alloc::collections::btree_set",
    BinaryHeap => "alloc::collections::binary_heap",
    LinkedList => "alloc::collections::LinkedList",
}

impl<K: Describe, V: Describe> Describe for HashMap<K, V> {
    fn describe<D>(describer: D) -> Result<D::Ok, D::Error>
    where
        D: Describer,
    {
        describer.describe_map::<K, V>(TypeName::new("HashMap", "std::collections::hash_map"))
    }
}

impl<K: Describe, V: Describe> Describe for BTreeMap<K, V> {
    fn describe<D>(describer: D) -> Result<D::Ok, D::Error>
    where
        D: Describer,
    {
        describer.describe_map::<K, V>(TypeName::new("BTreeMap", "alloc::collections::btree_map"))
    }
}

impl Describe for String {
    fn describe<D>(describer: D) -> Result<D::Ok, D::Error>
    where
        D: Describer,
    {
        describer.describe_string(TypeName::new("String", "alloc::string"))
    }
}

impl Describe for () {
    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        describer.describe_unit()
    }
}

/// Generates the `Describe` impl for tuples of different arity.
///
/// The generated impl will call `describe_tuple`, and then call `describe_element`
/// for each type param.
macro_rules! describe_tuple {
    ( $($ty:ident),* ) => {
        impl<$( $ty, )*> Describe for ($( $ty, )*) where $( $ty: Describe, )* {
            fn describe<Desc: Describer>(describer: Desc) -> Result<Desc::Ok, Desc::Error> {
                let mut describer = describer.describe_tuple()?;
                $(
                    describer.describe_element::<$ty>()?;
                )*
                describer.end()
            }
        }
    }
}

describe_tuple!(A);
describe_tuple!(A, B);
describe_tuple!(A, B, C);
describe_tuple!(A, B, C, D);
describe_tuple!(A, B, C, D, E);
describe_tuple!(A, B, C, D, E, F);
describe_tuple!(A, B, C, D, E, F, G);
describe_tuple!(A, B, C, D, E, F, G, H);
describe_tuple!(A, B, C, D, E, F, G, H, I);
describe_tuple!(A, B, C, D, E, F, G, H, I, J);
describe_tuple!(A, B, C, D, E, F, G, H, I, J, K);
describe_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
describe_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
describe_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
describe_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
describe_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

impl<'a, T> Describe for &'a [T]
where
    T: Describe,
{
    fn describe<D>(describer: D) -> Result<D::Ok, D::Error>
    where
        D: Describer,
    {
        describer.describe_slice::<T>()
    }
}

/// Generates the `Describe` impl for arrays of different length.
///
/// For array types like `[T; 12]`, each length of array is considered a different
/// type. Right now there's no way to generically implement a trait for all lengths
/// of an array, so this macro provides a way to manually implement `Describe` for
/// different lengths of array.
macro_rules! describe_array {
    ( $( $len:expr ),* ) => {
        $(
            impl<T> Describe for [T; $len] where T: Describe {
                fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
                    describer.describe_array::<T>($len)
                }
            }
        )*
    }
}

describe_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
);
