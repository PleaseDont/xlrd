use binrw::BinRead;

// 2.4.21
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 16))]
    _len: u16,

    #[br(assert(_version == 0x0600))] // BIFF8
    _version: u16,
    pub stream_type: StreamType, // dt
    _build: u16,
    #[br(assert(_year == 0x07CC || _year == 0x07CD))]
    _year: u16,
    _mask: u64, // ignore
}

#[derive(Debug, BinRead)]
pub enum StreamType {
    #[br(magic(0x0005u16))]
    Workbook,
    #[br(magic(0x0010u16))]
    Worksheet,
    #[br(magic(0x0020u16))]
    Chartsheet,
    #[br(magic(0x0040u16))]
    Macrosheet,
}
