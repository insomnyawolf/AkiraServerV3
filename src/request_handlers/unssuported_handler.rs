use crate::request_handlers::check_write;
use crate::response::headers::ResponseHeaders;
use crate::response::status::HttpStatus;
use crate::utils::log::log_warning;
use std::io::Write;
use std::net::TcpStream;

pub fn handle_unsupported(mut stream: &TcpStream) {
    log_warning(&"Unsupported Method");
    let mut headers = ResponseHeaders::new(HttpStatus::NotImplemented);
    check_write(stream.write(&headers.get_headers().as_bytes()));
}
