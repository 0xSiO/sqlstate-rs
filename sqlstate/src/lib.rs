#![warn(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod standard;

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub mod postgres;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Category {
    Success,
    Warning,
    NoData,
    Exception,
}

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostgresSqlState {
    Standard(standard::SqlState),
    Custom(postgres::SqlState),
}

#[cfg(feature = "postgres")]
impl std::str::FromStr for PostgresSqlState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<postgres::SqlState>()
            .map_or_else(|_| Self::Standard(s.parse().unwrap()), Self::Custom))
    }
}
