use binrw::BinRead;

#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    #[br(assert(_ccv == 56))]
    _ccv: u16,

    #[br(count = _ccv)]
    rgbs: Vec<LongRGB>,

    #[br(ignore)]
    pub colors: Vec<String>,
}
impl Data {
    pub fn decode(&mut self) {
        self.colors = self.rgbs.iter().map(|rgb| rgb.argb()).collect();
    }
}

#[derive(Debug, BinRead)]
pub struct LongRGB {
    red: u8,
    green: u8,
    blue: u8,
    #[br(assert(_reserved == 0x00))]
    _reserved: u8,
}
impl LongRGB {
    pub fn argb(&self) -> String {
        format!("FF{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
