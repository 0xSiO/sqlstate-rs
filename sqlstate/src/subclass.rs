use std::{convert::Infallible, str::FromStr};

pub enum Warning {
    CursorOperationConflict,
    DisconnectError,
    NullValueEliminatedInSetFunction,
    StringDataRightTruncation,
    InsufficientItemDescriptorAreas,
    PrivilegeNotRevoked,
    PrivilegeNotGranted,
    SearchConditionTooLongForInformationSchema,
    QueryExpressionTooLongForInformationSchema,
    DefaultValueTooLongForInformationSchema,
    ResultSetsReturned,
    AdditionalResultSetsReturned,
    AttemptToReturnTooManyResultSets,
    StatementTooLongForInformationSchema,
    ColumnCannotBeMapped,
    SqlJavaPathTooLongForInformationSchema,
    InvalidNumberOfConditions,
    ArrayDataRightTruncation,
    Other(String),
}

impl FromStr for Warning {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "001" => Ok(Self::CursorOperationConflict),
            "002" => Ok(Self::DisconnectError),
            "003" => Ok(Self::NullValueEliminatedInSetFunction),
            "004" => Ok(Self::StringDataRightTruncation),
            "005" => Ok(Self::InsufficientItemDescriptorAreas),
            "006" => Ok(Self::PrivilegeNotRevoked),
            "007" => Ok(Self::PrivilegeNotGranted),
            "009" => Ok(Self::SearchConditionTooLongForInformationSchema),
            "00A" => Ok(Self::QueryExpressionTooLongForInformationSchema),
            "00B" => Ok(Self::DefaultValueTooLongForInformationSchema),
            "00C" => Ok(Self::ResultSetsReturned),
            "00D" => Ok(Self::AdditionalResultSetsReturned),
            "00E" => Ok(Self::AttemptToReturnTooManyResultSets),
            "00F" => Ok(Self::StatementTooLongForInformationSchema),
            "010" => Ok(Self::ColumnCannotBeMapped),
            "011" => Ok(Self::SqlJavaPathTooLongForInformationSchema),
            "012" => Ok(Self::InvalidNumberOfConditions),
            "02F" => Ok(Self::ArrayDataRightTruncation),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

impl Warning {
    pub fn as_str(&self) -> &str {
        use Warning::*;

        match self {
            CursorOperationConflict => "001",
            DisconnectError => "002",
            NullValueEliminatedInSetFunction => "003",
            StringDataRightTruncation => "004",
            InsufficientItemDescriptorAreas => "005",
            PrivilegeNotRevoked => "006",
            PrivilegeNotGranted => "007",
            SearchConditionTooLongForInformationSchema => "009",
            QueryExpressionTooLongForInformationSchema => "00A",
            DefaultValueTooLongForInformationSchema => "00B",
            ResultSetsReturned => "00C",
            AdditionalResultSetsReturned => "00D",
            AttemptToReturnTooManyResultSets => "00E",
            StatementTooLongForInformationSchema => "00F",
            ColumnCannotBeMapped => "010",
            SqlJavaPathTooLongForInformationSchema => "011",
            InvalidNumberOfConditions => "012",
            ArrayDataRightTruncation => "02F",
            Other(subclass) => subclass.as_str(),
        }
    }
}

pub enum NoData {
    NoAdditionalResultSetsReturned,
    Other(String),
}

impl FromStr for NoData {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "001" => Ok(Self::NoAdditionalResultSetsReturned),
            other => Ok(Self::Other(other.to_string())),
        }
    }
}

impl NoData {
    pub fn as_str(&self) -> &str {
        use NoData::*;

        match self {
            NoAdditionalResultSetsReturned => "001",
            Other(subclass) => subclass.as_str(),
        }
    }
}

pub enum DynamicSqlError {
    UsingClauseDoesNotMatchDynamicParameterSpecifications,
    UsingClauseDoesNotMatchTargetSpecifications,
    CursorSpecificationCannotBeExecuted,
    UsingClauseRequiredForDynamicParameters,
    PreparedStatementNotACursorSpecification,
    RestrictedDataTypeAttributeViolation,
    UsingClauseRequiredForResultFields,
    InvalidDescriptorCount,
    InvalidDescriptorIndex,
    DataTypeTransformFunctionViolation,
    UndefinedDataValue,
    InvalidDataTarget,
    InvalidLevelValue,
    InvalidDatetimeIntervalCode,
    Other(String),
}

impl FromStr for DynamicSqlError {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use DynamicSqlError::*;

