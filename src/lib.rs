//! This crate provides convenient traits for converting between TOML and serializable values.
//!
//! # Examples
//!
//! Converts from a TOML string to a serializable value:
//!
//! ```
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate tomlconv;
//!
//! use tomlconv::FromToml;
//!
//! // Defines a deserializable struct.
//! #[derive(Deserialize)]
//! struct Foo {
//!     bar: String,
//!     baz: usize
//! }
//! impl FromToml for Foo {}
//!
//! # fn main() {
//! // Converts from the TOML string to a `Foo` value.
//! let toml = r#"
//! bar = "aaa"
//! baz = 123
//! "#;
//! let foo = Foo::from_toml_str(toml).unwrap();
//! assert_eq!(foo.bar, "aaa");
//! assert_eq!(foo.baz, 123);
//! # }
//! ```
#![warn(missing_docs)]
extern crate serde;
extern crate toml;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use traits::{FromToml, ToToml};

mod error;
mod traits;

/// A specialized `Result` type for this crate.
pub type Result<T> = ::std::result::Result<T, Error>;
