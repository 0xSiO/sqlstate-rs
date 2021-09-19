//! Classes and subclasses for standard return codes.

use sqlstate_macros::class;

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum Success {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum Warning {
    #[subclass("001")]
    CursorOperationConflict,
    #[subclass("002")]
    DisconnectError,
    #[subclass("003")]
    NullValueEliminatedInSetFunction,
    #[subclass("004")]
    StringDataRightTruncation,
    #[subclass("005")]
    InsufficientItemDescriptorAreas,
    #[subclass("006")]
    PrivilegeNotRevoked,
    #[subclass("007")]
    PrivilegeNotGranted,
    #[subclass("009")]
    SearchConditionTooLongForInformationSchema,
    #[subclass("00A")]
    QueryExpressionTooLongForInformationSchema,
    #[subclass("00B")]
    DefaultValueTooLongForInformationSchema,
    #[subclass("00C")]
    ResultSetsReturned,
    #[subclass("00D")]
    AdditionalResultSetsReturned,
    #[subclass("00E")]
    AttemptToReturnTooManyResultSets,
    #[subclass("00F")]
    StatementTooLongForInformationSchema,
    #[subclass("010")]
    ColumnCannotBeMapped,
    #[subclass("011")]
    SqlJavaPathTooLongForInformationSchema,
    #[subclass("012")]
    InvalidNumberOfConditions,
    #[subclass("02F")]
    ArrayDataRightTruncation,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum NoData {
    #[subclass("001")]
    NoAdditionalResultSetsReturned,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum DynamicSqlError {
    #[subclass("001")]
    UsingClauseDoesNotMatchDynamicParameterSpecifications,
    #[subclass("002")]
    UsingClauseDoesNotMatchTargetSpecifications,
    #[subclass("003")]
    CursorSpecificationCannotBeExecuted,
    #[subclass("004")]
    UsingClauseRequiredForDynamicParameters,
    #[subclass("005")]
    PreparedStatementNotACursorSpecification,
    #[subclass("006")]
    RestrictedDataTypeAttributeViolation,
    #[subclass("007")]
    UsingClauseRequiredForResultFields,
    #[subclass("008")]
    InvalidDescriptorCount,
    #[subclass("009")]
    InvalidDescriptorIndex,
    #[subclass("00B")]
    DataTypeTransformFunctionViolation,
    #[subclass("00C")]
    UndefinedDataValue,
    #[subclass("00D")]
    InvalidDataTarget,
    #[subclass("00E")]
    InvalidLevelValue,
    #[subclass("00F")]
    InvalidDatetimeIntervalCode,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ConnectionException {
    #[subclass("001")]
    SqlClientUnableToEstablishSqlConnection,
    #[subclass("002")]
    ConnectionNameInUse,
    #[subclass("003")]
    ConnectionDoesNotExist,
    #[subclass("004")]
    SqlServerRejectedEstablishmentOfSqlConnection,
    #[subclass("006")]
    ConnectionFailure,
    #[subclass("007")]
    TransactionResolutionUnknown,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum TriggeredActionException {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum FeatureNotSupported {
    #[subclass("001")]
    MultipleServerTransactions,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidTargetTypeSpecification {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidSchemaNameListSpecification {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LocatorException {
    #[subclass("001")]
    InvalidSpecification,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ResignalWhenHandlerNotActive {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidGrantor {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidSqlInvokedProcedureReference {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SqlXmlMappingError {
    #[subclass("001")]
    UnmappableXmlName,
    #[subclass("002")]
    InvalidXmlCharacter,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidRoleSpecification {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidTransformGroupNameSpecification {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum TargetTableDisagreesWithCursorSpecification {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum AttemptToAssignToNonUpdatableColumn {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum AttemptToAssignToOrderingColumn {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ProhibitedStatementDuringTriggerExecution {
    #[subclass("001")]
    ModifyTableModifiedByDataChangeDeltaTable,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidForeignServerSpecification {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum PassthroughSpecificCondition {
    #[subclass("001")]
    InvalidCursorOption,
    #[subclass("002")]
    InvalidCursorAllocation,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum DiagnosticsException {
    #[subclass("001")]
    MaximumNumberOfStackedDiagnosticsAreasExceeded,
    #[subclass("002")]
    StackedDiagnosticsAccessedWithoutActiveHandler,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum XQueryError {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum CaseNotFoundForCaseStatement {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum CardinalityViolation {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum DataException {
    #[subclass("001")]
    StringDataRightTruncation,
    #[subclass("002")]
    NullValueNoIndicatorParameter,
    #[subclass("003")]
    NumericValueOutOfRange,
    #[subclass("004")]
    NullValueNotAllowed,
    #[subclass("005")]
    ErrorInAssignment,
    #[subclass("006")]
    InvalidIntervalFormat,
    #[subclass("007")]
    InvalidDatetimeFormat,
    #[subclass("008")]
    DatetimeFieldOverflow,
    #[subclass("009")]
    InvalidTimeZoneDisplacementValue,
    #[subclass("00B")]
    EscapeCharacterConflict,
    #[subclass("00C")]
    InvalidUseOfEscapeCharacter,
    #[subclass("00D")]
    InvalidEscapeOctet,
    #[subclass("00E")]
    NullValueInArrayTarget,
    #[subclass("00F")]
    ZeroLengthCharacterString,
    #[subclass("00G")]
    MostSpecificTypeMismatch,
    #[subclass("00H")]
    SequenceGeneratorLimitExceeded,
    #[subclass("00J")]
    NonidenticalNotationsWithTheSameName,
    #[subclass("00K")]
    NonidenticalUnparsedEntitiesWithTheSameName,
    #[subclass("00L")]
    NotAnXmlDocument,
    #[subclass("00M")]
    InvalidXmlDocument,
    #[subclass("00N")]
    InvalidXmlContent,
    #[subclass("00P")]
    IntervalValueOutOfRange,
    #[subclass("00Q")]
    MultisetValueOverflow,
    #[subclass("00R")]
    XmlValueOverflow,
    #[subclass("00S")]
    InvalidComment,
    #[subclass("00T")]
    InvalidProcessingInstruction,
    #[subclass("00U")]
    NotAnXQueryDocumentNode,
    #[subclass("00V")]
    InvalidXQueryContextItem,
    #[subclass("00W")]
    XQuerySerializationError,
    #[subclass("010")]
    InvalidIndicatorParameterValue,
    #[subclass("011")]
    SubstringError,
    #[subclass("012")]
    DivisionByZero,
    #[subclass("013")]
    InvalidPrecedingOrFollowingSizeInWindowFunction,
    #[subclass("014")]
    InvalidArgumentForNtileFunction,
    #[subclass("015")]
    IntervalFieldOverflow,
    #[subclass("016")]
    InvalidArgumentForNthValueFunction,
    #[subclass("017")]
    InvalidDataSpecifiedForDatalink,
    #[subclass("018")]
    InvalidCharacterValueForCast,
    #[subclass("019")]
    InvalidEscapeCharacter,
    #[subclass("01A")]
    NullArgumentPassedToDatalinkConstructor,
    #[subclass("01B")]
    InvalidRegularExpression,
    #[subclass("01C")]
    NullRowNotPermittedInTable,
    #[subclass("01D")]
    DatalinkValueExceedsMaximumLength,
    #[subclass("01E")]
    InvalidArgumentForNaturalLogarithm,
    #[subclass("01F")]
    InvalidArgumentForPowerFunction,
    #[subclass("01G")]
    InvalidArgumentForWidthBucketFunction,
    #[subclass("01H")]
    InvalidRowVersion,
    #[subclass("01J")]
    XQuerySequenceCannotBeValidated,
    #[subclass("01K")]
    XQueryDocumentNodeCannotBeValidated,
    #[subclass("01L")]
    NoXmlSchemaFound,
    #[subclass("01M")]
    ElementNamespaceNotDeclared,
    #[subclass("01N")]
    GlobalElementNotDeclared,
    #[subclass("01P")]
    NoXmlElementWithSpecifiedQName,
    #[subclass("01Q")]
    NoXmlElementWithSpecifiedNamespace,
    #[subclass("01R")]
    ValidationFailure,
    #[subclass("01S")]
    InvalidQueryRegularExpression,
    #[subclass("01T")]
    InvalidQueryOptionFlag,
    #[subclass("01U")]
    AttemptToReplaceAZeroLengthString,
    #[subclass("01V")]
    InvalidQueryReplacementString,
    #[subclass("01W")]
    InvalidRowCountInFetchFirstClause,
    #[subclass("01X")]
    InvalidRowCountInResultOffsetClause,
    #[subclass("021")]
    CharacterNotInRepertoire,
    #[subclass("022")]
    IndicatorOverflow,
    #[subclass("023")]
    InvalidParameterValue,
    #[subclass("024")]
    UnterminatedCString,
    #[subclass("025")]
    InvalidEscapeSequence,
    #[subclass("026")]
    StringDataLengthMismatch,
    #[subclass("027")]
    TrimError,
    #[subclass("029")]
    NoncharacterInUcsString,
    #[subclass("02A")]
    NullValueInFieldReference,
    #[subclass("02D")]
    NullValueSubstitutedForMutatorSubjectParameter,
    #[subclass("02E")]
    ArrayElementError,
    #[subclass("02F")]
    ArrayDataRightTruncation,
    #[subclass("02G")]
    InvalidRepeatArgumentInASampleClause,
    #[subclass("02H")]
    InvalidSampleSize,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum IntegrityConstraintViolation {
    #[subclass("001")]
    RestrictViolation,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidCursorState {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidTransactionState {
    #[subclass("001")]
    ActiveSqlTransaction,
    #[subclass("002")]
    BranchTransactionAlreadyActive,
    #[subclass("003")]
    InappropriateAccessModeForBranchTransaction,
    #[subclass("004")]
    InappropriateIsolationLevelForBranchTransaction,
    #[subclass("005")]
    NoActiveSqlTransactionForBranchTransaction,
    #[subclass("006")]
    ReadOnlySqlTransaction,
    #[subclass("007")]
    SchemaAndDataStatementMixingNotSupported,
    #[subclass("008")]
    HeldCursorRequiresSameIsolationLevel,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidSqlStatementName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum TriggeredDataChangeViolation {
    #[subclass("001")]
    ModifyTableModifiedByDataChangeDeltaTable,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidAuthorizationSpecification {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum DependentPrivilegeDescriptorsExist {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidCharsetName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidTransactionTermination {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidConnectionName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SqlRoutineException {
    #[subclass("002")]
    ModifyingSqlDataNotPermitted,
    #[subclass("003")]
    ProhibitedSqlStatementAttempted,
    #[subclass("004")]
    ReadingSqlDataNotPermitted,
    #[subclass("005")]
    FunctionExecutedNoReturnStatement,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidCollationName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidSqlStatementIdentifier {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidSqlDescriptorName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidCursorName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidConditionNumber {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum CursorSensitivityException {
    #[subclass("001")]
    RequestRejected,
    #[subclass("002")]
    RequestFailed,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ExternalRoutineException {
    #[subclass("001")]
    ContainingSqlNotPermitted,
    #[subclass("002")]
    ModifyingSqlDataNotPermitted,
    #[subclass("003")]
    ProhibitedSqlStatementAttempted,
    #[subclass("004")]
    ReadingSqlDataNotPermitted,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ExternalRoutineInvocationException {
    #[subclass("004")]
    NullValueNotAllowed,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SavepointException {
    #[subclass("001")]
    InvalidSavepointSpecification,
    #[subclass("002")]
    TooManySavepoints,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum AmbiguousCursorName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidCatalogName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidSchemaName {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum TransactionRollback {
    #[subclass("001")]
    SerializationFailure,
    #[subclass("002")]
    IntegrityConstraintViolation,
    #[subclass("003")]
    StatementCompletionUnknown,
    #[subclass("004")]
    TriggeredActionException,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SyntaxErrorOrAccessRuleViolation {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum WithCheckOptionViolation {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum UnhandledUserDefinedException {}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum OlbSpecificError {
    #[subclass("001")]
    InvalidUrl,
    #[subclass("002")]
    InvalidJarName,
    #[subclass("003")]
    InvalidClassDeletion,
    #[subclass("005")]
    InvalidReplacement,
    #[subclass("00A")]
    AttemptToReplaceUninstalledJar,
    #[subclass("00B")]
    AttemptToRemoveUninstalledJar,
    #[subclass("00C")]
    InvalidJarRemoval,
    #[subclass("00D")]
    InvalidPath,
    #[subclass("00E")]
    SelfReferencingPath,
    #[subclass("102")]
    InvalidJarNameInPath,
    #[subclass("103")]
    UnresolvedClassName,
    #[subclass("110")]
    UnsupportedFeature,
    #[subclass("120")]
    InvalidClassDeclaration,
    #[subclass("121")]
    InvalidColumnName,
    #[subclass("122")]
    InvalidNumberOfColumns,
    #[subclass("130")]
    InvalidProfileState,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum DatalinkException {
    #[subclass("001")]
    ExternalFileNotLinked,
    #[subclass("002")]
    ExternalFileAlreadyLinked,
    #[subclass("003")]
    ReferencedFileDoesNotExist,
    #[subclass("004")]
    InvalidWriteToken,
    #[subclass("005")]
    InvalidDatalinkConstruction,
    #[subclass("006")]
    InvalidWritePermissionForUpdate,
    #[subclass("007")]
    ReferencedFileNotValid,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum FdwSpecificCondition {
    #[subclass("001")]
    MemoryAllocationError,
    #[subclass("002")]
    DynamicParameterValueNeeded,
    #[subclass("004")]
    InvalidDataType,
    #[subclass("005")]
    ColumnNameNotFound,
    #[subclass("006")]
    InvalidDataTypeDescriptors,
    #[subclass("007")]
    InvalidColumnName,
    #[subclass("008")]
    InvalidColumnNumber,
    #[subclass("009")]
    InvalidUseOfNullPointer,
    #[subclass("00A")]
    InvalidStringFormat,
    #[subclass("00B")]
    InvalidHandle,
    #[subclass("00C")]
    InvalidOptionIndex,
    #[subclass("00D")]
    InvalidOptionName,
    #[subclass("00J")]
    OptionNameNotFound,
    #[subclass("00K")]
    ReplyHandle,
    #[subclass("00L")]
    UnableToCreateExecution,
    #[subclass("00M")]
    UnableToCreateReply,
    #[subclass("00N")]
    UnableToEstablishConnection,
    #[subclass("00P")]
    NoSchemas,
    #[subclass("00Q")]
    SchemaNotFound,
    #[subclass("00R")]
    TableNotFound,
    #[subclass("010")]
    FunctionSequenceError,
    #[subclass("014")]
    LimitOnNumberOfHandlesExceeded,
    #[subclass("021")]
    InconsistentDescriptorInformation,
    #[subclass("024")]
    InvalidAttributeValue,
    #[subclass("090")]
    InvalidStringLengthOrBufferLength,
    #[subclass("091")]
    InvalidDescriptorFieldIdentifier,
}

#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum CliSpecificCondition {
    // No subclass value defined for these two variants.
    // DynamicParameterValueNeeded,
    // InvalidHandle,
    #[subclass("001")]
    MemoryAllocationError,
    #[subclass("003")]
    InvalidDataTypeInApplicationDescriptor,
    #[subclass("004")]
    InvalidDataType,
    #[subclass("007")]
    AssociatedStatementIsNotPrepared,
    #[subclass("008")]
    OperationCanceled,
    #[subclass("009")]
    InvalidUseOfNullPointer,
    #[subclass("010")]
    FunctionSequenceError,
    #[subclass("011")]
    AttributeCannotBeSetNow,
    #[subclass("012")]
    InvalidTransactionOperationCode,
    #[subclass("013")]
    MemoryManagementError,
    #[subclass("014")]
    LimitOnNumberOfHandlesExceeded,
    #[subclass("017")]
    InvalidUseOfAutomaticallyAllocatedDescriptorHandle,
    #[subclass("018")]
    ServerDeclinedCancellationRequest,
    #[subclass("019")]
    NonStringDataCannotBeSentInPieces,
    #[subclass("020")]
    AttemptToConcatenateANullValue,
    #[subclass("021")]
    InconsistentDescriptorInformation,
    #[subclass("024")]
    InvalidAttributeValue,
    #[subclass("055")]
    NonStringDataCannotBeUsedWithStringRoutine,
    #[subclass("090")]
    InvalidStringLengthOrBufferLength,
    #[subclass("091")]
    InvalidDescriptorFieldIdentifier,
    #[subclass("092")]
    InvalidAttributeIdentifier,
    #[subclass("093")]
    InvalidDatalinkValue,
    #[subclass("095")]
    InvalidFunctionIdSpecified,
    #[subclass("096")]
    InvalidInformationType,
    #[subclass("097")]
    ColumnTypeOutOfRange,
    #[subclass("098")]
    ScopeOutOfRange,
    #[subclass("099")]
    NullableTypeOutOfRange,
    #[subclass("103")]
    InvalidRetrievalCode,
    #[subclass("104")]
    InvalidLengthPrecisionValue,
    #[subclass("105")]
    InvalidParamterMode,
    #[subclass("106")]
    InvalidFetchOrientation,
    #[subclass("107")]
    RowValueOutOfRange,
    #[subclass("108")]
    InvalidCursorPosition,
    #[subclass("C00")]
    OptionalFeatureNotImplemented,
}

// TODO: RDA subconditions in ISO9579
#[class(standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum RemoteDatabaseAccess {}
