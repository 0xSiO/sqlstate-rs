use sqlstate_macros::state;

pub mod class;

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
