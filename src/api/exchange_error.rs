use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeError {
    error_code: u32,
    error_msg: String,
}

impl ExchangeError {
    pub fn new(msg: &str, code: u32) -> Self {
        Self {
            error_msg: msg.to_string(),
            error_code: code,
        }
    }
}
impl fmt::Display for ExchangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Api Error  message {}, code {}",
            self.error_msg, self.error_code
        )?;
        Ok(())
    }
}

impl Error for ExchangeError {}
