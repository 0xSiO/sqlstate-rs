pub mod error;
pub mod subclass;

use std::convert::TryFrom;

pub use self::{error::ParseError, subclass::*};

pub enum Category {
    Success,
    Warning,
    NoData,
    Exception,
}

#[non_exhaustive]
pub enum SqlState {
    Success,
    Warning(Warning),
    NoData(NoData),
    DynamicSqlError(DynamicSqlError),
    ConnectionException(ConnectionException),
    TriggeredActionException,
    FeatureNotSupported(FeatureNotSupported),
    InvalidTargetTypeSpecification,
    InvalidSchemaNameListSpecification,
    LocatorException(LocatorException),
    ResignalWhenHandlerNotActive,
    InvalidGrantor,
    InvalidSqlInvokedProcedureReference,
    SqlXmlMappingError(SqlXmlMappingError),
    InvalidRoleSpecification,
    InvalidTransformGroupNameSpecification,
    TargetTableDisagreesWithCursorSpecification,
    AttemptToAssignToNonUpdatableColumn,
    AttemptToAssignToOrderingColumn,
    ProhibitedStatementDuringTriggerExecution(ProhibitedStatementDuringTriggerExecution),
    InvalidForeignServerSpecification,
    PassthroughSpecificCondition(PassthroughSpecificCondition),
    DiagnosticsException(DiagnosticsException),
    XQueryError,
    CaseNotFoundForCaseStatement,
    CardinalityViolation,
    DataException(DataException),
    IntegrityConstraintViolation(IntegrityConstraintViolation),
    InvalidCursorState,
    InvalidTransactionState(InvalidTransactionState),
    InvalidSqlStatementName,
    TriggeredDataChangeViolation(TriggeredDataChangeViolation),
    InvalidAuthorizationSpecification,
    DependentPrivilegeDescriptorsExist,
    InvalidCharsetName,
    InvalidTransactionTermination,
    InvalidConnectionName,
    SqlRoutineException(SqlRoutineException),
    InvalidCollationName,
    InvalidSqlStatementIdentifier,
    InvalidSqlDescriptorName,
    InvalidCursorName,
    InvalidConditionNumber,
    CursorSensitivityException(CursorSensitivityException),
    ExternalRoutineException(ExternalRoutineException),
    ExternalRoutineInvocationException(ExternalRoutineInvocationException),
    SavepointException(SavepointException),
    AmbiguousCursorName,
    InvalidCatalogName,
    InvalidSchemaName,
    TransactionRollback(TransactionRollback),
    SyntaxErrorOrAccessRuleViolation,
    WithCheckOptionViolation,
    UnhandledUserDefinedException,
    OlbSpecificError(OlbSpecificError),
    DatalinkException(DatalinkException),
    FdwSpecificCondition(FdwSpecificCondition),
    CliSpecificCondition(CliSpecificCondition),
    RemoteDatabaseAccess(RemoteDatabaseAccess),
    Other(String),
}

impl TryFrom<&str> for SqlState {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // SQL standard requires length to be 5 bytes
        if value.len() != 5 {
            return Err(ParseError::InvalidLength(value.len()));
        }

        let (class, subclass) = value.split_at(2);

