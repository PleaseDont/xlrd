use binrw::BinRead;
use encoding_rs::Encoding;

// 2.4.52
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 2))]
    _len: u16,

    pub value: u16,

    #[br(ignore)]
    pub encoding: Option<&'static Encoding>,
}

impl Data {
    pub fn decode(&mut self) {
        self.encoding = codepage::to_encoding(self.value);
    }
}
