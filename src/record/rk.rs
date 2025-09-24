use binrw::BinRead;
use modular_bitfield::{bitfield, prelude::B30};

// 2.4.220
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 10))]
    _len: u16,

    pub row: u16,
    pub col: u16,

    _rkrec: RkRec,

    #[br(calc = _rkrec.ixfe())]
    pub ixfe: u16,
    #[br(calc = { let v = _rkrec.num() as f64; if _rkrec.fx100() { v / 100.0 } else { v } })]
    pub num: f64,
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes)]
pub struct RkRec {
    #[skip(setters)]
    pub ixfe: u16,
    #[skip(setters)]
    pub fx100: bool,
    #[skip]
    fint: bool,
    #[skip(setters)]
    pub num: B30,
}
