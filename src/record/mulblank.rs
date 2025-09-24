use binrw::BinRead;

// 2.4.175
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    _row: u16,
    #[br(assert(_col_min <= 254))]
    _col_min: u16,

    #[br(count = (_len - 6) / 2)]
    _ixfs: Vec<u16>,

    #[br(assert(_col_min < _col_max))]
    _col_max: u16,
}
