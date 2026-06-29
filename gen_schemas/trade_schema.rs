pub struct WireTrade {
    pub t: i64,
    pub px: i64,
    pub qty: i64,
    pub aggr: bool,
}
pub struct Trade {
    pub t: crate::primitives::Timestamp,
    pub px: crate::primitives::FpInt<8u32>,
    pub qty: i64,
    pub aggr: crate::primitives::Side,
}
