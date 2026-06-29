pub mod primitives {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct Timestamp(i64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct FpInt<const N: u32>(i64);

    pub type Px = FpInt<8>;
    pub type Qty = FpInt<8>;

    pub enum Side {
        Buy,
        Sell,
    }
}

pub mod wire {
    include!(concat!(env!("OUT_DIR"), "/trade_schema.rs"));
    include!(concat!(env!("OUT_DIR"), "/lob_update_schema.rs"));
}
