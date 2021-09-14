use sqlstate_macros::subclass;

#[subclass]
pub enum Warning {
    #[state("001")]
    CursorOperationConflict,
    #[state("002")]
    DisconnectError,
    #[state("003")]
    NullValueEliminatedInSetFunction,
    #[state("004")]
    StringDataRightTruncation,
    #[state("005")]
    InsufficientItemDescriptorAreas,
    #[state("006")]
    PrivilegeNotRevoked,
    #[state("007")]
    PrivilegeNotGranted,
    #[state("009")]
    SearchConditionTooLongForInformationSchema,
    #[state("00A")]
    QueryExpressionTooLongForInformationSchema,
    #[state("00B")]
    DefaultValueTooLongForInformationSchema,
    #[state("00C")]
    ResultSetsReturned,
    #[state("00D")]
    AdditionalResultSetsReturned,
    #[state("00E")]
    AttemptToReturnTooManyResultSets,
    #[state("00F")]
    StatementTooLongForInformationSchema,
    #[state("010")]
    ColumnCannotBeMapped,
    #[state("011")]
    SqlJavaPathTooLongForInformationSchema,
    #[state("012")]
    InvalidNumberOfConditions,
    #[state("02F")]
    ArrayDataRightTruncation,
}

#[subclass]
pub enum NoData {
    #[state("001")]
    NoAdditionalResultSetsReturned,
}

#[subclass]
pub enum DynamicSqlError {
    #[state("001")]
    UsingClauseDoesNotMatchDynamicParameterSpecifications,
    #[state("002")]
    UsingClauseDoesNotMatchTargetSpecifications,
    #[state("003")]
    CursorSpecificationCannotBeExecuted,
    #[state("004")]
    UsingClauseRequiredForDynamicParameters,
    #[state("005")]
    PreparedStatementNotACursorSpecification,
    #[state("006")]
    RestrictedDataTypeAttributeViolation,
    #[state("007")]
    UsingClauseRequiredForResultFields,
    #[state("008")]
    InvalidDescriptorCount,
    #[state("009")]
    InvalidDescriptorIndex,
    #[state("00B")]
    DataTypeTransformFunctionViolation,
    #[state("00C")]
    UndefinedDataValue,
    #[state("00D")]
    InvalidDataTarget,
    #[state("00E")]
    InvalidLevelValue,
    #[state("00F")]
    InvalidDatetimeIntervalCode,
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
