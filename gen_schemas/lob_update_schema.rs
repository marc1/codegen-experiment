pub struct WireLobUpdate {
    pub t: i64,
    pub px: i64,
    pub qty: i64,
    pub side: bool,
}
pub struct LobUpdate {
    pub t: crate::primitives::Timestamp,
    pub px: crate::primitives::FpInt<8u32>,
    pub qty: crate::primitives::FpInt<8u32>,
    pub side: crate::primitives::Side,
}
