use binrw::BinRead;

// 2.4.90
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 14))]
    _len: u16,

    #[br(assert(_row_min <= 0x0000FFFF))]
    _row_min: u32,
    #[br(assert(_row_max <= 0x00010000))]
    _row_max: u32,
    #[br(assert(_col_min <= 0x00FF))]
    _col_min: u16,
    #[br(assert(_col_max <= 0x0100))]
    _col_max: u16,

    #[br(assert(_reserved == 0x0000))]
    _reserved: u16,
}