        match value {
            "001" => Ok(UsingClauseDoesNotMatchDynamicParameterSpecifications),
            "002" => Ok(UsingClauseDoesNotMatchTargetSpecifications),
            "003" => Ok(CursorSpecificationCannotBeExecuted),
            "004" => Ok(UsingClauseRequiredForDynamicParameters),
            "005" => Ok(PreparedStatementNotACursorSpecification),
            "006" => Ok(RestrictedDataTypeAttributeViolation),
            "007" => Ok(UsingClauseRequiredForResultFields),
            "008" => Ok(InvalidDescriptorCount),
            "009" => Ok(InvalidDescriptorIndex),
            "00B" => Ok(DataTypeTransformFunctionViolation),
            "00C" => Ok(UndefinedDataValue),
            "00D" => Ok(InvalidDataTarget),
            "00E" => Ok(InvalidLevelValue),
            "00F" => Ok(InvalidDatetimeIntervalCode),
            other => Ok(Other(other.to_string())),
        }
    }
}

impl DynamicSqlError {
    pub fn as_str(&self) -> &str {
        use DynamicSqlError::*;

        match self {
            Other(subclass) => subclass.as_str(),
            _ => todo!(),
        }
    }
}

pub enum ConnectionException {
    SqlClientUnableToEstablishSqlConnection,
    ConnectionNameInUse,
    ConnectionDoesNotExist,
    SqlServerRejectedEstablishmentOfSqlConnection,
    ConnectionFailure,
    TransactionResolutionUnknown,
}

pub enum FeatureNotSupported {
    MultipleServerTransactions,
}

pub enum LocatorException {
    InvalidSpecification,
}

pub enum SqlXmlMappingError {
    UnmappableXmlName,
    InvalidXmlCharacter,
}

pub enum ProhibitedStatementDuringTriggerExecution {
    ModifyTableModifiedByDataChangeDeltaTable,
}

pub enum PassthroughSpecificCondition {
    InvalidCursorOption,
    InvalidCursorAllocation,
}

pub enum DiagnosticsException {
    MaximumNumberOfStackedDiagnosticsAreasExceeded,
    StackedDiagnosticsAccessedWithoutActiveHandler,
}

pub enum DataException {
    StringDataRightTruncation,
    NullValueNoIndicatorParameter,
    NumericValueOutOfRange,
    NullValueNotAllowed,
    ErrorInAssignment,
    InvalidIntervalFormat,
    InvalidDatetimeFormat,
    DatetimeFieldOverflow,
    InvalidTimeZoneDisplacementValue,
    EscapeCharacterConflict,
    InvalidUseOfEscapeCharacter,
    InvalidEscapeOctet,
    NullValueInArrayTarget,
    ZeroLengthCharacterString,
    MostSpecificTypeMismatch,
    SequenceGeneratorLimitExceeded,
    NonidenticalNotationsWithTheSameName,
    NonidenticalUnparsedEntitiesWithTheSameName,
    NotAnXmlDocument,
    InvalidXmlDocument,
    InvalidXmlContent,
    IntervalValueOutOfRange,
    MultisetValueOverflow,
    XmlValueOverflow,
    InvalidComment,
    InvalidProcessingInstruction,
    NotAnXQueryDocumentNode,
    InvalidXQueryContextItem,
    XQuerySerializationError,
    InvalidIndicatorParameterValue,
    SubstringError,
    DivisionByZero,
    InvalidPrecedingOrFollowingSizeInWindowFunction,
    InvalidArgumentForNtileFunction,
    IntervalFieldOverflow,
    InvalidArgumentForNthValueFunction,
    InvalidDataSpecifiedForDatalink,
    InvalidCharacterValueForCast,
    InvalidEscapeCharacter,
    NullArgumentPassedToDatalinkConstructor,
    InvalidRegularExpression,
    NullRowNotPermittedInTable,
    DatalinkValueExceedsMaximumLength,
    InvalidArgumentForNaturalLogarithm,
    InvalidArgumentForPowerFunction,
    InvalidArgumentForWidthBucketFunction,
    InvalidRowVersion,
    XQuerySequenceCannotBeValidated,
    XQueryDocumentNodeCannotBeValidated,
    NoXmlSchemaFound,
    ElementNamespaceNotDeclared,
    GlobalElementNotDeclared,
    NoXmlElementWithSpecifiedQName,
    NoXmlElementWithSpecifiedNamespace,
    ValidationFailure,
    InvalidQueryRegularExpression,
    InvalidQueryOptionFlag,
    AttemptToReplaceAZeroLengthString,
    InvalidQueryReplacementString,
    InvalidRowCountInFetchFirstClause,
    InvalidRowCountInResultOffsetClause,
    CharacterNotInRepertoire,
    IndicatorOverflow,
    InvalidParameterValue,
    UnterminatedCString,
    InvalidEscapeSequence,
    StringDataLengthMismatch,
    TrimError,
    NoncharacterInUcsString,
    NullValueInFieldReference,
    NullValueSubstitutedForMutatorSubjectParameter,
    ArrayElementError,
    ArrayDataRightTruncation,
    InvalidRepeatArgumentInASampleClause,
    InvalidSampleSize,
}

