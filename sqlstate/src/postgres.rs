pub mod class;

use self::class::*;

// TODO: For FromStr, throw error if unable to parse rather than returning Other("...")
//       If error, then try parsing standard SqlState instead
pub enum SqlState {
    Standard(crate::standard::SqlState),
    Custom(PostgresSqlState),
}

pub enum PostgresSqlState {
    Warning(Warning),
    SqlStatementNotYetComplete,
}
