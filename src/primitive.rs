//! Implementations of `Describe` for primitives and types provided by the standard libary.

use crate::describe::*;
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
    u8 => describe_u8,
    u16 => describe_u16,
    u32 => describe_u32,
    u64 => describe_u64,
    u128 => describe_u128,
    bool => describe_bool,
    char => describe_char,
    String => describe_string,
    Option<T> => describe_option,
    Vec<T> => describe_seq,
    VecDeque<T> => describe_seq,
    HashMap<K, V> => describe_map,
    BTreeMap<K, V> => describe_map,
    HashSet<T> => describe_seq,
    BTreeSet<T> => describe_seq,
    BinaryHeap<T> => describe_seq,
    LinkedList<T> => describe_seq,
}

impl Describe for () {
    fn describe<D: Describer>(describer: D) -> Result<D::Ok, D::Error> {
        describer.describe_unit()
    }
}

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
