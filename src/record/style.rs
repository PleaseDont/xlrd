use binrw::BinRead;
use encoding_rs::Encoding;

// 2.4.269
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    #[br(map = |x: u16| x & 0x0FFF, restore_position)]
    _ixfe: u16,
    /// _unused bits = 3
    #[br(map = |x: u16| x >> 15 == 0x0001)]
    _builtin: bool,

    #[br(args(_builtin))]
    pub style: Style,

    #[br(restore_position)]
    _ext_magic: u16,
    #[br(if(_ext_magic == 0x0892))]
    _ext: Option<super::styleext::Data>,
}
impl Data {
    pub fn decode(&mut self, encoding: &'static Encoding) {
        if let Style::User { ustr, name } = &mut self.style {
            *name = super::xlstring(encoding, ustr.hbyte, &ustr.bytes);
        };
    }
}

#[derive(Debug, BinRead)]
#[br(import(_builtin: bool))]
pub enum Style {
    #[br(pre_assert(_builtin))]
    BuiltIn {
        _isbi: u8,
        #[br(args(_isbi))]
        _outlevel: OutlineLevel,
    },
    #[br(pre_assert(!_builtin))]
    User {
        ustr: super::XLUnicodeString,
        #[br(ignore)]
        name: String,
    },
}

#[derive(Debug, BinRead)]
#[br(import(isbi: u8))]
pub enum OutlineLevel {
    #[br(magic = 0x00u8, pre_assert(isbi < 0x03))]
    Outline1,
    #[br(magic = 0x01u8, pre_assert(isbi < 0x03))]
    Outline2,
    #[br(magic = 0x02u8, pre_assert(isbi < 0x03))]
    Outline3,
    #[br(magic = 0x03u8, pre_assert(isbi < 0x03))]
    Outline4,
    #[br(magic = 0x04u8, pre_assert(isbi < 0x03))]
    Outline5,
    #[br(magic = 0x05u8, pre_assert(isbi < 0x03))]
    Outline6,
    #[br(magic = 0x06u8, pre_assert(isbi < 0x03))]
    Outline7,
    #[br(magic = 0xFFu8)]
    None,
}
