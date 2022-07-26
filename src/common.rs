use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum UsciError {
  IOError(io::Error),
  CommonError(String)
}

impl From<io::Error> for UsciError {

  fn from(error: io::Error) -> Self {
    return UsciError::IOError(error);
  }

}

impl Display for UsciError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      UsciError::IOError(ref err) => write!(f, "IO Error: {}", err),
      UsciError::CommonError(ref message) => write!(f, "Error: {}", message),
    }
  }
}

impl Error for UsciError {

  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      UsciError::IOError(ref err) => Some(err),
      _ => None
    }
  }

}

pub type UsciResult<T> = Result<T, UsciError>;