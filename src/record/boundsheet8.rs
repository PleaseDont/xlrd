use binrw::BinRead;
use encoding_rs::Encoding;
use enum_display::EnumDisplay;
use modular_bitfield::{Specifier, bitfield, prelude::B6};

// 2.4.28
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    pub pos: u32,

    #[br(restore_position)]
    _byte1: u8,
    pub state: SheetState,
    pub r#type: SheetType,

    xlstr: super::ShortXLUnicodeString,

    #[br(ignore)]
    pub name: String,
}

impl Data {
    pub fn decode(&mut self, encoding: &'static Encoding) {
        self.name = super::xlstring(encoding, self.xlstr.hbyte, &self.xlstr.bytes);
    }
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes)]
pub struct SheetState {
    #[skip(setters)]
    pub state: State,
    #[skip]
    __: B6,
}

#[derive(Debug, Specifier, EnumDisplay)]
#[bits = 2]
pub enum State {
    Visible,    // 0x0
    Hidden,     // 0x1
    VeryHidden, // 0x2
}

#[derive(Debug, BinRead)]
pub enum SheetType {
    #[br(magic(0x00u8))]
    Worksheet,
    #[br(magic(0x01u8))]
    Macrosheet,
    #[br(magic(0x02u8))]
    Chartsheet,
    #[br(magic(0x06u8))]
    VBAModule,
}
