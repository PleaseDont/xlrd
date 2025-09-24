use super::rk::RkRec;
use binrw::BinRead;

// 2.4.175
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    pub row: u16,
    #[br(assert(col_min <= 254))]
    pub col_min: u16,

    #[br(count = _len / 6 - 1)]
    _rks: Vec<RkRec>,

    #[br(assert(col_min < _col_max))]
    _col_max: u16,

    #[br(calc = _rks.iter().map(|rk| {
        let v = rk.num() as f64;
        (rk.ixfe(), if rk.fx100() { v / 100.0 } else { v })
    }).collect::<Vec<_>>())]
    pub values: Vec<(u16, f64)>, // (ixfe, num)
}
