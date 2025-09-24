#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Binrw(#[from] binrw::Error),
    #[error(transparent)]
    Xlsx(#[from] umya_spreadsheet::XlsxError),

    #[error("File type may not supported")]
    XlsExt,
    #[error("Wrong stream type, expected {expect:?}, got {actual:?}")]
    StreamType {
        expect: crate::record::bof::StreamType,
        actual: crate::record::bof::StreamType,
    },
    #[error("Couldn't detect encoding from codepage {0}")]
    CodePage(u16),
    #[error("Encrypted file not supported")]
    FillPass,
}

impl Error {
    pub fn msg(s: impl Into<String>) -> Self {
        Self::Custom(s.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
