use std::str::FromStr;

use crate::{error::ParseError, postgres, standard, Category};

#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostgresSqlState {
    Standard(standard::SqlState),
    Custom(postgres::SqlState),
}

impl PostgresSqlState {
    pub fn category(&self) -> Category {
        match self {
            Self::Standard(state) => state.category(),
            Self::Custom(state) => state.category(),
        }
    }
}

impl FromStr for PostgresSqlState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<postgres::SqlState>() {
            Ok(state) => Ok(Self::Custom(state)),
            Err(_) => Ok(Self::Standard(s.parse()?)),
        }
    }
}
