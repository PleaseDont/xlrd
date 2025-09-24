use binrw::BinRead;

#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    // frtHeader
    #[br(assert(_rt == 0x087D))]
    _rt: u16,
    #[br(assert(_flags == 0))]
    _flags: u16,
    #[br(assert(_reserved == 0))]
    _reserved: u64,
    #[br(assert(_reserved1 == 0))]
    _reserved1: u16,

    #[br(assert(_ixfe <= 4050))]
    _ixfe: u16,

    #[br(assert(_reserved2 == 0))]
    _reserved2: u16,

    _cexts: u16,

    #[br(count = _cexts)]
    _exts: Vec<ExtProp>,
}

#[derive(Debug, BinRead)]
enum ExtProp {
    #[br(magic = 0x0004u16)]
    Foreground { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x0005u16)]
    Background { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x0006u16)]
    Gradient { _cb: u16, _fill: XFExtGradient },
    #[br(magic = 0x0007u16)]
    TopBorder { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x0008u16)]
    BottomBorder { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x0009u16)]
    LeftBorder { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x000Au16)]
    RightBorder { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x000Bu16)]
    DiagonalBorder { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x000Du16)]
    Text { _cb: u16, _color: FullColorExt },
    #[br(magic = 0x000Eu16)]
    Font { _cb: u16, _scheme: FontScheme },
    #[br(magic = 0x000Fu16)]
    Indent {
        _cb: u16,
        #[br(assert(_level <= 250))]
        _level: u16,
    },
}

#[derive(Debug, BinRead)]
enum XColorType {
    #[br(magic = 0x0000u16)] // Automatic color
    Auto,
    #[br(magic = 0x0001u16)] // Indexed color
    Indexed,
    #[br(magic = 0x0002u16)] // RGB color
    Rgb,
    #[br(magic = 0x0003u16)] // Theme color
    Themed,
    #[br(magic = 0x0004u16)] // not set
    None,
}

#[derive(Debug, BinRead)]
pub struct LongRGBA {
    _red: u8,
    _green: u8,
    _blue: u8,
    _alpha: u8,
}

#[derive(Debug, BinRead)]
enum ColorTheme {
    #[br(magic = 0x0000_0000u32)]
    Dark1,
    #[br(magic = 0x0000_0001u32)]
    Light1,
    #[br(magic = 0x0000_0002u32)]
    Dark2,
    #[br(magic = 0x0000_0003u32)]
    Light2,
    #[br(magic = 0x0000_0004u32)]
    Accent1,
    #[br(magic = 0x0000_0005u32)]
    Accent2,
    #[br(magic = 0x0000_0006u32)]
    Accent3,
    #[br(magic = 0x0000_0007u32)]
    Accent4,
    #[br(magic = 0x0000_0008u32)]
    Accent5,
    #[br(magic = 0x0000_0009u32)]
    Accent6,
    #[br(magic = 0x0000_000Au32)]
    Hyperlink,
    #[br(magic = 0x0000_000Bu32)]
    FollowedHyperlink,
}

#[derive(Debug, BinRead)]
#[br(import(color_type: &XColorType))]
enum ColorValue {
    #[br(pre_assert(matches!(color_type, &XColorType::Auto | &XColorType::None)), assert(_val == 0))]
    None { _val: u32 },
    #[br(pre_assert(matches!(color_type, &XColorType::Indexed)))]
    Index {
        #[br(map = |x: u32| x & 0x7F)]
        _val: u32,
    },
    #[br(pre_assert(matches!(color_type, &XColorType::Rgb)))]
    Rgba { _color: LongRGBA },
    #[br(pre_assert(matches!(color_type, &XColorType::Themed)))]
    Theme { _theme: ColorTheme },
}

#[derive(Debug, BinRead)]
struct FullColorExt {
    _color_type: XColorType,
    _tint_shade: i16, // darken <--> lighten

    #[br(args(&_color_type))]
    _color_value: ColorValue,
    _unused: u64, // undefined, ignored.
}

#[derive(Debug, BinRead)]
enum GradientType {
    #[br(magic = 0x00000000u32)]
    Linear,
    #[br(magic = 0x00000001u32)]
    Rectangular,
}

#[derive(Debug, BinRead)]
pub struct XFPropGradient {
    _gradient_type: GradientType,
    _degree: f64,
    #[br(assert(matches!(_fill2left, 0.0..=1.0)))]
    _fill2left: f64,
    #[br(assert(matches!(_fill2right, 0.0..=1.0)))]
    _fill2right: f64,
    #[br(assert(matches!(_fill2top, 0.0..=1.0)))]
    _fill2top: f64,
    #[br(assert(matches!(_fill2bottom, 0.0..=1.0)))]
    _fill2bottom: f64,
}

#[derive(Debug, BinRead)]
struct GradStop {
    _color_type: XColorType,
    _value: u32, // todo
    #[br(assert(matches!(_position, 0.0..=1.0)))]
    _position: f64,
    #[br(assert(matches!(_position, -1.0..=1.0)))]
    _tint: f64, // darken <--> lighten
}

#[derive(Debug, BinRead)]
struct XFExtGradient {
    _gradient: XFPropGradient,
    _cstops: u32,
    #[br(count = _cstops)]
    _grad_stops: Vec<GradStop>,
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
