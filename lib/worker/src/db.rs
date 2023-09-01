mod errors;
mod publications;

pub use errors::Result;
pub use publications::{is_namespace_owner, namespace_create, pub_table_create, pub_table_insert};
