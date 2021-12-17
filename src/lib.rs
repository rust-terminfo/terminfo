#[macro_use]
extern crate nom;
extern crate phf;
extern crate fnv;

mod error;
pub use crate::error::{Error, Result};

/// Parsers for various formats.
pub mod parser;

/// String capability expansion.
#[macro_use]
pub mod expand;
pub use crate::expand::Expand;

/// Standard terminal capabilities.
pub mod capability;
pub use crate::capability::{Capability, Value};

mod database;
pub use crate::database::Database;

/// Constants to deal with name differences across terminfo and termcap.
pub mod names;
