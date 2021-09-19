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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_category() {
        use crate::postgres::{class::Warning, SqlState};

        assert_eq!(
            PostgresSqlState::Custom(SqlState::Warning(Some(Warning::DeprecatedFeature)))
                .category(),
            Category::Warning
        );
    }

    #[test]
    fn invalid_length() {
        for i in 0..5 {
            assert_eq!(
                "0".repeat(i).parse::<PostgresSqlState>(),
                Err(ParseError::InvalidLength(i))
            );
        }
    }

    #[test]
    fn fallback_standard_category() {
        use crate::standard::SqlState;

        assert_eq!(
            PostgresSqlState::Standard(SqlState::Success(None)).category(),
            Category::Success
        );
    }

    #[test]
    fn parsing_custom() {
        use crate::postgres::{class::OperatorIntervention, SqlState};

        assert_eq!(
            "57P01".parse::<PostgresSqlState>(),
            Ok(PostgresSqlState::Custom(SqlState::OperatorIntervention(
                Some(OperatorIntervention::AdminShutdown)
            )))
        );
    }

    #[test]
    fn parsing_standard() {
        use crate::standard::{class::Warning, SqlState};

        assert_eq!(
            "01002".parse::<PostgresSqlState>(),
            Ok(PostgresSqlState::Standard(SqlState::Warning(Some(
                Warning::DisconnectError
            ))))
        );
    }
}
