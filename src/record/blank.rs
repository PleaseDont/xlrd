use binrw::BinRead;

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 6))]
    _len: u16,

    _row: u16,
    _col: u16,
    _ixfe: u16,
}
