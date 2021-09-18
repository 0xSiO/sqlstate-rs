Representations and parsing logic for [`SQLSTATE`](https://en.wikipedia.org/wiki/SQLSTATE)
return codes.

## Example

```rust
use sqlstate::{
   standard::{
       class::{DataException::DivisionByZero, Warning::PrivilegeNotGranted},
       SqlState,
   },
};

assert_eq!("00000".parse::<SqlState>()?, SqlState::Success(None));
assert_eq!("01007".parse::<SqlState>()?, SqlState::Warning(Some(PrivilegeNotGranted)));
assert_eq!("22012".parse::<SqlState>()?, SqlState::DataException(Some(DivisionByZero)));
assert_eq!("XX102".parse::<SqlState>()?, SqlState::Other(String::from("XX102")));
```

## Features

- `postgres`: Enables PostgreSQL-specific types
