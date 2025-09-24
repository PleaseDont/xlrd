use binrw::BinRead;

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 10))]
    _len: u16,

    pub row: u16,
    pub col: u16,
    pub ixfe: u16,
    pub isst: u32,
}
