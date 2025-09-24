use binrw::BinRead;

// 2.4.117
#[allow(dead_code)]
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,
    _enc_type: u16,
    #[br(count = _len - 2)]
    _enc_info: Vec<u8>,
}
