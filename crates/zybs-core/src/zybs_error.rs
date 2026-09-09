use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ZybsError {
    passthrough: Option<Box<dyn Error>>,
    message: Option<String>,
}

impl Display for ZybsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.passthrough.is_some() && self.message.is_some() {
            write!(f, "an error occurred: {}\nwith the underlying cause: {}", self.message.as_ref().unwrap(), self.passthrough.as_ref().unwrap())
        } else if let Some(e) = &self.passthrough {
            write!(f, "an underlying error occurred: {}", e)
        } else if let Some(e) = &self.message {
            write!(f, "an error occurred: {}", e)
        } else {
            write!(f, "an unknown error occurred")
        }
    }
}

impl Error for ZybsError {}