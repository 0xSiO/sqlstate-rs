use sqlstate_macros::class;

pub enum SqlState {
    Standard(crate::standard::SqlState),
    Custom(PostgresSqlState),
}

pub enum PostgresSqlState {
    Warning(Warning),
    SqlStatementNotYetComplete,
}

#[class]
pub enum Warning {
    #[subclass("008")]
    ImplicitZeroBitPadding,
    #[subclass("P01")]
    DeprecatedFeature,
}
