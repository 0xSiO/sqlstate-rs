use sqlstate_macros::state;

use crate::Category;

pub mod class;
pub(crate) mod wrapper;

use self::class::*;

#[state(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SqlState {
    #[class("01")]
    Warning(Option<Warning>),
    #[class("03")]
    SqlStatementNotYetComplete(Option<SqlStatementNotYetComplete>),
    #[class("08")]
    ConnectionException(Option<ConnectionException>),
    #[class("0B")]
    InvalidTransactionInitiation(Option<InvalidTransactionInitiation>),
    #[class("0L")]
    InvalidGrantor(Option<InvalidGrantor>),
    #[class("22")]
    DataException(Option<DataException>),
    #[class("23")]
    IntegrityConstraintViolation(Option<IntegrityConstraintViolation>),
    #[class("25")]
    InvalidTransactionState(Option<InvalidTransactionState>),
    #[class("28")]
    InvalidAuthorizationSpecification(Option<InvalidAuthorizationSpecification>),
    #[class("2B")]
    DependentPrivilegeDescriptorsExist(Option<DependentPrivilegeDescriptorsExist>),
    #[class("39")]
    ExternalRoutineInvocationException(Option<ExternalRoutineInvocationException>),
    #[class("40")]
    TransactionRollback(Option<TransactionRollback>),
    #[class("42")]
    SyntaxErrorOrAccessRuleViolation(Option<SyntaxErrorOrAccessRuleViolation>),
    #[class("53")]
    InsufficientResources(Option<InsufficientResources>),
    #[class("54")]
    ProgramLimitExceeded(Option<ProgramLimitExceeded>),
    #[class("55")]
    ObjectNotInPrerequisiteState(Option<ObjectNotInPrerequisiteState>),
    #[class("57")]
    OperatorIntervention(Option<OperatorIntervention>),
    #[class("58")]
    SystemError(Option<SystemError>),
    #[class("72")]
    SnapshotFailure(Option<SnapshotFailure>),
    #[class("F0")]
    ConfigurationFileError(Option<ConfigurationFileError>),
    #[class("P0")]
    PlPgSqlError(Option<PlPgSqlError>),
    #[class("XX")]
    InternalError(Option<InternalError>),
}

impl SqlState {
    pub fn category(&self) -> Category {
        match self {
            Self::Warning(_) => Category::Warning,
            _ => Category::Exception,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::error::ParseError;

    fn check(state: &str, value: SqlState) {
        assert_eq!(state.parse::<SqlState>().unwrap(), value);
    }

    #[test]
    fn class() {
        assert_eq!(SqlState::Warning(None).class(), "01");
        assert_eq!(SqlState::InternalError(None).class(), "XX");
    }

    #[test]
    fn subclass() {
        assert_eq!(SqlState::SnapshotFailure(None).subclass(), None);
        assert_eq!(
            SqlState::Warning(Some(Warning::DeprecatedFeature)).subclass(),
            Some("P01")
        );
        assert_eq!(
            SqlState::InternalError(Some(InternalError::DataCorrupted)).subclass(),
            Some("001")
        );
    }

    #[test]
    fn category() {
        assert_eq!(
            SqlState::Warning(Some(Warning::ImplicitZeroBitPadding)).category(),
            Category::Warning
        );
        assert_eq!(
            SqlState::InternalError(Some(InternalError::IndexCorrupted)).category(),
            Category::Exception
        );
    }

    #[test]
    fn invalid_length() {
        for i in 0..5 {
            assert_eq!(
                "0".repeat(i).parse::<SqlState>(),
                Err(ParseError::InvalidLength(i))
            );
        }
    }

    #[test]
    fn empty_class() {
        check("0B000", SqlState::InvalidTransactionInitiation(None));
        assert_eq!(
            "0B001".parse::<SqlState>(),
            Err(ParseError::UnknownSubclass(String::from("001"))),
        );
    }

    #[test]
    fn unknown_class() {
        assert_eq!(
            "QQ999".parse::<SqlState>(),
            Err(ParseError::UnknownState(String::from("QQ999"))),
        );
    }

    #[test]
    fn one_subclass() {
        check("F0000", SqlState::ConfigurationFileError(None));
        check(
            "F0001",
            SqlState::ConfigurationFileError(Some(ConfigurationFileError::LockFileExists)),
        );
        assert_eq!(
            "F000F".parse::<SqlState>(),
            Err(ParseError::UnknownSubclass(String::from("00F"))),
        );
    }

    #[test]
    fn many_subclasses() {
        check("57000", SqlState::OperatorIntervention(None));
        check(
            "57014",
            SqlState::OperatorIntervention(Some(OperatorIntervention::QueryCanceled)),
        );
        check(
            "57P01",
            SqlState::OperatorIntervention(Some(OperatorIntervention::AdminShutdown)),
        );
        check(
            "57P03",
            SqlState::OperatorIntervention(Some(OperatorIntervention::CannotConnectNow)),
        );
        assert_eq!(
            "57FFF".parse::<SqlState>(),
            Err(ParseError::UnknownSubclass(String::from("FFF"))),
        );
    }
}
