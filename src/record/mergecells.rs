use binrw::BinRead;

// 2.4.168
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    #[br(assert(_cmcs <= 1026))]
    _cmcs: u16,

    #[br(count = _cmcs)]
    pub refs: Vec<Ref8>,
}

#[derive(Debug, BinRead)]
#[br(assert(row_min <= row_max && col_min <= col_max))]
pub struct Ref8 {
    pub row_min: u16,
    pub row_max: u16,
    pub col_min: u16,
    pub col_max: u16,
}
