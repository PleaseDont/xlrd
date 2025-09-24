use super::{
    font::Script,
    style::OutlineLevel,
    // xf::{HorizAlign, Pattern, ReadingOrder, VertAlign},
    xfext::{LongRGBA, XFPropGradient},
};
use binrw::BinRead;
use encoding_rs::Encoding;

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_magic == 0x0892))]
    _magic: u16,
    _len: u16,

    // frtHeader
    #[br(assert(_rt == 0x0892))]
    _rt: u16,
    #[br(assert(_flags == 0))]
    _flags: u16,
    #[br(assert(_reserved1 == 0))]
    _reserved1: u64,

    _byte1: u8,
    #[br(calc = _byte1 & 0x01 == 0x01)]
    _builtin: bool,
    #[br(calc = _byte1 >> 1 & 0x01 == 0x01)]
    _hidden: bool,
    #[br(calc = _byte1 >> 2 & 0x01 == 0x01, assert(if _custom { _builtin } else { true }))]
    _custom: bool,
    // reserved bits = 5
    _catagory: Category,

    #[br(args(_builtin))]
    _builtin_data: BuiltInData,

    _wstr: LPWideString,
    #[br(ignore)]
    _name: String,

    // xfProps
    #[br(assert(_reserved2 == 0))]
    _reserved2: u16,
    _cprops: u16,
    #[br(count = _cprops)]
    _props: Vec<XFProp>,
}

impl Data {
    pub fn _decode(&mut self, ecoding: &'static Encoding) {
        self._name = super::xlstring(ecoding, false, &self._wstr._bytes);
        self._props.iter_mut().for_each(|prop| match prop {
            XFProp::FontName {
                _cb: _,
                _wstr,
                _name,
            } => {
                *_name = super::xlstring(ecoding, false, &_wstr._bytes);
            }
            XFProp::NumberFormat {
                _cb: _,
                _ustr,
                _format,
            } => {
                *_format = super::xlstring(ecoding, _ustr.hbyte, &_ustr.bytes);
            }
            _ => (),
        });
    }
}

#[derive(Debug, BinRead)]
pub enum Category {
    #[br(magic = 0x00u8)]
    Custom,
    #[br(magic = 0x01u8)]
    GoodBadNeutral,
    #[br(magic = 0x02u8)]
    DataModel,
    #[br(magic = 0x03u8)]
    TitleHeading,
    #[br(magic = 0x04u8)]
    ThemedCell,
    #[br(magic = 0x05u8)]
    NumberFormat,
}

#[derive(Debug, BinRead)]
#[br(import(_builtin: bool))]
enum BuiltInData {
    #[br(pre_assert(_builtin))]
    Style {
        _isbi: u8,
        #[br(args(_isbi))]
        _outlevel: OutlineLevel,
    },
    #[br(pre_assert(!_builtin), assert(_v == 0xFFFF))]
    None { _v: u16 },
}

#[derive(Debug, BinRead)]
struct LPWideString {
    _cch: u16,
    #[br(count = _cch * 2)]
    _bytes: Vec<u8>,
}

