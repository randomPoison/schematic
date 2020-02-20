use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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

/// Unique name for a type.
///
/// All types are uniquely identified by a combination of their name and the module
/// in which they were declared; Since two types with the same name cannot be
/// declared in the same module, `TypeName` is always sufficient to disambiguate
/// between two types with the same name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeName {
    /// The local name of the type.
    pub name: Cow<'static, str>,

    /// The path to the module where the type is declared, starting with the crate name.
    ///
    /// Note that this may not be the same module that the type is publicly exported
    /// from in the owning crate.
    pub module: Cow<'static, str>,
}
