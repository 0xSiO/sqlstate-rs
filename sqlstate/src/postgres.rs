use sqlstate_macros::state;

pub mod class;

use self::class::*;

// TODO: For FromStr, throw error if unable to parse rather than returning Other("...")
//       If error, then try parsing standard SqlState instead
pub enum SqlState {
    Standard(crate::standard::SqlState),
    Custom(PostgresSqlState),
}

#[state(non_standard)]
#[non_exhaustive]
pub enum PostgresSqlState {
    #[class("01")]
    Warning(Warning),
    #[class("03")]
    SqlStatementNotYetComplete,
    #[class("08")]
    ConnectionException(ConnectionException),
    #[class("0B")]
    InvalidTransactionInitiation,
    #[class("0L")]
    InvalidGrantor(InvalidGrantor),
    #[class("22")]
    DataException(DataException),
    #[class("23")]
    IntegrityConstraintViolation(IntegrityConstraintViolation),
    #[class("25")]
    InvalidTransactionState(InvalidTransactionState),
    #[class("28")]
    InvalidAuthorizationSpecification(InvalidAuthorizationSpecification),
    #[class("2B")]
    DependentPrivilegeDescriptorsExist(DependentPrivilegeDescriptorsExist),
    #[class("39")]
    ExternalRoutineInvocationException(ExternalRoutineInvocationException),
    #[class("40")]
    TransactionRollback(TransactionRollback),
    #[class("42")]
    SyntaxErrorOrAccessRuleViolation(SyntaxErrorOrAccessRuleViolation),
    #[class("53")]
    InsufficientResources(InsufficientResources),
    #[class("54")]
    ProgramLimitExceeded(ProgramLimitExceeded),
    #[class("55")]
    ObjectNotInPrerequisiteState(ObjectNotInPrerequisiteState),
    #[class("57")]
    OperatorIntervention(OperatorIntervention),
    #[class("58")]
    SystemError(SystemError),
    #[class("72")]
    SnapshotFailure,
    #[class("F0")]
    ConfigurationFileError(ConfigurationFileError),
    #[class("P0")]
    PlPgSqlError(PlPgSqlError),
    #[class("XX")]
    InternalError(InternalError),
}
