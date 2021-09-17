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
    Warning(Option<Warning>),
    #[class("02")]
    NoData(Option<NoData>),
    #[class("07")]
    DynamicSqlError(Option<DynamicSqlError>),
    #[class("08")]
    ConnectionException(Option<ConnectionException>),
    #[class("09")]
    TriggeredActionException,
    #[class("0A")]
    FeatureNotSupported(Option<FeatureNotSupported>),
    #[class("0D")]
    InvalidTargetTypeSpecification,
    #[class("0E")]
    InvalidSchemaNameListSpecification,
    #[class("0F")]
    LocatorException(Option<LocatorException>),
    #[class("0K")]
    ResignalWhenHandlerNotActive,
    #[class("0L")]
    InvalidGrantor,
    #[class("0M")]
    InvalidSqlInvokedProcedureReference,
    #[class("0N")]
    SqlXmlMappingError(Option<SqlXmlMappingError>),
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
    ProhibitedStatementDuringTriggerExecution(Option<ProhibitedStatementDuringTriggerExecution>),
    #[class("0X")]
    InvalidForeignServerSpecification,
    #[class("0Y")]
    PassthroughSpecificCondition(Option<PassthroughSpecificCondition>),
    #[class("0Z")]
    DiagnosticsException(Option<DiagnosticsException>),
    #[class("10")]
    XQueryError,
    #[class("20")]
    CaseNotFoundForCaseStatement,
    #[class("21")]
    CardinalityViolation,
    #[class("22")]
    DataException(Option<DataException>),
    #[class("23")]
    IntegrityConstraintViolation(Option<IntegrityConstraintViolation>),
    #[class("24")]
    InvalidCursorState,
    #[class("25")]
    InvalidTransactionState(Option<InvalidTransactionState>),
    #[class("26")]
    InvalidSqlStatementName,
    #[class("27")]
    TriggeredDataChangeViolation(Option<TriggeredDataChangeViolation>),
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
    SqlRoutineException(Option<SqlRoutineException>),
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
    CursorSensitivityException(Option<CursorSensitivityException>),
    #[class("38")]
    ExternalRoutineException(Option<ExternalRoutineException>),
    #[class("39")]
    ExternalRoutineInvocationException(Option<ExternalRoutineInvocationException>),
    #[class("3B")]
    SavepointException(Option<SavepointException>),
    #[class("3C")]
    AmbiguousCursorName,
    #[class("3D")]
    InvalidCatalogName,
    #[class("3F")]
    InvalidSchemaName,
    #[class("40")]
    TransactionRollback(Option<TransactionRollback>),
    #[class("42")]
    SyntaxErrorOrAccessRuleViolation,
    #[class("44")]
    WithCheckOptionViolation,
    #[class("45")]
    UnhandledUserDefinedException,
    #[class("46")]
    OlbSpecificError(Option<OlbSpecificError>),
    #[class("HW")]
    DatalinkException(Option<DatalinkException>),
    #[class("HV")]
    FdwSpecificCondition(Option<FdwSpecificCondition>),
    #[class("HY")]
    CliSpecificCondition(Option<CliSpecificCondition>),
    #[class("HZ")]
    RemoteDatabaseAccess(Option<RemoteDatabaseAccess>),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn check(state: &str, value: SqlState) {
        assert_eq!(state.parse::<SqlState>().unwrap(), value);
    }

    #[test]
    fn success() {
        check("00000", SqlState::Success);
    }

    #[test]
    fn warning() {
        check("01000", SqlState::Warning(None));
        check(
            "01005",
            SqlState::Warning(Some(Warning::InsufficientItemDescriptorAreas)),
        );
        check(
            "0100A",
            SqlState::Warning(Some(Warning::QueryExpressionTooLongForInformationSchema)),
        );
        check(
            "0102F",
            SqlState::Warning(Some(Warning::ArrayDataRightTruncation)),
        );
    }

    #[test]
    fn no_data() {
        check("02000", SqlState::NoData(None));
        check(
            "02001",
            SqlState::NoData(Some(NoData::NoAdditionalResultSetsReturned)),
        );
    }
}
