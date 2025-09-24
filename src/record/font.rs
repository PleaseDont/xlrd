use binrw::BinRead;
use encoding_rs::Encoding;
use enum_display::EnumDisplay;
use modular_bitfield::{bitfield, prelude::B1};

// 2.4.122
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    #[br( assert(matches!(height, 0 | 20..=8191)))]
    pub height: u16,

    pub info: Info,

    #[br(assert(_reserved == 0x00))]
    _reserved: u8,

    #[br(assert(matches!(icv, 0x0008..=0x003F | 0x0051 | 0x7FFF)))]
    pub icv: u16, // color index

    #[br(assert(matches!(_weight, 0| 100..=1000)))]
    _weight: u16,
    #[br(calc = _weight == 700)]
    pub bold: bool,

    _script: Script,
    pub underline: Underline,
    #[br(assert(matches!(family, 0..=5)))]
    pub family: u8,
    #[br(assert(matches!(charset, 0..=2 | 77 | 128..=130 | 134 | 136 | 161..=163 | 177 | 178 | 186 | 204 | 221 | 238 | 255)))]
    pub charset: u8,

    _unused3: u8,

    xlstr: super::ShortXLUnicodeString,

    #[br(ignore)]
    pub name: String,
}

impl Data {
    pub fn decode(&mut self, encoding: &'static Encoding) {
        self.name = super::xlstring(encoding, self.xlstr.hbyte, &self.xlstr.bytes)
    }
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes)]
pub struct Info {
    #[skip]
    unused1: B1,
    #[skip(setters)]
    pub italic: bool,
    #[skip]
    unused2: B1,
    #[skip(setters)]
    pub strike_out: bool,
    #[skip]
    outline: bool,
    #[skip]
    shadow: bool,
    #[skip]
    condense: bool,
    #[skip]
    extended: bool,
}

#[derive(Debug, BinRead)]
pub enum Script {
    #[br(magic = 0x0000u16)]
    None,
    #[br(magic = 0x0001u16)]
    Super,
    #[br(magic = 0x0002u16)]
    Sub,
}

#[derive(Debug, BinRead, EnumDisplay)]
#[enum_display(case = "Camel")]
pub enum Underline {
    #[br(magic = 0x00u8)]
    None,
    #[br(magic = 0x01u8)]
    Single,
    #[br(magic = 0x02u8)]
    Double,
    #[br(magic = 0x21u8)]
    SingleAccountant,
    #[br(magic = 0x22u8)]
    DoubleAccountant,
}
