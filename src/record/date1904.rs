use binrw::BinRead;

// 2.4.77
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 2))]
    _len: u16,

    #[br(map = |x: u16| x == 0x0001)]
    pub is1904: bool,
}
