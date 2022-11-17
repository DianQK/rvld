use std::{array::TryFromSliceError, io};
use thiserror::Error;

pub type LinkerResult<T> = Result<T, LinkerError>;

#[derive(Error, Debug)]
pub enum LinkerError {
    #[error("wrong args")]
    WrongArgs,
    #[error("file not exists")]
    IOError(#[from] io::Error),
    #[error("not an ELF file")]
    NotELF,
    #[error("byte read fail")]
    ByteReadFail(#[from] TryFromSliceError),
}
