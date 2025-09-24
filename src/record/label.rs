use binrw::BinRead;
use encoding_rs::Encoding;

#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    pub row: u16,
    pub col: u16,
    pub ixfe: u16,

    xlstr: super::XLUnicodeString,

    #[br(ignore)]
    pub content: String,
}

impl Data {
    pub fn decode(&mut self, encoding: &'static Encoding) {
        self.content = super::xlstring(encoding, self.xlstr.hbyte, &self.xlstr.bytes);
    }
}
