//! These prelude re-exports are a set of exports that are commonly used from within the library.
//!
//! These are not publicly re-exported to the end user, and must stay as a private module.

pub use std::result::Result as StdResult;

#[cfg(feature = "client")]
pub use crate::client::ClientError;
pub use crate::error::{Error, Result};
pub use crate::json::{JsonMap, Value};
