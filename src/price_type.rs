#[cfg(feature = "bigdecimal")]
pub type PriceType = bigdecimal::BigDecimal;

#[cfg(not(feature = "bigdecimal"))]
pub type PriceType = f64;

pub fn price_type_from_f64_ok(value: f64) -> Option<PriceType> {
    #[cfg(feature = "bigdecimal")]
    {
        PriceType::try_from(value).ok()
    }
    #[cfg(not(feature = "bigdecimal"))]
    {
        Some(value)
    }
}