        match class {
            "00" => Ok(Self::Success),
            "01" => Ok(Self::Warning(Warning::try_from(subclass)?)),
            "02" => Ok(Self::NoData(NoData::try_from(subclass)?)),
            "07" => Ok(Self::DynamicSqlError(DynamicSqlError::try_from(subclass)?)),
            "08" => Ok(Self::ConnectionException(ConnectionException::try_from(
                subclass,
            )?)),
            "09" => Ok(Self::TriggeredActionException),
            "0A" => Ok(Self::FeatureNotSupported(FeatureNotSupported::try_from(
                subclass,
            )?)),
            "0D" => Ok(Self::InvalidTargetTypeSpecification),
            "0E" => Ok(Self::InvalidSchemaNameListSpecification),
            "0F" => Ok(Self::LocatorException(LocatorException::try_from(
                subclass,
            )?)),
            "0K" => Ok(Self::ResignalWhenHandlerNotActive),
            "0L" => Ok(Self::InvalidGrantor),
            "0M" => Ok(Self::InvalidSqlInvokedProcedureReference),
            "0N" => Ok(Self::SqlXmlMappingError(SqlXmlMappingError::try_from(
                subclass,
            )?)),
            "0P" => Ok(Self::InvalidRoleSpecification),
            "0S" => Ok(Self::InvalidTransformGroupNameSpecification),
            "0T" => Ok(Self::TargetTableDisagreesWithCursorSpecification),
            "0U" => Ok(Self::AttemptToAssignToNonUpdatableColumn),
            "0V" => Ok(Self::AttemptToAssignToOrderingColumn),
            "0W" => Ok(Self::ProhibitedStatementDuringTriggerExecution(
                ProhibitedStatementDuringTriggerExecution::try_from(subclass)?,
            )),
            "0X" => Ok(Self::InvalidForeignServerSpecification),
            "0Y" => Ok(Self::PassthroughSpecificCondition(
                PassthroughSpecificCondition::try_from(subclass)?,
            )),
            "0Z" => Ok(Self::DiagnosticsException(DiagnosticsException::try_from(
                subclass,
            )?)),
            "10" => Ok(Self::XQueryError),
            "20" => Ok(Self::CaseNotFoundForCaseStatement),
            "21" => Ok(Self::CardinalityViolation),
            "22" => Ok(Self::DataException(DataException::try_from(subclass)?)),
            "23" => Ok(Self::IntegrityConstraintViolation(
                IntegrityConstraintViolation::try_from(subclass)?,
            )),
            "24" => Ok(Self::InvalidCursorState),
            "25" => Ok(Self::InvalidTransactionState(
                InvalidTransactionState::try_from(subclass)?,
            )),
            "26" => Ok(Self::InvalidSqlStatementName),
            "27" => Ok(Self::TriggeredDataChangeViolation(
                TriggeredDataChangeViolation::try_from(subclass)?,
            )),
            "28" => Ok(Self::InvalidAuthorizationSpecification),
            "2B" => Ok(Self::DependentPrivilegeDescriptorsExist),
            "2C" => Ok(Self::InvalidCharsetName),
            "2D" => Ok(Self::InvalidTransactionTermination),
            "2E" => Ok(Self::InvalidConnectionName),
            "2F" => Ok(Self::SqlRoutineException(SqlRoutineException::try_from(
                subclass,
            )?)),
            "2H" => Ok(Self::InvalidCollationName),
            "30" => Ok(Self::InvalidSqlStatementIdentifier),
            "33" => Ok(Self::InvalidSqlDescriptorName),
            "34" => Ok(Self::InvalidCursorName),
            "35" => Ok(Self::InvalidConditionNumber),
            "36" => Ok(Self::CursorSensitivityException(
                CursorSensitivityException::try_from(subclass)?,
            )),
            "38" => Ok(Self::ExternalRoutineException(
                ExternalRoutineException::try_from(subclass)?,
            )),
            "39" => Ok(Self::ExternalRoutineInvocationException(
                ExternalRoutineInvocationException::try_from(subclass)?,
            )),
            "3B" => Ok(Self::SavepointException(SavepointException::try_from(
                subclass,
            )?)),
            "3C" => Ok(Self::AmbiguousCursorName),
            "3D" => Ok(Self::InvalidCatalogName),
            "3F" => Ok(Self::InvalidSchemaName),
            "40" => Ok(Self::TransactionRollback(TransactionRollback::try_from(
                subclass,
            )?)),
            "42" => Ok(Self::SyntaxErrorOrAccessRuleViolation),
            "44" => Ok(Self::WithCheckOptionViolation),
            "45" => Ok(Self::UnhandledUserDefinedException),
            "46" => Ok(Self::OlbSpecificError(OlbSpecificError::try_from(
                subclass,
            )?)),
            "HW" => Ok(Self::DatalinkException(DatalinkException::try_from(
                subclass,
            )?)),
            "HV" => Ok(Self::FdwSpecificCondition(FdwSpecificCondition::try_from(
                subclass,
            )?)),
            "HY" => Ok(Self::CliSpecificCondition(CliSpecificCondition::try_from(
                subclass,
            )?)),
            "HZ" => Ok(Self::RemoteDatabaseAccess(RemoteDatabaseAccess::try_from(
                subclass,
            )?)),
            other => Ok(Self::Other(value.to_string())),
        }
    }
}
