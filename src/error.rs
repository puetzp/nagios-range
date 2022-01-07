use std::error::Error as StdError;
use std::fmt;

/// A global error enum whose variants encapsulate more specific
/// error types.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyRange,
    StartGreaterThanEnd,
    ParseStartPoint(std::num::ParseIntError),
    ParseEndPoint(std::num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyRange => write!(f, "the range string must not be empty"),
            Self::StartGreaterThanEnd => {
                write!(f, "the start point must be lesser than the end point")
            }
            Self::ParseStartPoint(e) => {
                write!(
                    f,
                    "the start point could not be parsed as signed integer: {}",
                    e
                )
            }
            Self::ParseEndPoint(e) => {
                write!(
                    f,
                    "the end point could not be parsed as signed integer: {}",
                    e
                )
            }
        }
    }
}

impl StdError for Error {}
