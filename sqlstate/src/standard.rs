use std::str::FromStr;

pub mod class;

use crate::{error::ParseError, standard::class::*};

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

impl FromStr for SqlState {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // SQL standard requires length to be 5 bytes
        if value.len() != 5 {
            return Err(ParseError::InvalidLength(value.len()));
        }

        let (class, subclass) = value.split_at(2);

        match class {
            "00" => Ok(Self::Success),
            "01" => Ok(Self::Warning(subclass.parse().unwrap())),
            "02" => Ok(Self::NoData(subclass.parse().unwrap())),
            "07" => Ok(Self::DynamicSqlError(subclass.parse().unwrap())),
            "08" => Ok(Self::ConnectionException(subclass.parse().unwrap())),
            "09" => Ok(Self::TriggeredActionException),
            "0A" => Ok(Self::FeatureNotSupported(subclass.parse().unwrap())),
            "0D" => Ok(Self::InvalidTargetTypeSpecification),
            "0E" => Ok(Self::InvalidSchemaNameListSpecification),
            "0F" => Ok(Self::LocatorException(subclass.parse().unwrap())),
            "0K" => Ok(Self::ResignalWhenHandlerNotActive),
            "0L" => Ok(Self::InvalidGrantor),
            "0M" => Ok(Self::InvalidSqlInvokedProcedureReference),
            "0N" => Ok(Self::SqlXmlMappingError(subclass.parse().unwrap())),
            "0P" => Ok(Self::InvalidRoleSpecification),
            "0S" => Ok(Self::InvalidTransformGroupNameSpecification),
            "0T" => Ok(Self::TargetTableDisagreesWithCursorSpecification),
            "0U" => Ok(Self::AttemptToAssignToNonUpdatableColumn),
            "0V" => Ok(Self::AttemptToAssignToOrderingColumn),
            "0W" => Ok(Self::ProhibitedStatementDuringTriggerExecution(
                subclass.parse().unwrap(),
            )),
            "0X" => Ok(Self::InvalidForeignServerSpecification),
            "0Y" => Ok(Self::PassthroughSpecificCondition(
                subclass.parse().unwrap(),
            )),
            "0Z" => Ok(Self::DiagnosticsException(subclass.parse().unwrap())),
            "10" => Ok(Self::XQueryError),
            "20" => Ok(Self::CaseNotFoundForCaseStatement),
            "21" => Ok(Self::CardinalityViolation),
            "22" => Ok(Self::DataException(subclass.parse().unwrap())),
            "23" => Ok(Self::IntegrityConstraintViolation(
                subclass.parse().unwrap(),
            )),
            "24" => Ok(Self::InvalidCursorState),
            "25" => Ok(Self::InvalidTransactionState(subclass.parse().unwrap())),
            "26" => Ok(Self::InvalidSqlStatementName),
            "27" => Ok(Self::TriggeredDataChangeViolation(
                subclass.parse().unwrap(),
            )),
            "28" => Ok(Self::InvalidAuthorizationSpecification),
            "2B" => Ok(Self::DependentPrivilegeDescriptorsExist),
            "2C" => Ok(Self::InvalidCharsetName),
            "2D" => Ok(Self::InvalidTransactionTermination),
            "2E" => Ok(Self::InvalidConnectionName),
            "2F" => Ok(Self::SqlRoutineException(subclass.parse().unwrap())),
            "2H" => Ok(Self::InvalidCollationName),
            "30" => Ok(Self::InvalidSqlStatementIdentifier),
            "33" => Ok(Self::InvalidSqlDescriptorName),
            "34" => Ok(Self::InvalidCursorName),
            "35" => Ok(Self::InvalidConditionNumber),
            "36" => Ok(Self::CursorSensitivityException(subclass.parse().unwrap())),
            "38" => Ok(Self::ExternalRoutineException(subclass.parse().unwrap())),
            "39" => Ok(Self::ExternalRoutineInvocationException(
                subclass.parse().unwrap(),
            )),
            "3B" => Ok(Self::SavepointException(subclass.parse().unwrap())),
            "3C" => Ok(Self::AmbiguousCursorName),
            "3D" => Ok(Self::InvalidCatalogName),
            "3F" => Ok(Self::InvalidSchemaName),
            "40" => Ok(Self::TransactionRollback(subclass.parse().unwrap())),
            "42" => Ok(Self::SyntaxErrorOrAccessRuleViolation),
            "44" => Ok(Self::WithCheckOptionViolation),
            "45" => Ok(Self::UnhandledUserDefinedException),
            "46" => Ok(Self::OlbSpecificError(subclass.parse().unwrap())),
            "HW" => Ok(Self::DatalinkException(subclass.parse().unwrap())),
            "HV" => Ok(Self::FdwSpecificCondition(subclass.parse().unwrap())),
            "HY" => Ok(Self::CliSpecificCondition(subclass.parse().unwrap())),
            "HZ" => Ok(Self::RemoteDatabaseAccess(subclass.parse().unwrap())),
            _ => Ok(Self::Other(value.to_string())),
        }
    }
}
