use binrw::BinRead;
use modular_bitfield::{bitfield, prelude::B12};

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 4))]
    _len: u16,

    _info: Info,
    #[br(assert(matches!(_height, 0..=8179)))]
    _height: i16,
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes,
    assert(self.reserved() == 0))]
struct Info {
    #[skip]
    user_set: bool,
    #[skip]
    zero: bool,
    #[skip]
    top_bdr: bool,
    #[skip]
    bot_bdr: bool,
    #[skip(setters)]
    reserved: B12,
}
