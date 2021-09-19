//! Abstractions for standard return codes.

use sqlstate_macros::state;

use crate::Category;

pub mod class;

use self::class::*;

/// A representation for a standard `SQLSTATE` code.
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

    use crate::error::ParseError;

    fn check(state: &str, value: SqlState) {
        assert_eq!(state.parse::<SqlState>().unwrap(), value);
    }

    #[test]
    fn class() {
        assert_eq!(SqlState::Success(None).class(), "00");
        assert_eq!(SqlState::Warning(None).class(), "01");
        assert_eq!(SqlState::DynamicSqlError(None).class(), "07");
    }

    #[test]
    fn subclass() {
        assert_eq!(SqlState::Success(None).subclass(), None);
        assert_eq!(
            SqlState::Warning(Some(Warning::InsufficientItemDescriptorAreas)).subclass(),
            Some("005")
        );
        assert_eq!(
            SqlState::DynamicSqlError(Some(DynamicSqlError::InvalidDataTarget)).subclass(),
            Some("00D")
        );
    }

    #[test]
    fn category() {
        assert_eq!(SqlState::Success(None).category(), Category::Success);
        assert_eq!(SqlState::Warning(None).category(), Category::Warning);
        assert_eq!(SqlState::NoData(None).category(), Category::NoData);
        assert_eq!(
            SqlState::DynamicSqlError(Some(DynamicSqlError::InvalidDataTarget)).category(),
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
        check("00000", SqlState::Success(None));
        check(
            "00001",
            SqlState::Success(Some(Success::Other(String::from("001")))),
        );
    }

    #[test]
    fn unknown_class() {
        check("QQ999", SqlState::Other(String::from("QQ999")));
    }

    #[test]
    fn one_subclass() {
        check("02000", SqlState::NoData(None));
        check(
            "02001",
            SqlState::NoData(Some(NoData::NoAdditionalResultSetsReturned)),
        );
        check(
            "0200F",
            SqlState::NoData(Some(NoData::Other(String::from("00F")))),
        );
    }

    #[test]
    fn many_subclasses() {
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
        check(
            "01FFF",
            SqlState::Warning(Some(Warning::Other(String::from("FFF")))),
        )
    }
}
