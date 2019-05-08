use crate::request::multipart_form::*;
use crate::request::headers::*;
use crate::request::method::*;

use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Default)]
pub struct Request {
    #[derivative(Debug = "ignore")]
    raw: String,
    pub is_valid_request: bool,
    pub method: Method,
    pub path: String,
    pub request_headers: RequestHeaders,
    pub form_data: MultipartFormData,
}

impl Request {
    /** Parse request and headers from byte buffer **/
    pub fn parse(mut stream: TcpStream, timeout: Option<Duration>) -> Request {
        // Create Structure with default values
        let mut req = Request::default();

        // Create Empty Byte Vector
        let mut buffer_full: Vec<u8> = Vec::new();

        stream.set_read_timeout(timeout).ok();
        // Read bytes for the specified timeout
        stream.read_to_end(&mut buffer_full).ok();

        //Parse request data
        req.raw = String::from_utf8_lossy(&buffer_full.as_slice())
            .to_string()
            .replace('\u{0}', "");

        //println!("{}\n", req.raw);

        let request_arr: Vec<&str> = req.raw.splitn(3, ' ').collect();

        if request_arr.len() >= 3 {
            req.method = Method::from_str(&request_arr[0].to_string()).unwrap();
            req.path = percent_encoding::percent_decode(request_arr[1].as_bytes())
                .decode_utf8()
                .unwrap()
                .to_string();

            if request_arr[2].contains("boundary=--------------------------") {
                let data = request_arr[2].replace("boundary=", "");
                let split_data: Vec<&str> = data.split("--------------------------").collect();

                let mut multipart_form = MultipartFormData::default();
                for thing in split_data {
                    if thing.starts_with("--") {
                        multipart_form.add(thing.to_string());
                    } else {
                        req.request_headers = RequestHeaders::parse(thing);
                    }
                }
                req.form_data = multipart_form;
            } else {
                req.request_headers = RequestHeaders::parse(request_arr[2]);
            }

            req.is_valid_request = true;
        }
        req
    }

    /** Obtains resource path relative to the specified location **/
    pub fn get_local_path(&self, root_folder: &String) -> String {
        root_folder.to_string() + &self.path
    }

    /** Returns Raw String **/
    pub fn get_raw(&self) -> String {
        self.raw.to_string()
    }
}
