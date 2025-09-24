use binrw::BinRead;
use modular_bitfield::{
    bitfield,
    prelude::{B1, B3, B12},
};

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 16))]
    _len: u16,

    pub row: u16, // rowmic <= row <= rowmac
    #[br(assert(_col_min <= 0x00FF))]
    _col_min: u16,
    #[br(assert(_col_max <= 0x0100))]
    _col_max: u16,
    #[br(assert(matches!(height, 2..=8192)))]
    pub height: u16,

    #[br(assert(_reserved1 == 0))]
    _reserved1: u16,
    _unused1: u16, // undefined ignored

    pub info: Info,
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes,
    assert(!self.reserved2() && self.reserved3() == 1))]
pub struct Info {
    #[skip]
    outlevel: B3,
    #[skip(setters)]
    reserved2: bool,
    #[skip]
    collapsed: bool,
    #[skip(setters)]
    pub hidden: bool,
    #[skip]
    user_set: bool,
    #[skip]
    formatted: bool,
    #[skip(setters)]
    reserved3: u8,
    #[skip]
    ixfe: B12,
    #[skip(setters)]
    pub top_bdr: bool,
    #[skip]
    bot_bdr: bool,
    #[skip]
    phonetic: bool,
    #[skip]
    __: B1,
}
