use binrw::BinRead;
use enum_display::EnumDisplay;
use modular_bitfield::{
    Specifier, bitfield,
    prelude::{B2, B4, B7, B12},
};

// 2.4.353
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 20))]
    _len: u16,

    #[br(assert(ifnt != 4 && ifnt <= 1022))]
    pub ifnt: u16, // 2 bytes
    pub ifmt: u16, // 2 bytes

    pub protection: Protection, // 2 bytes
    pub alignment: Alignment,   // 3 bytes
    _independent: Independent,  // 1 byte
    #[br(pad_after = -1)]
    pub borders: Borders, // 8 bytes
    #[br(map = |x: u8| x >> 1 & 0x01 == 0x01, restore_position)]
    _has_ext: bool, // -1 byte
    #[br(pad_after = -1)]
    pub fill: Fill, // 3 bytes
    #[br(map = |x: u8| x >> 6 & 0x01 == 0x01, restore_position)]
    _pivot_button: bool, // 0 byte
    #[br(map = |x: u8| x >> 7 & 0x01 == 0x01, assert(!_reserved3))]
    _reserved3: bool, // 0 byte
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes,
    assert(if self.is_style() { !self.prefix123() && self.parent_ixfe() == 0x0FFF } else { true }))]
pub struct Protection {
    #[skip(setters)]
    pub locked: bool, //  1 bit
    #[skip(setters)]
    pub hidden: bool, //  1 bit
    #[skip(setters)]
    is_style: bool, //  1 bit
    #[skip(setters)]
    prefix123: bool, //  1 bit
    #[skip(setters)]
    parent_ixfe: B12, // 12 bits
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes,
    assert(if self.just_last() { self.horiz_align() == HorizAlign::Distributed } else { true }),
    assert(!self.reserved()))]
pub struct Alignment {
    #[skip(setters)]
    pub horiz_align: HorizAlign, // 3 bits
    #[skip(setters)]
    pub warp_text: bool, // 1 bit
    #[skip(setters)]
    pub vert_align: VertAlign, // 3 bits
    #[skip(setters)]
    just_last: bool, // 1 bit
    #[skip(setters)]
    pub text_rotation: u8, // 8 bits
    #[skip]
    indent: B4, // 4 bits
    #[skip]
    shrink_fit: bool, // 1 bit
    #[skip(setters)]
    reserved: bool, // 1 bit
    #[skip]
    reading_order: ReadingOrder, // 2 bits
}

#[derive(Debug, Specifier, PartialEq)]
#[bits = 3]
pub enum HorizAlign {
    General,          // 0x00
    Left,             // 0x01
    Center,           // 0x02
    Right,            // 0x03
    Fill,             // 0x04
    Justify,          // 0x05
    CenterContinuous, // 0x06
    Distributed,      // 0x07
}

#[derive(Debug, Specifier)]
#[bits = 3]
pub enum VertAlign {
    Top,         // 0x00
    Center,      // 0x01
    Bottom,      // 0x02
    Justify,     // 0x03
    Distributed, // 0x04
}

#[derive(Debug, Specifier)]
#[bits = 2]
pub enum ReadingOrder {
    Context, // 0x00
    Ltr,     // 0x01
    Rtl,     // 0x02
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes,
    assert(self.reserved() == 0))]
pub struct Independent {
    #[skip(setters)]
    reserved: B2, // 2 bits
    #[skip]
    format: bool, // 1 bit
    #[skip]
    font: bool, // 1 bit
    #[skip]
    alignment: bool, // 1 bit
    #[skip]
    border: bool, // 1 bit
    #[skip]
    fill: bool, // 1 bit
    #[skip]
    protection: bool, // 1 bit
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes)]
pub struct Borders {
    #[skip(setters)]
    pub left_style: BorderStyle, // 4 bits
    #[skip(setters)]
    pub right_style: BorderStyle, // 4 bits
    #[skip(setters)]
    pub top_style: BorderStyle, // 4 bits
    #[skip(setters)]
    pub bottom_style: BorderStyle, // 4 bits
    #[skip(setters)]
    pub left_icv: B7, // 7 bits
    #[skip(setters)]
    pub right_icv: B7, // 7 bits
    #[skip(setters)]
    pub diagonal_type: B2, // 2 bits
    #[skip(setters)]
    pub top_icv: B7, // 7 bits
    #[skip(setters)]
    pub bottom_icv: B7, // 7 bits
    #[skip(setters)]
    pub diagonal_icv: B7, // 7 bits
    #[skip(setters)]
    pub diagonal_style: BorderStyle, // 4 bits
    #[skip]
    __: B7, // 7 bits
}

#[derive(Debug, Specifier, EnumDisplay)]
#[enum_display(case = "Camel")]
#[bits = 4]
pub enum BorderStyle {
    None,              // 0x0
    Thin,              // 0x1
    Medium,            // 0x2
    Dashed,            // 0x3
    Dotted,            // 0x4
    Thick,             // 0x5
    Double,            // 0x6
    Hair,              // 0x7
    MediumDashed,      // 0x8
    DashDot,           // 0x9
    MediumDashDot,     // 0xA
    DashDotDot,        // 0xB
    MediumDashDotDot,  // 0xC
    SlantedDashDotDot, // 0xD
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes)]
pub struct Fill {
    #[skip]
    __: B2, // 2 bits
    #[skip(setters)]
    pub pattern: Pattern, // 6 bits
    #[skip(setters)]
    pub fore_icv: B7, // 7 bits
    #[skip(setters)]
    pub back_icv: B7, // 7 bits
    #[skip]
    __: B2, // 2 bits
}

#[derive(Debug, Specifier)]
#[bits = 6]
pub enum Pattern {
    None,            // 0x00
    Solid,           // 0x01
    MediumGray,      // 0x02
    DarkGray,        // 0x03
    LightGray,       // 0x04
    DarkHorizontal,  // 0x05
    DarkVertical,    // 0x06
    DarkDown,        // 0x07
    DarkUp,          // 0x08
    DarkGrid,        // 0x09
    DarkTrellis,     // 0x0A
    LightHorizontal, // 0x0B
    LightVertical,   // 0x0C
    LightDown,       // 0x0D
    LightUp,         // 0x0E
    LightGrid,       // 0x0F
    LightTrellis,    // 0x10
    Gray125,         // 0x11
    Gray0625,        // 0x12
}
