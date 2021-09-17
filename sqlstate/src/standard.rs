use sqlstate_macros::state;

pub mod class;

use self::class::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Category {
    Success,
    Warning,
    NoData,
    Exception,
}

#[state(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SqlState {
    #[class("00")]
    Success,
    #[class("01")]
    Warning(Warning),
    #[class("02")]
    NoData(NoData),
    #[class("07")]
    DynamicSqlError(DynamicSqlError),
    #[class("08")]
    ConnectionException(ConnectionException),
    #[class("09")]
    TriggeredActionException,
    #[class("0A")]
    FeatureNotSupported(FeatureNotSupported),
    #[class("0D")]
    InvalidTargetTypeSpecification,
    #[class("0E")]
    InvalidSchemaNameListSpecification,
    #[class("0F")]
    LocatorException(LocatorException),
    #[class("0K")]
    ResignalWhenHandlerNotActive,
    #[class("0L")]
    InvalidGrantor,
    #[class("0M")]
    InvalidSqlInvokedProcedureReference,
    #[class("0N")]
    SqlXmlMappingError(SqlXmlMappingError),
    #[class("0P")]
    InvalidRoleSpecification,
    #[class("0S")]
    InvalidTransformGroupNameSpecification,
    #[class("0T")]
    TargetTableDisagreesWithCursorSpecification,
    #[class("0U")]
    AttemptToAssignToNonUpdatableColumn,
    #[class("0V")]
    AttemptToAssignToOrderingColumn,
    #[class("0W")]
    ProhibitedStatementDuringTriggerExecution(ProhibitedStatementDuringTriggerExecution),
    #[class("0X")]
    InvalidForeignServerSpecification,
    #[class("0Y")]
    PassthroughSpecificCondition(PassthroughSpecificCondition),
    #[class("0Z")]
    DiagnosticsException(DiagnosticsException),
    #[class("10")]
    XQueryError,
    #[class("20")]
    CaseNotFoundForCaseStatement,
    #[class("21")]
    CardinalityViolation,
    #[class("22")]
    DataException(DataException),
    #[class("23")]
    IntegrityConstraintViolation(IntegrityConstraintViolation),
    #[class("24")]
    InvalidCursorState,
    #[class("25")]
    InvalidTransactionState(InvalidTransactionState),
    #[class("26")]
    InvalidSqlStatementName,
    #[class("27")]
    TriggeredDataChangeViolation(TriggeredDataChangeViolation),
    #[class("28")]
    InvalidAuthorizationSpecification,
    #[class("2B")]
    DependentPrivilegeDescriptorsExist,
    #[class("2C")]
    InvalidCharsetName,
    #[class("2D")]
    InvalidTransactionTermination,
    #[class("2E")]
    InvalidConnectionName,
    #[class("2F")]
    SqlRoutineException(SqlRoutineException),
    #[class("2H")]
    InvalidCollationName,
    #[class("30")]
    InvalidSqlStatementIdentifier,
    #[class("33")]
    InvalidSqlDescriptorName,
    #[class("34")]
    InvalidCursorName,
    #[class("35")]
    InvalidConditionNumber,
    #[class("36")]
    CursorSensitivityException(CursorSensitivityException),
    #[class("38")]
    ExternalRoutineException(ExternalRoutineException),
    #[class("39")]
    ExternalRoutineInvocationException(ExternalRoutineInvocationException),
    #[class("3B")]
    SavepointException(SavepointException),
    #[class("3C")]
    AmbiguousCursorName,
    #[class("3D")]
    InvalidCatalogName,
    #[class("3F")]
    InvalidSchemaName,
    #[class("40")]
    TransactionRollback(TransactionRollback),
    #[class("42")]
    SyntaxErrorOrAccessRuleViolation,
    #[class("44")]
    WithCheckOptionViolation,
    #[class("45")]
    UnhandledUserDefinedException,
    #[class("46")]
    OlbSpecificError(OlbSpecificError),
    #[class("HW")]
    DatalinkException(DatalinkException),
    #[class("HV")]
    FdwSpecificCondition(FdwSpecificCondition),
    #[class("HY")]
    CliSpecificCondition(CliSpecificCondition),
    #[class("HZ")]
    RemoteDatabaseAccess(RemoteDatabaseAccess),
}

impl SqlState {
    pub fn category(&self) -> Category {
        match self {
            Self::Success => Category::Success,
            Self::Warning(_) => Category::Warning,
            Self::NoData(_) => Category::NoData,
            _ => Category::Exception,
        }
    }
}
