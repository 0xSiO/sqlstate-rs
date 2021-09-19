#![warn(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! Representations and parsing logic for [`SQLSTATE`](https://en.wikipedia.org/wiki/SQLSTATE)
//! return codes.
//!
//! # Examples
//!
//! Parsing return codes according to the SQL standard:
//! ```
//! use sqlstate::standard::{
//!     class::{DataException::DivisionByZero, Warning::PrivilegeNotGranted},
//!     SqlState,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! assert_eq!("00000".parse::<SqlState>()?, SqlState::Success(None));
//! assert_eq!("01007".parse::<SqlState>()?, SqlState::Warning(Some(PrivilegeNotGranted)));
//!
//! // Unknown codes are represented as `Other`
//! assert_eq!("XX001".parse::<SqlState>()?, SqlState::Other(String::from("XX001")));
//! #     Ok(())
//! # }
//! ```
//!
//! Examining the pieces of a return code:
//! ```
//! use sqlstate::standard::{class::Warning::PrivilegeNotGranted, SqlState};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let success = SqlState::Success(None);
//! let warning = SqlState::Warning(Some(PrivilegeNotGranted));
//! assert_eq!((success.class(), success.subclass()), ("00", None));
//! assert_eq!((warning.class(), warning.subclass()), ("01", Some("007")));
//! #     Ok(())
//! # }
//! ```
//!
//! Parsing return codes specific to PostgreSQL:
//! ```
//! use sqlstate::{
//!     postgres::{
//!         class::{DataException::InvalidJsonText, InternalError::DataCorrupted},
//!         SqlState::*,
//!     },
//!     standard, PostgresSqlState,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! assert_eq!("22032".parse::<PostgresSqlState>()?,
//!            PostgresSqlState::Custom(DataException(Some(InvalidJsonText))));
//! assert_eq!("XX001".parse::<PostgresSqlState>()?,
//!            PostgresSqlState::Custom(InternalError(Some(DataCorrupted))));
//!
//! // Can also fall back to standard codes
//! assert_eq!("00000".parse::<PostgresSqlState>()?,
//!            PostgresSqlState::Standard(standard::SqlState::Success(None)));
//! #     Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - `postgres`: Enables PostgreSQL-specific types.

pub mod error;
pub mod standard;

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres;

#[cfg(feature = "postgres")]
pub use postgres::wrapper::PostgresSqlState;

pub use standard::SqlState;

/// A category for a given `SQLSTATE` code.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Category {
    Success,
    Warning,
    NoData,
    Exception,
}