#[derive(Debug, BinRead)]
enum XFProp {
    #[br(magic = 0x0000u16)]
    FillPattern { _cb: u16, _fill: Pattern },
    #[br(magic = 0x0001u16)]
    ForegroundColor { _cb: u16, _color: XFPropColor },
    #[br(magic = 0x0002u16)]
    BackgroundColor { _cb: u16, _color: XFPropColor },
    #[br(magic = 0x0003u16)]
    GradientFill { _cb: u16, _fill: XFPropGradient },
    #[br(magic = 0x0004u16)]
    GradientStop { _cb: u16, _stop: XFPropGradientStop },
    #[br(magic = 0x0005u16)]
    TextColor { _cb: u16, _color: XFPropColor },
    #[br(magic = 0x0006u16)]
    TopBorder { _cb: u16, _border: XFPropBorder },
    #[br(magic = 0x0007u16)]
    BottomBorder { _cb: u16, _border: XFPropBorder },
    #[br(magic = 0x0008u16)]
    LeftBorder { _cb: u16, _border: XFPropBorder },
    #[br(magic = 0x0009u16)]
    RightBorder { _cb: u16, _border: XFPropBorder },
    #[br(magic = 0x000Au16)]
    DiagonalBorder { _cb: u16, _border: XFPropBorder },
    #[br(magic = 0x000Bu16)]
    VerticalBorder { _cb: u16, _border: XFPropBorder },
    #[br(magic = 0x000Cu16)]
    HorizontalBorder { _cb: u16, _border: XFPropBorder },
    #[br(magic = 0x000Du16)]
    DiagonalUp {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _used: bool,
    },
    #[br(magic = 0x000Eu16)]
    DiagonalDown {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _used: bool,
    },
    #[br(magic = 0x000Fu16)]
    HorizAlign { _cb: u16, _align: HorizAlign },
    #[br(magic = 0x0010u16)]
    VertAlign { _cb: u16, _align: VertAlign },
    #[br(magic = 0x0011u16)]
    TextRotation { _cb: u16, _trot: u8 },
    #[br(magic = 0x0012u16)]
    Indentation {
        _cb: u16,
        #[br(assert(_level <= 15))]
        _level: u16,
    },
    #[br(magic = 0x0013u16)]
    ReadingOrder { _cb: u16, _order: ReadingOrder },
    #[br(magic = 0x0014u16)]
    TextWrap {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _wrap: bool,
    },
    #[br(magic = 0x0015u16)]
    TextDistribute {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _distribute: bool,
    },
    #[br(magic = 0x0016u16)]
    ShrinkToFit {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _shrink: bool,
    },
    #[br(magic = 0x0017u16)]
    CellMerged {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _merged: bool,
    },
    #[br(magic = 0x0018u16)]
    FontName {
        _cb: u16,
        _wstr: LPWideString,
        #[br(ignore)]
        _name: String,
    },
    #[br(magic = 0x0019u16)]
    FontBold { _cb: u16, _bold: Bold },
    #[br(magic = 0x001Au16)]
    Underline { _cb: u16, _underline: Underline },
    #[br(magic = 0x001Bu16)]
    Script { _cb: u16, _script: Script },
    #[br(magic = 0x001Cu16)]
    TextItalic {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _italic: bool,
    },
    #[br(magic = 0x001Du16)]
    TextStrikethrough {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _strikethrough: bool,
    },
    #[br(magic = 0x001Eu16)]
    TextOutline {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _outline: bool,
    },
    #[br(magic = 0x001Fu16)]
    TextShadow {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _shadow: bool,
    },
    #[br(magic = 0x0020u16)]
    TextCondense {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _condense: bool,
    },
    #[br(magic = 0x0021u16)]
    TextExtend {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _extend: bool,
    },
    #[br(magic = 0x0022u16)]
    CharSet { _cb: u16, _charset: CharSet },
    #[br(magic = 0x0023u16)]
    FontFamily { _cb: u16, _family: Family },
    #[br(magic = 0x0024u16)]
    FontSize {
        _cb: u16,
        #[br(assert(matches!(_size, 20..=8191)))]
        _size: u32,
    },
    #[br(magic = 0x0025u16)]
    FontScheme { _cb: u16, _scheme: FontScheme },
    #[br(magic = 0x0026u16)]
    NumberFormat {
        _cb: u16,
        _ustr: super::XLUnicodeString,
        #[br(ignore)]
        _format: String,
    },
    #[br(magic = 0x0029u16)]
    NumberFormatId { _cb: u16, _id: u16 },
    #[br(magic = 0x002Au16)]
    RelativeIndentation {
        _cb: u16,
        #[br(assert(matches!(_level, -15..=15 | 255)))]
        _level: i16,
    },
    #[br(magic = 0x002Bu16)]
    Locked {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _locked: bool,
    },
    #[br(magic = 0x002Cu16)]
    Hidden {
        _cb: u16,
        #[br(map = |x: u8| x == 0x01)]
        _hidden: bool,
    },
}

