use hidapi::{self, HidError};
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct NoDeviceError {
    pub msg: String,
}
impl Error for NoDeviceError {}
impl fmt::Display for NoDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No studio display found")
    }
}
impl NoDeviceError {
    pub fn new(message: String) -> Self {
        Self { msg: message }
    }
}

#[derive(Debug)]
pub struct InvalidBitSize {
    expected: usize,
    recieved: usize,
}
impl Error for InvalidBitSize {}
impl fmt::Display for InvalidBitSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Expected a size of {:?} got {:?}",
            self.expected, self.recieved
        )
    }
}
impl InvalidBitSize {
    pub fn new(expected: usize, recieved: usize) -> Self {
        Self {
            expected: expected,
            recieved: recieved,
        }
    }
}

#[derive(Debug)]
pub struct ConversionError {}
impl Error for ConversionError {}
impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conversion failed")
    }
}
impl ConversionError {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub enum DisplayError {
    HidError(HidError),
    NoDeviceError(NoDeviceError),
    InvalidBitSize(InvalidBitSize),
    ConversionError(ConversionError),
}

impl Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::HidError(error) => write!(f, "{}", error),
            DisplayError::NoDeviceError(error) => write!(f, "{}", error),
            DisplayError::InvalidBitSize(error) => write!(f, "{}", error),
            DisplayError::ConversionError(error) => write!(f, "{}", error),
        }
    }
}

impl From<HidError> for DisplayError {
    fn from(error: HidError) -> Self {
        DisplayError::HidError(error)
    }
}

impl From<NoDeviceError> for DisplayError {
    fn from(error: NoDeviceError) -> Self {
        DisplayError::NoDeviceError(error)
    }
}

impl From<InvalidBitSize> for DisplayError {
    fn from(error: InvalidBitSize) -> Self {
        DisplayError::InvalidBitSize(error)
    }
}

impl From<ConversionError> for DisplayError {
    fn from(error: ConversionError) -> Self {
        DisplayError::ConversionError(error)
    }
}

impl Error for DisplayError {}