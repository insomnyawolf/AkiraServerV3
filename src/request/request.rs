use crate::request::form::*;
use crate::request::headers::*;
use crate::request::method::*;

use crate::request::other::Other;
use crate::utils::log::log_warning;
use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Default)]
// ToDo UrlEncoded Variables
/// This Struct Contains all the request data parsed and ready to use
pub struct Request {
    #[derivative(Debug = "ignore")]
    raw: String,
    #[derivative(Debug = "ignore")]
    pub is_valid_request: bool,
    pub method: Method,
    pub path: String,
    pub request_headers: RequestHeaders,
    pub form_data: FormData,
    pub other: Other,
}

impl Request {
    /// Parse request and headers from byte buffer
    pub fn parse(mut stream: &TcpStream, timeout: Option<Duration>) -> Request {
        // Create Structure with default values
        let mut req = Request::default();
        let mut form_data = FormData::default();
        let mut headers = RequestHeaders::default();

        // Create Empty Byte Vector
        let mut buffer_full: Vec<u8> = Vec::new();

        stream.set_read_timeout(timeout).ok();
        // Read bytes for the specified timeout
        stream.read_to_end(&mut buffer_full).ok();

        //Parse request data
        req.raw = unsafe { String::from_utf8_unchecked(buffer_full) }
            .to_string()
            .replace('\u{0}', "");

        //println!("{}", &req.raw);

        let request_arr: Vec<&str> = req.raw.splitn(3, ' ').collect();

        if request_arr.len() >= 3 {
            req.method = Method::from_str(&request_arr[0].to_string());

            let rs: Vec<&str> = request_arr[2].splitn(2, "\r\n\r\n").collect();
            for part in rs {
                if part.starts_with("HTTP") {
                    headers = RequestHeaders::parse(part);
                } else if part.contains(&headers.content_bounds) && &headers.content_bounds != "" {
                    form_data.add_multipart(part.to_string(), &headers.content_bounds);
                } else if part.contains("=") {
                    form_data.add_url_encoded(part.to_string());
                } else {
                    if part != "" {
                        println!("Failed to parse:\n{}\n", part);
                    }
                }
            }
            req.request_headers = headers;
            req.form_data = form_data;

            match percent_encoding::percent_decode(request_arr[1].as_bytes()).decode_utf8() {
                Ok(value) => {
                    req.path = value.trim_start_matches("/..").to_string();
                    req.is_valid_request = true;
                }
                Err(err) => {
                    log_warning(&err);
                }
            }
        }
        req.other = Other::parse(stream);
        req
    }

    /// Obtains resource path relative to the specified location
    pub fn get_local_path(&self, root_folder: &String) -> String {
        root_folder.to_string() + &self.path
    }

    /// Returns Raw String
    pub fn get_raw(&self) -> String {
        self.raw.to_string()
    }
}
