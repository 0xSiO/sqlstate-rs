use sqlstate_macros::state;

pub mod class;

use self::class::*;

// TODO: For FromStr, throw error if unable to parse rather than returning Other("...")
//       If error, then try parsing standard SqlState instead
pub enum SqlState {
    Standard(crate::standard::SqlState),
    Custom(PostgresSqlState),
}

#[state(non_standard)]
pub enum PostgresSqlState {
    Warning(Warning),
    SqlStatementNotYetComplete,
}
