#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    RegexError(#[from] regex::Error),
    #[error("goblin error")]
    ParseError(#[from] goblin::error::Error),
    #[error("unsupported  binary format")]
    UnsupportedBinaryFormat(&'static str),
    #[error("unsupported  binary format")]
    Bincode(#[from] Box<bincode::ErrorKind>),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    UnresolvedRvaError(u32),
    #[error("{0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("undefined stream")]
    UndefinedStream,
    #[error("undefined meta data table index (0..63) {0}")]
    UndefinedMetaDataTableIndex(u32),
    #[error("undefined meta data table name {0}")]
    UndefinedMetaDataTableName(&'static str),
    #[error("data not enough {0} required {1}")]
    NotEnoughData(usize, usize),
    #[error("not implemented")]
    NoiImplementedError,
}