#[derive(Debug, BinRead)]
pub enum Pattern {
    #[br(magic = 0x00u8)]
    None,
    #[br(magic = 0x01u8)]
    Solid,
    #[br(magic = 0x02u8)]
    MediumGray,
    #[br(magic = 0x03u8)]
    DarkGray,
    #[br(magic = 0x04u8)]
    LightGray,
    #[br(magic = 0x05u8)]
    DarkHorizontal,
    #[br(magic = 0x06u8)]
    DarkVertical,
    #[br(magic = 0x07u8)]
    DarkDown,
    #[br(magic = 0x08u8)]
    DarkUp,
    #[br(magic = 0x09u8)]
    DarkGrid,
    #[br(magic = 0x0Au8)]
    DarkTrellis,
    #[br(magic = 0x0Bu8)]
    LightHorizontal,
    #[br(magic = 0x0Cu8)]
    LightVertical,
    #[br(magic = 0x0Du8)]
    LightDown,
    #[br(magic = 0x0Eu8)]
    LightUp,
    #[br(magic = 0x0Fu8)]
    LightGrid,
    #[br(magic = 0x10u8)]
    LightTrellis,
    #[br(magic = 0x11u8)]
    Gray125,
    #[br(magic = 0x12u8)]
    Gray0625,
}

#[derive(Debug, BinRead)]
pub enum HorizAlign {
    #[br(magic = 0x00u8)]
    General,
    #[br(magic = 0x01u8)]
    Left,
    #[br(magic = 0x02u8)]
    Center,
    #[br(magic = 0x03u8)]
    Right,
    #[br(magic = 0x04u8)]
    Fill,
    #[br(magic = 0x05u8)]
    Justify,
    #[br(magic = 0x06u8)]
    CenterContinuous,
    #[br(magic = 0x07u8)]
    Distributed,
}

#[derive(Debug, BinRead)]
pub enum VertAlign {
    #[br(magic = 0x00u8)]
    Top,
    #[br(magic = 0x01u8)]
    Center,
    #[br(magic = 0x02u8)]
    Bottom,
    #[br(magic = 0x03u8)]
    Justify,
    #[br(magic = 0x04u8)]
    Distributed,
}

#[derive(Debug, BinRead)]
pub enum ReadingOrder {
    #[br(magic = 0x00u8)]
    Context,
    #[br(magic = 0x01u8)]
    Ltr,
    #[br(magic = 0x02u8)]
    Rtl,
}

#[derive(Debug, BinRead)]
struct XFPropColor {
    #[br(assert(_byte1 & 0x01 == 0x01), restore_position)] // valid_rgba must be true
    _byte1: u8,
    #[br(args(_byte1 >> 1))]
    _color_type: XColorType,

    #[br(args(&_color_type))]
    _color_index: ColorIndex,

    _tint_shade: i16, // darken <--> lighten

    _rgba: LongRGBA,
}

#[derive(Debug, BinRead)]
#[br(import(byte1: u8))]
enum XColorType {
    #[br(pre_assert(byte1 == 0x00))] // Automatic color
    Auto { _v: u8 },
    #[br(pre_assert(byte1 == 0x01))] // Indexed color
    Indexed { _v: u8 },
    #[br(pre_assert(byte1 == 0x02))] // RGB color
    Rgb { _v: u8 },
    #[br(pre_assert(byte1 == 0x03))] // Theme color
    Themed { _v: u8 },
    #[br(pre_assert(byte1 == 0x04))] // not set
    None { _v: u8 },
}

#[derive(Debug, BinRead)]
#[br(import(color_type: &XColorType))]
enum ColorIndex {
    #[br(pre_assert(matches!(color_type, &XColorType::Indexed{_v:_})))]
    Index {
        #[br(map = |x: u8| x & 0x7F)]
        _v: u8,
    },
    #[br(pre_assert(matches!(color_type, &XColorType::Themed{_v:_})))]
    Theme { _theme: ColorTheme },
    #[br(pre_assert(!matches!(color_type, &XColorType::Indexed{_v:_} | &XColorType::Themed{_v:_})))]
    None { _v: u8 },
}

#[derive(Debug, BinRead)]
struct XFPropGradientStop {
    _unused: u16, // 0x00000000
    #[br(assert(matches!(_position, 0.0..=1.0)))]
    _position: f64, // 0.0..=1.0
    _color: XFPropColor,
}

