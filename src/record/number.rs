use binrw::BinRead;

// 2.4.180
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 14))]
    _len: u16,

    pub row: u16,
    pub col: u16,
    pub ixfe: u16,

    pub num: f64,
}
