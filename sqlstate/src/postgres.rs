use sqlstate_macros::subclass;

pub enum SqlState {
    Standard(crate::SqlState),
    Custom(PostgresSqlState),
}

pub enum PostgresSqlState {
    Warning(Warning),
    SqlStatementNotYetComplete,
}

#[subclass]
pub enum Warning {
    #[state("008")]
    ImplicitZeroBitPadding,
    #[state("P01")]
    DeprecatedFeature,
}
