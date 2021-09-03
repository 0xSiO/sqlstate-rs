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
}

pub enum NoData {
    NoAdditionalResultSetsReturned,
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
