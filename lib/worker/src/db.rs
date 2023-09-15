mod errors;
mod publications;

pub use errors::{Error, Result};
pub use publications::{
    export_into, is_namespace_owner, namespace_create, pub_table_create, pub_table_insert,
};
