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
    Success(Option<Success>),
    #[class("01")]
    Warning(Option<Warning>),
    #[class("02")]
    NoData(Option<NoData>),
    #[class("07")]
    DynamicSqlError(Option<DynamicSqlError>),
    #[class("08")]
    ConnectionException(Option<ConnectionException>),
    #[class("09")]
    TriggeredActionException(Option<TriggeredActionException>),
    #[class("0A")]
    FeatureNotSupported(Option<FeatureNotSupported>),
    #[class("0D")]
    InvalidTargetTypeSpecification(Option<InvalidTargetTypeSpecification>),
    #[class("0E")]
    InvalidSchemaNameListSpecification(Option<InvalidSchemaNameListSpecification>),
    #[class("0F")]
    LocatorException(Option<LocatorException>),
    #[class("0K")]
    ResignalWhenHandlerNotActive(Option<ResignalWhenHandlerNotActive>),
    #[class("0L")]
    InvalidGrantor(Option<InvalidGrantor>),
    #[class("0M")]
    InvalidSqlInvokedProcedureReference(Option<InvalidSqlInvokedProcedureReference>),
    #[class("0N")]
    SqlXmlMappingError(Option<SqlXmlMappingError>),
    #[class("0P")]
    InvalidRoleSpecification(Option<InvalidRoleSpecification>),
    #[class("0S")]
    InvalidTransformGroupNameSpecification(Option<InvalidTransformGroupNameSpecification>),
    #[class("0T")]
    TargetTableDisagreesWithCursorSpecification(
        Option<TargetTableDisagreesWithCursorSpecification>,
    ),
    #[class("0U")]
    AttemptToAssignToNonUpdatableColumn(Option<AttemptToAssignToNonUpdatableColumn>),
    #[class("0V")]
    AttemptToAssignToOrderingColumn(Option<AttemptToAssignToOrderingColumn>),
    #[class("0W")]
    ProhibitedStatementDuringTriggerExecution(Option<ProhibitedStatementDuringTriggerExecution>),
    #[class("0X")]
    InvalidForeignServerSpecification(Option<InvalidForeignServerSpecification>),
    #[class("0Y")]
    PassthroughSpecificCondition(Option<PassthroughSpecificCondition>),
    #[class("0Z")]
    DiagnosticsException(Option<DiagnosticsException>),
    #[class("10")]
    XQueryError(Option<XQueryError>),
    #[class("20")]
    CaseNotFoundForCaseStatement(Option<CaseNotFoundForCaseStatement>),
    #[class("21")]
    CardinalityViolation(Option<CardinalityViolation>),
    #[class("22")]
    DataException(Option<DataException>),
    #[class("23")]
    IntegrityConstraintViolation(Option<IntegrityConstraintViolation>),
    #[class("24")]
    InvalidCursorState(Option<InvalidCursorState>),
    #[class("25")]
    InvalidTransactionState(Option<InvalidTransactionState>),
    #[class("26")]
    InvalidSqlStatementName(Option<InvalidSqlStatementName>),
    #[class("27")]
    TriggeredDataChangeViolation(Option<TriggeredDataChangeViolation>),
    #[class("28")]
    InvalidAuthorizationSpecification(Option<InvalidAuthorizationSpecification>),
    #[class("2B")]
    DependentPrivilegeDescriptorsExist(Option<DependentPrivilegeDescriptorsExist>),
    #[class("2C")]
    InvalidCharsetName(Option<InvalidCharsetName>),
    #[class("2D")]
    InvalidTransactionTermination(Option<InvalidTransactionTermination>),
    #[class("2E")]
    InvalidConnectionName(Option<InvalidConnectionName>),
    #[class("2F")]
    SqlRoutineException(Option<SqlRoutineException>),
    #[class("2H")]
    InvalidCollationName(Option<InvalidCollationName>),
    #[class("30")]
    InvalidSqlStatementIdentifier(Option<InvalidSqlStatementIdentifier>),
    #[class("33")]
    InvalidSqlDescriptorName(Option<InvalidSqlDescriptorName>),
    #[class("34")]
    InvalidCursorName(Option<InvalidCursorName>),
    #[class("35")]
    InvalidConditionNumber(Option<InvalidConditionNumber>),
    #[class("36")]
    CursorSensitivityException(Option<CursorSensitivityException>),
    #[class("38")]
    ExternalRoutineException(Option<ExternalRoutineException>),
    #[class("39")]
    ExternalRoutineInvocationException(Option<ExternalRoutineInvocationException>),
    #[class("3B")]
    SavepointException(Option<SavepointException>),
    #[class("3C")]
    AmbiguousCursorName(Option<AmbiguousCursorName>),
    #[class("3D")]
    InvalidCatalogName(Option<InvalidCatalogName>),
    #[class("3F")]
    InvalidSchemaName(Option<InvalidSchemaName>),
    #[class("40")]
    TransactionRollback(Option<TransactionRollback>),
    #[class("42")]
    SyntaxErrorOrAccessRuleViolation(Option<SyntaxErrorOrAccessRuleViolation>),
    #[class("44")]
    WithCheckOptionViolation(Option<WithCheckOptionViolation>),
    #[class("45")]
    UnhandledUserDefinedException(Option<UnhandledUserDefinedException>),
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
            Self::Success(_) => Category::Success,
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
        check("00000", SqlState::Success(None));
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

    #[test]
    fn dynamic_sql_error() {
        check("07000", SqlState::DynamicSqlError(None));
        check(
            "07001",
            SqlState::DynamicSqlError(Some(
                DynamicSqlError::UsingClauseDoesNotMatchDynamicParameterSpecifications,
            )),
        );
        check(
            "07005",
            SqlState::DynamicSqlError(Some(
                DynamicSqlError::PreparedStatementNotACursorSpecification,
            )),
        );
        check(
            "0700B",
            SqlState::DynamicSqlError(Some(DynamicSqlError::DataTypeTransformFunctionViolation)),
        );
        check(
            "0700F",
            SqlState::DynamicSqlError(Some(DynamicSqlError::InvalidDatetimeIntervalCode)),
        );
    }

    #[test]
    fn connection_exception() {
        check("08000", SqlState::ConnectionException(None));
        check(
            "08002",
            SqlState::ConnectionException(Some(ConnectionException::ConnectionNameInUse)),
        );
        check(
            "08007",
            SqlState::ConnectionException(Some(ConnectionException::TransactionResolutionUnknown)),
        );
    }

    #[test]
    fn triggered_action_exception() {
        check("09000", SqlState::TriggeredActionException(None));
    }

    #[test]
    fn feature_not_supported() {
        check("0A000", SqlState::FeatureNotSupported(None));
        check(
            "0A001",
            SqlState::FeatureNotSupported(Some(FeatureNotSupported::MultipleServerTransactions)),
        );
    }

    #[test]
    fn invalid_target_type_specification() {
        check("0D000", SqlState::InvalidTargetTypeSpecification(None));
    }

    #[test]
    fn invalid_schema_name_list_specification() {
        check("0E000", SqlState::InvalidSchemaNameListSpecification(None));
    }
}
