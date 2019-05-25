//! # Utils Module
use crate::utils::log::log_error;
use std::io;

pub mod log;

pub fn check_stream_write(result: io::Result<usize>) {
    match result {
        Err(err) => {
            log_error(&err);
        }
        Ok(_value) => {}
    }
}

pub fn check_console_write(result: io::Result<()>) {
    match result {
        Err(err) => {
            log_error(&err);
        }
        Ok(_value) => {}
    }
}