pub enum IntegrityConstraintViolation {
    RestrictViolation,
}

pub enum InvalidTransactionState {
    ActiveSqlTransaction,
    BranchTransactionAlreadyActive,
    InappropriateAccessModeForBranchTransaction,
    InappropriateIsolationLevelForBranchTransaction,
    NoActiveSqlTransactionForBranchTransaction,
    ReadOnlySqlTransaction,
    SchemaAndDataStatementMixingNotSupported,
    HeldCursorRequiresSameIsolationLevel,
}

pub enum TriggeredDataChangeViolation {
    ModifyTableModifiedByDataChangeDeltaTable,
}

pub enum SqlRoutineException {
    ModifyingSqlDataNotPermitted,
    ProhibitedSqlStatementAttempted,
    ReadingSqlDataNotPermitted,
    FunctionExecutedNoReturnStatement,
}

pub enum CursorSensitivityException {
    RequestRejected,
    RequestFailed,
}

pub enum ExternalRoutineException {
    ContainingSqlNotPermitted,
    ModifyingSqlDataNotPermitted,
    ProhibitedSqlStatementAttempted,
    ReadingSqlDataNotPermitted,
}

pub enum ExternalRoutineInvocationException {
    NullValueNotAllowed,
}

pub enum SavepointException {
    InvalidSavepointSpecification,
    TooManySavepoints,
}

pub enum TransactionRollback {
    SerializationFailure,
    IntegrityConstraintViolation,
    StatementCompletionUnknown,
    TriggeredActionException,
}

pub enum OlbSpecificError {
    InvalidUrl,
    InvalidJarName,
    InvalidClassDeletion,
    InvalidReplacement,
    AttemptToReplaceUninstalledJar,
    AttemptToRemoveUninstalledJar,
    InvalidJarRemoval,
    InvalidPath,
    SelfReferencingPath,
    InvalidJarNameInPath,
    UnresolvedClassName,
    UnsupportedFeature,
    InvalidClassDeclaration,
    InvalidColumnName,
    InvalidNumberOfColumns,
    InvalidProfileState,
}

pub enum DatalinkException {
    ExternalFileNotLinked,
    ExternalFileAlreadyLinked,
    ReferencedFileDoesNotExist,
    InvalidWriteToken,
    InvalidDatalinkConstruction,
    InvalidWritePermissionForUpdate,
    ReferencedFileNotValid,
}

pub enum FdwSpecificCondition {
    MemoryAllocationError,
    DynamicParameterValueNeeded,
    InvalidDataType,
    ColumnNameNotFound,
    InvalidDataTypeDescriptors,
    InvalidColumnName,
    InvalidColumnNumber,
    InvalidUseOfNullPointer,
    InvalidStringFormat,
    InvalidHandle,
    InvalidOptionIndex,
    InvalidOptionName,
    OptionNameNotFound,
    ReplyHandle,
    UnableToCreateExecution,
    UnableToCreateReply,
    UnableToEstablishConnection,
    NoSchemas,
    SchemaNotFound,
    TableNotFound,
    FunctionSequenceError,
    LimitOnNumberOfHandlesExceeded,
    InconsistentDescriptorInformation,
    InvalidAttributeValue,
    InvalidStringLengthOrBufferLength,
    InvalidDescriptorFieldIdentifier,
}

pub enum CliSpecificCondition {
    DynamicParameterValueNeeded,
    InvalidHandle,
    MemoryAllocationError,
    InvalidDataTypeInApplicationDescriptor,
    InvalidDataType,
    AssociatedStatementIsNotPrepared,
    OperationCanceled,
    InvalidUseOfNullPointer,
    FunctionSequenceError,
    AttributeCannotBeSetNow,
    InvalidTransactionOperationCode,
    MemoryManagementError,
    LimitOnNumberOfHandlesExceeded,
    InvalidUseOfAutomaticallyAllocatedDescriptorHandle,
    ServerDeclinedCancellationRequest,
    NonStringDataCannotBeSentInPieces,
    AttemptToConcatenateANullValue,
    InconsistentDescriptorInformation,
    InvalidAttributeValue,
    NonStringDataCannotBeUsedWithStringRoutine,
    InvalidStringLengthOrBufferLength,
    InvalidDescriptorFieldIdentifier,
    InvalidAttributeIdentifier,
    InvalidDatalinkValue,
    InvalidFunctionIdSpecified,
    InvalidInformationType,
    ColumnTypeOutOfRange,
    ScopeOutOfRange,
    NullableTypeOutOfRange,
    InvalidRetrievalCode,
    InvalidLengthPrecisionValue,
    InvalidParamterMode,
    InvalidFetchOrientation,
    RowValueOutOfRange,
    InvalidCursorPosition,
    OptionalFeatureNotImplemented,
}

pub enum RemoteDatabaseAccess {
    Unknown,
}
