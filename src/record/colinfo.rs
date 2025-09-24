use binrw::BinRead;
use modular_bitfield::{
    bitfield,
    prelude::{B1, B3, B4},
};

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 12))]
    _len: u16,

    #[br(assert(col_min <= 0x0100))]
    pub col_min: u16,
    #[br(assert(col_min <= col_max && col_max <= 0x0100))]
    pub col_max: u16,
    pub width: u16,
    _ixfe: u16,

    pub info: Info,
    _unused2: u16, // undefined ignored
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes,
    assert(self.reserved1() == 0 && self.reserved2() == 0))]
pub struct Info {
    #[skip(setters)]
    pub hidden: bool,
    #[skip(setters)]
    pub user_set: bool,
    #[skip(setters)]
    pub best_fit: bool,
    #[skip]
    phonetic: bool,
    #[skip(setters)]
    reserved1: B4,
    #[skip]
    outlevel: B3,
    #[skip]
    __: B1,
    #[skip]
    collapsed: bool,
    #[skip(setters)]
    reserved2: B3,
}