#[allow(dead_code)]
#[derive(Debug, BinRead)]
enum BorderStyle {
    #[br(magic = 0x0000u16)] // No border
    None,
    #[br(magic = 0x0001u16)] // Thin line
    Thin,
    #[br(magic = 0x0002u16)] // Medium line
    Medium,
    #[br(magic = 0x0003u16)] // Dashed line
    Dashed,
    #[br(magic = 0x0004u16)] // Dotted line
    Dotted,
    #[br(magic = 0x0005u16)] // Thick line
    Thick,
    #[br(magic = 0x0006u16)] // Double line
    Double,
    #[br(magic = 0x0007u16)] // Hairline
    Hair,
    #[br(magic = 0x0008u16)] // Medium dashed line
    MediumDashed,
    #[br(magic = 0x0009u16)] // Dash-dot line
    DashDot,
    #[br(magic = 0x000Au16)] // Medium dash-dot line
    MediumDashDot,
    #[br(magic = 0x000Bu16)] // Dash-dot-dot line
    DashDotDot,
    #[br(magic = 0x000Cu16)] // Medium dash-dot-dot line
    MediumDashDotDot,
    #[br(magic = 0x000Du16)] // Slanted dash-dot-dot line
    SlantedDashDotDot,
}

#[derive(Debug, BinRead)]
struct XFPropBorder {
    _color: XFPropColor,
    _border_style: BorderStyle,
}

#[derive(Debug, BinRead)]
enum Bold {
    #[br(magic = 0x0190u16)]
    Normal,
    #[br(magic = 0x02BCu16)]
    Bold,
}

#[derive(Debug, BinRead)]
enum FontScheme {
    #[br(magic = 0x00u8)]
    None,
    #[br(magic = 0x01u8)]
    Major,
    #[br(magic = 0x02u8)]
    Minor,
    #[br(magic = 0xFFu8)]
    Nil,
}

#[derive(Debug, BinRead)]
enum ColorTheme {
    #[br(magic = 0x00u8)]
    Dark1,
    #[br(magic = 0x01u8)]
    Light1,
    #[br(magic = 0x02u8)]
    Dark2,
    #[br(magic = 0x03u8)]
    Light2,
    #[br(magic = 0x04u8)]
    Accent1,
    #[br(magic = 0x05u8)]
    Accent2,
    #[br(magic = 0x06u8)]
    Accent3,
    #[br(magic = 0x07u8)]
    Accent4,
    #[br(magic = 0x08u8)]
    Accent5,
    #[br(magic = 0x09u8)]
    Accent6,
    #[br(magic = 0x0Au8)]
    Hyperlink,
    #[br(magic = 0x0Bu8)]
    FollowedHyperlink,
}

#[derive(Debug, BinRead)]
enum Underline {
    #[br(magic = 0x0000u16)]
    None,
    #[br(magic = 0x0001u16)]
    Single,
    #[br(magic = 0x0002u16)]
    Double,
    #[br(magic = 0x0021u16)]
    SingleAccountant,
    #[br(magic = 0x0022u16)]
    DoubleAccountant,
}

#[derive(Debug, BinRead)]
enum Family {
    #[br(magic = 0x00u8)]
    None,
    #[br(magic = 0x01u8)]
    Roman,
    #[br(magic = 0x02u8)]
    Swiss,
    #[br(magic = 0x03u8)]
    Modern,
    #[br(magic = 0x04u8)]
    Script,
    #[br(magic = 0x05u8)]
    Decorative,
}

#[derive(Debug, BinRead)]
enum CharSet {
    #[br(magic = 0x00u8)]
    Ansi,
    #[br(magic = 0x01u8)]
    Default,
    #[br(magic = 0x02u8)]
    Symbol,
    #[br(magic = 0x4Du8)]
    Mac,
    #[br(magic = 0x80u8)]
    Shiftjis,
    #[br(magic = 0x81u8)]
    Hangeul,
    #[br(magic = 0x82u8)]
    Johab,
    #[br(magic = 0x86u8)]
    Gb2312,
    #[br(magic = 0x88u8)]
    ChineseBig5,
    #[br(magic = 0xA1u8)]
    Greek,
    #[br(magic = 0xA2u8)]
    Turkish,
    #[br(magic = 0xA3u8)]
    Vietnamese,
    #[br(magic = 0xB1u8)]
    Hebrew,
    #[br(magic = 0xB2u8)]
    Arabic,
    #[br(magic = 0xBAu8)]
    Baltic,
    #[br(magic = 0xCCu8)]
    Russian,
    #[br(magic = 0xDDu8)]
    Thai,
    #[br(magic = 0xEEu8)]
    EastEurope,
    #[br(magic = 0xFFu8)]
    Oem,
}
