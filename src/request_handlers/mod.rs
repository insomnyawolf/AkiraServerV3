use crate::utils::log::log_error;
use std::io;

pub mod get_handler;
pub mod unssuported_handler;

fn check_write(result: io::Result<usize>) {
    match result {
        Err(err) => {
            log_error(&err);
        }
        Ok(_value) => {}
    }
}
