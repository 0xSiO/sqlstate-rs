#![warn(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! Representations and parsing logic for [`SQLSTATE`](https://en.wikipedia.org/wiki/SQLSTATE)
//! return codes.
//!
//! # Example
//!
//! ```
//! use sqlstate::{
//!    standard::{
//!        class::{DataException::DivisionByZero, Warning::PrivilegeNotGranted},
//!        SqlState,
//!    },
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! assert_eq!("00000".parse::<SqlState>()?, SqlState::Success(None));
//! assert_eq!("01007".parse::<SqlState>()?, SqlState::Warning(Some(PrivilegeNotGranted)));
//! assert_eq!("22012".parse::<SqlState>()?, SqlState::DataException(Some(DivisionByZero)));
//! assert_eq!("XX102".parse::<SqlState>()?, SqlState::Other(String::from("XX102")));
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Category {
    Success,
    Warning,
    NoData,
    Exception,
}
