use sqlstate_macros::class;

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum Warning {
    #[subclass("008")]
    ImplicitZeroBitPadding,
    #[subclass("P01")]
    DeprecatedFeature,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SqlStatementNotYetComplete {}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ConnectionException {
    #[subclass("P01")]
    ProtocolViolation,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidTransactionInitiation {}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidGrantor {
    #[subclass("P01")]
    InvalidGrantOperation,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum DataException {
    #[subclass("030")]
    DuplicateJsonObjectKeyValue,
    #[subclass("031")]
    InvalidArgumentForSqlJsonDatetimeFunction,
    #[subclass("032")]
    InvalidJsonText,
    #[subclass("033")]
    InvalidSqlJsonSubscript,
    #[subclass("034")]
    MoreThanOneSqlJsonItem,
    #[subclass("035")]
    NoSqlJsonItem,
    #[subclass("036")]
    NonNumericSqlJsonItem,
    #[subclass("037")]
    NonUniqueKeysInJsonObject,
    #[subclass("038")]
    SingletonSqlJsonItemRequired,
    #[subclass("039")]
    SqlJsonArrayNotFound,
    #[subclass("03A")]
    SqlJsonMemberNotFound,
    #[subclass("03B")]
    SqlJsonNumberNotFound,
    #[subclass("03C")]
    SqlJsonObjectNotFound,
    #[subclass("03D")]
    TooManyJsonArrayElements,
    #[subclass("03E")]
    TooManyJsonObjectMembers,
    #[subclass("03F")]
    SqlJsonScalarRequired,
    #[subclass("P01")]
    FloatingPointException,
    #[subclass("P02")]
    InvalidTextRepresentation,
    #[subclass("P03")]
    InvalidBinaryRepresentation,
    #[subclass("P04")]
    BadCopyFileFormat,
    #[subclass("P05")]
    UntranslatableCharacter,
    #[subclass("P06")]
    NonstandardUseOfEscapeCharacter,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum IntegrityConstraintViolation {
    #[subclass("502")]
    NotNullViolation,
    #[subclass("503")]
    ForeignKeyViolation,
    #[subclass("505")]
    UniqueViolation,
    #[subclass("514")]
    CheckViolation,
    #[subclass("P01")]
    ExclusionViolation,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidTransactionState {
    #[subclass("P01")]
    NoActiveSqlTransaction,
    #[subclass("P02")]
    InFailedSqlTransaction,
    #[subclass("P03")]
    IdleInTransactionSessionTimeout,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InvalidAuthorizationSpecification {
    #[subclass("P01")]
    InvalidPassword,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum DependentPrivilegeDescriptorsExist {
    #[subclass("P01")]
    DependentObjectsStillExist,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ExternalRoutineInvocationException {
    #[subclass("001")]
    InvalidSqlStateReturned,
    #[subclass("P01")]
    TriggerProtocolViolated,
    #[subclass("P02")]
    SrfProtocolViolated,
    #[subclass("P03")]
    EventTriggerProtocolViolated,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum TransactionRollback {
    #[subclass("P01")]
    DeadlockDetected,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SyntaxErrorOrAccessRuleViolation {
    #[subclass("501")]
    InsufficientPrivilege,
    #[subclass("601")]
    SyntaxError,
    #[subclass("602")]
    InvalidName,
    #[subclass("611")]
    InvalidColumnDefinition,
    #[subclass("622")]
    NameTooLong,
    #[subclass("701")]
    DuplicateColumn,
    #[subclass("702")]
    AmbiguousColumn,
    #[subclass("703")]
    UndefinedColumn,
    #[subclass("704")]
    UndefinedObject,
    #[subclass("710")]
    DuplicateObject,
    #[subclass("712")]
    DuplicateAlias,
    #[subclass("723")]
    DuplicateFunction,
    #[subclass("725")]
    AmbiguousFunction,
    #[subclass("803")]
    GroupingError,
    #[subclass("804")]
    DatatypeMismatch,
    #[subclass("809")]
    WrongObjectType,
    #[subclass("830")]
    InvalidForeignKey,
    #[subclass("846")]
    CannotCoerce,
    #[subclass("883")]
    UndefinedFunction,
    #[subclass("8C9")]
    GeneratedAlways,
    #[subclass("939")]
    ReservedName,
    #[subclass("P01")]
    UndefinedTable,
    #[subclass("P02")]
    UndefinedParameter,
    #[subclass("P03")]
    DuplicateCursor,
    #[subclass("P04")]
    DuplicateDatabase,
    #[subclass("P05")]
    DuplicatePreparedStatement,
    #[subclass("P06")]
    DuplicateSchema,
    #[subclass("P07")]
    DuplicateTable,
    #[subclass("P08")]
    AmbiguousParameter,
    #[subclass("P09")]
    AmbiguousAlias,
    #[subclass("P10")]
    InvalidColumnReference,
    #[subclass("P11")]
    InvalidCursorDefinition,
    #[subclass("P12")]
    InvalidDatabaseDefinition,
    #[subclass("P13")]
    InvalidFunctionDefinition,
    #[subclass("P14")]
    InvalidPreparedStatementDefinition,
    #[subclass("P15")]
    InvalidSchemaDefinition,
    #[subclass("P16")]
    InvalidTableDefinition,
    #[subclass("P17")]
    InvalidObjectDefinition,
    #[subclass("P18")]
    IndeterminateDatatype,
    #[subclass("P19")]
    InvalidRecursion,
    #[subclass("P20")]
    WindowingError,
    #[subclass("P21")]
    CollationMismatch,
    #[subclass("P22")]
    IndeterminateCollation,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InsufficientResources {
    #[subclass("100")]
    DiskFull,
    #[subclass("200")]
    OutOfMemory,
    #[subclass("300")]
    TooManyConnections,
    #[subclass("400")]
    ConfigurationLimitExceeded,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ProgramLimitExceeded {
    #[subclass("001")]
    StatementTooComplex,
    #[subclass("011")]
    TooManyColumns,
    #[subclass("023")]
    TooManyArguments,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ObjectNotInPrerequisiteState {
    #[subclass("006")]
    ObjectInUse,
    #[subclass("P02")]
    CannotChangeRuntimeParam,
    #[subclass("P03")]
    LockNotAvailable,
    #[subclass("P04")]
    UnsafeNewEnumValueUsage,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum OperatorIntervention {
    #[subclass("014")]
    QueryCanceled,
    #[subclass("P01")]
    AdminShutdown,
    #[subclass("P02")]
    CrashShutdown,
    #[subclass("P03")]
    CannotConnectNow,
    #[subclass("P04")]
    DatabaseDropped,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SystemError {
    #[subclass("030")]
    IoError,
    #[subclass("P01")]
    UndefinedFile,
    #[subclass("P02")]
    DuplicateFile,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum SnapshotFailure {}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ConfigurationFileError {
    #[subclass("001")]
    LockFileExists,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum PlPgSqlError {
    #[subclass("001")]
    RaiseException,
    #[subclass("002")]
    NoDataFound,
    #[subclass("003")]
    TooManyRows,
    #[subclass("004")]
    AssertFailure,
}

#[class(non_standard)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum InternalError {
    #[subclass("001")]
    DataCorrupted,
    #[subclass("002")]
    IndexCorrupted,
}
