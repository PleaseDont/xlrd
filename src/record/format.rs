use binrw::BinRead;
use encoding_rs::Encoding;

// 2.4.126
// built-in format
// 0 General
// 1 0
// 2 0.00
// 3 #,##0
// 4 #,##0.00
// 9 0%
// 10 0.00%
// 11 0.00E+00
// 12 # ?/?
// 13 # ??/??
// 14 mm-dd-yy
// 15 d-mmm-yy
// 16 d-mmm
// 17 mmm-yy
// 18 h:mm AM/PM
// 19 h:mm:ss AM/PM
// 20 h:mm
// 21 h:mm:ss
// 22 m/d/yy h:mm
// 27 yyyy"年"m"月"
// 28 m"月"d"日"
// 29 m"月"d"日"
// 30 m-d-yy
// 31 yyyy"年"m"月"d"日"
// 32 h"时"mm"分"
// 33 h"时"mm"分"ss"秒"
// 34 上午/下午 h"时"mm"分"
// 35 上午/下午 h"时"mm"分"ss"秒"
// 36 yyyy"年"m"月"
// 37 #,##0 ;(#,##0)
// 38 #,##0 ;[Red](#,##0)
// 39 #,##0.00;(#,##0.00)
// 40 #,##0.00;[Red](#,##0.00)
// 45 mm:ss
// 46 [h]:mm:ss
// 47 mmss.0
// 48 ##0.0E+0
// 49 @
// 50 yyyy"年"m"月"
// 51 m"月"d"日"
// 52 yyyy"年"m"月"
// 53 m"月"d"日"
// 54 m"月"d"日"
// 55 上午/下午 h"时"mm"分"
// 56 上午/下午 h"时"mm"分"ss"秒"
// 57 yyyy"年"m"月"
// 58 m"月"d"日"
// 59 t0
// 60 t0.00
// 61 t#,##0
// 62 t#,##0.00
// 67 t0%
// 68 t0.00%
// 69 t# ?/?
// 70 t# ??/??
// 71 ว/ด/ปปปป
// 72 ว-ดดด-ปป
// 73 ว-ดดด
// 74 ดดด-ปป
// 75 ช:นน
// 76 ช:นน:ทท
// 77 ว/ด/ปปปป ช:นน
// 78 นน:ทท
// 79 [ช]:นน:ทท
// 80 นน:ทท.0
// 81 d/m/bb
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    #[br(map = |x: u16| x as u32, assert(matches!(ifmt, 5..=8 | 23..=26 | 41..=44 | 63..=66 | 164..=392)))]
    pub ifmt: u32,

    ustr: super::XLUnicodeString,

    #[br(ignore)]
    pub code: String,
}
impl Data {
    pub fn decode(&mut self, encoding: &'static Encoding) {
        self.code = super::xlstring(encoding, self.ustr.hbyte, &self.ustr.bytes);
    }
}
