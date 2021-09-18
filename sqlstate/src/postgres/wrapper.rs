use std::{convert::Infallible, str::FromStr};

use crate::{postgres, standard, Category};

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
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<postgres::SqlState>()
            .map_or_else(|_| Self::Standard(s.parse().unwrap()), Self::Custom))
    }
}
