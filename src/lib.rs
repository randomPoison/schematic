mod describe;
mod primitive;
mod schema;
mod schema_describer;

pub use crate::{describe::*, schema::*, schema_describer::*};

/// Describes `T` into an in-memory representation of the type tree.
pub fn describe<T: Describe>() -> Result<Schema, ()> {
    let mut describe = SchemaDescriber;
    T::describe(&mut describe)
}
