use sqlstate_macros::class;

#[class(non_standard)]
#[non_exhaustive]
pub enum Warning {
    #[subclass("008")]
    ImplicitZeroBitPadding,
    #[subclass("P01")]
    DeprecatedFeature,
}
