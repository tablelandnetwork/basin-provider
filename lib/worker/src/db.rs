mod errors;
mod publications;
mod setup;

pub use errors::{Error, Result};
pub use publications::{is_namespace_owner, namespace_create, pub_table_create, pub_table_insert};
pub use setup::{drop, setup};
