use binrw::BinRead;

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 2))]
    _len: u16,

    #[br(assert(_width <= 0x00FF))]
    _width: u16,
}
