Representations and parsing logic for [`SQLSTATE`](https://en.wikipedia.org/wiki/SQLSTATE)
return codes.

## Examples

Parsing return codes according to the SQL standard:
```rust
use sqlstate::standard::{
    class::{DataException::DivisionByZero, Warning::PrivilegeNotGranted},
    SqlState,
};

assert_eq!("00000".parse::<SqlState>()?, SqlState::Success(None));
assert_eq!("01007".parse::<SqlState>()?, SqlState::Warning(Some(PrivilegeNotGranted)));

// Unknown codes are represented as `Other`
assert_eq!("XX001".parse::<SqlState>()?, SqlState::Other(String::from("XX001")));
```

Parsing return codes specific to PostgreSQL:
```rust
use sqlstate::{
    postgres::{
        class::{
            DataException::InvalidJsonText, InternalError::DataCorrupted,
            OperatorIntervention::CrashShutdown,
        },
        SqlState::*,
    },
    standard,
    PostgresSqlState,
};

assert_eq!("22032".parse::<PostgresSqlState>()?,
           PostgresSqlState::Custom(DataException(Some(InvalidJsonText))));
assert_eq!("XX001".parse::<PostgresSqlState>()?,
           PostgresSqlState::Custom(InternalError(Some(DataCorrupted))));

// Can also fall back to standard codes
assert_eq!("00000".parse::<PostgresSqlState>()?,
           PostgresSqlState::Standard(standard::SqlState::Success(None)));
```

## Features

- `postgres`: Enables PostgreSQL-specific types.
