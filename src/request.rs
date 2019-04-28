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

// https://tools.ietf.org/html/rfc2616#section-5.1.1
/** Enum that contains references to possible request methods **/
#[derive(Debug, PartialEq)]
pub enum Method {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    POST,
    PUT,
    TRACE,
    Unsupported,
}

impl Default for Method {
    fn default() -> Method {
        Method::Unsupported
    }
}

impl Method {
    /** Parse Method from String **/
    pub fn from_str(s: &String) -> Option<Method> {
        let string: &str = &s[..];
        match string {
            "CONNECT" => Some(Method::CONNECT),
            "DELETE" => Some(Method::DELETE),
            "GET" => Some(Method::GET),
            "HEAD" => Some(Method::HEAD),
            "OPTIONS" => Some(Method::OPTIONS),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "TRACE" => Some(Method::TRACE),
            _ => None,
        }
    }

    /** Converts Method to string **/
    pub fn as_str(&self) -> &str {
        match *self {
            Method::CONNECT => "CONNECT",
            Method::DELETE => "DELETE",
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::TRACE => "TRACE",
            Method::Unsupported => "Unsupported",
        }
    }
}

// https://en.wikipedia.org/wiki/List_of_HTTP_header_fields

#[derive(Debug, Default)]
pub struct RequestHeaders {
    // Standard request fields
    /** Acceptable instance-manipulations for the request **/
    pub acceptable_instance_manipulations: String,
    /** Media type(s) that is/are acceptable for the response **/
    pub accept: Vec<String>,
    /** Character sets that are acceptable **/
    pub accept_charset: String,
    /** List of acceptable encodings **/
    pub accept_encoding: Vec<String>,
    /** List of acceptable human languages for response **/
    pub accept_language: String,
    /** Acceptable version in time **/
    pub accept_datetime: String,
    /** Initiates a request for cross-origin resource sharing with Origin **/
    pub access_control_request_method: String,
    /** Authentication credentials for HTTP authentication **/
    pub authorization: String,
    /** Used to specify directives that must be obeyed by all caching mechanisms along the request-response chain **/
    pub cache_control: String,
    /** Control options for the current connection and list of hop-by-hop request fields **/
    pub connection: String,
    /** The length of the request body in octets (8-bit bytes) **/
    pub content_length: u64,
    /** A Base64-encoded binary MD5 sum of the content of the request body **/
    pub content_md5: String,
    /** The Media type of the body of the request (used with POST and PUT requests) **/
    pub content_type: String,
    /** An HTTP cookie previously sent by the server with Set-Cookie (below) **/
    pub cookie: String,
    /** The date and time at which the message was originated **/
    pub date: String,
    // Indicates that particular server behaviors are required by the client **/
    pub expect: String,
    /** Disclose original information of a client connecting to a web server through an HTTP proxy **/
    pub forwarded: String,
    /** The email address of the user making the request **/
    pub from: String,
    /** The domain name of the server (for virtual hosting), and the TCP port number on which the server is listening **/
    pub host: String,
    /* ToDo Omited
    HTTP2-Settings
    If-Match
    If-Modified-Since
    If-None-Match
    If-Range
    If-Unmodified-Since
    */
    /** Limit the number of times the message can be forwarded through proxies or gateways **/
    pub max_forwards: String,
    /** Initiates a request for cross-origin resource sharing (asks server for Access-Control-* response fields) **/
    pub origin: String,
    /** Implementation-specific fields that may have various effects anywhere along the request-response chain **/
    pub pragma: String,
    /** Authorization credentials for connecting to a proxy **/
    pub proxy_authorization: String,
    /** Request only part of an entity. Bytes are numbered from 0 **/
    pub range: String,
    /** This is the address of the previous web page from which a link to the currently requested page was followed (misspelled in the RFC) **/
    pub referer: String,
    /** The transfer encodings the user agent is willing to accept:
    the same values as for the response header field Transfer-Encoding can be used,
    plus the "trailers" value (related to the "chunked" transfer method)
    to notify the server it expects to receive additional fields in the trailer after the last, zero-sized, chunk **/
    pub transfer_encodings: String,
    /** user agent string **/
    pub user_agent: String,

    // ToDo Omited Upgrade
    /** Informs the server of proxies through which the request was sent **/
    pub via: String,
    /** A general warning about possible problems with the entity body **/
    pub warning: String,

    // End Of Std Fields

    // Common non-standard request fields
    /** HTTP Protocol used version **/
    pub version: String,
    /** Tells a server which (presumably in the middle of a HTTP -> HTTPS migration) hosts mixed content that the client would prefer redirection to HTTPS **/
    pub upgrade_insecure_requests: String,
    /** Requests a web application to disable their tracking of a user **/
    pub dnt: String,

    /** Undefined headers **/
    pub other: Vec<String>,
}

impl RequestHeaders {
    pub fn parse(raw: &str) -> RequestHeaders {
        let mut headers = RequestHeaders::default();

        let client_arr: Vec<&str> = raw.rsplit("\r\n").collect();

        // ToDo Improove this loop
        for data in client_arr {
            let current = data.to_string();

            if RequestHeaders::generate_field_string(
                &mut headers.acceptable_instance_manipulations,
                &current,
                "A-IM: ",
            ) {
            } else if current.starts_with("Accept: ") {
                // Todo Check This
                let values = current.trim_start_matches("Accept: ").replace(";", ",");
                let arr: Vec<&str> = values.split(",").collect();

                for data in arr {
                    headers.accept.push(data.to_string());
                }
            } else if generate_field_string(
                &mut headers.accept_charset,
                &current,
                "Accept-Charset: ",
            ) {
            } else if generate_field_string_vec(
                &mut headers.accept_encoding,
                &current,
                "Accept-Encoding: ",
            ) {
            } else if generate_field_string(
                &mut headers.accept_language,
                &current,
                "Accept-Language: ",
            ) {
            } else if generate_field_string(
                &mut headers.accept_datetime,
                &current,
                "Accept-Datetime: ",
            ) {
            } else if generate_field_string(
                &mut headers.access_control_request_method,
                &current,
                "Access-Control-Request-Method: ",
            ) {
            } else if generate_field_string(&mut headers.authorization, &current, "Authorization: ")
            {
            } else if generate_field_string(&mut headers.cache_control, &current, "Cache-Control: ")
            {
            } else if generate_field_string(&mut headers.connection, &current, "Connection: ") {
            } else if generate_field_u64(&mut headers.content_length, &current, "Content-Length: ")
            {
            } else if generate_field_string(&mut headers.content_md5, &current, "Content-MD5: ") {
            } else if generate_field_string(&mut headers.content_type, &current, "Content-Type: ") {
            } else if generate_field_string(&mut headers.cookie, &current, "Cookie: ") {
            } else if generate_field_string(&mut headers.date, &current, "Date: ") {
            } else if generate_field_string(&mut headers.expect, &current, "Expect: ") {
            } else if generate_field_string(&mut headers.forwarded, &current, "Forwarded: ") {
            } else if generate_field_string(&mut headers.from, &current, "From: ") {
            } else if generate_field_string(&mut headers.host, &current, "Host: ") {
            } else if generate_field_string(&mut headers.max_forwards, &current, "Max-Forwards: ") {
            } else if generate_field_string(&mut headers.origin, &current, "Origin: ") {
            } else if generate_field_string(&mut headers.pragma, &current, "Pragma: ") {
            } else if generate_field_string(
                &mut headers.proxy_authorization,
                &current,
                "Proxy-Authorization: ",
            ) {
            } else if generate_field_string(&mut headers.range, &current, "Range: ") {
            } else if generate_field_string(&mut headers.referer, &current, "Referer: ") {
            } else if generate_field_string(&mut headers.transfer_encodings, &current, "TE: ") {
            } else if generate_field_string(&mut headers.user_agent, &current, "User-Agent: ") {
            } else if generate_field_string(&mut headers.via, &current, "Via: ") {
            } else if generate_field_string(&mut headers.warning, &current, "Warning: ") {
            } else if generate_field_string(&mut headers.version, &current, "HTTP/") {
            } else if generate_field_string(
                &mut headers.upgrade_insecure_requests,
                &current,
                "Upgrade-Insecure-Requests: ",
            ) {
            } else if generate_field_string(&mut headers.dnt, &current, "DNT: ") {
            } else {
                headers.other.push(current.to_string());
            }
        }
        headers
    }
}

fn generate_field_string(field: &mut String, data: &str, pattern: &str) -> bool {
    if data.to_lowercase().starts_with(&pattern.to_lowercase()[..]) {
        *field = data[pattern.len()..].to_string();
        return true;
    }
    false
}

fn generate_field_string_vec(field: &mut Vec<String>, data: &str, pattern: &str) -> bool {
    if data.to_lowercase().starts_with(&pattern.to_lowercase()[..]) {
        let s = data[pattern.len()..].to_string();
        let values: Vec<&str> = s.split(" ").collect();
        for value in values {
            field.push(value.to_string());
        }
        return true;
    }
    false
}

fn generate_field_u64(field: &mut u64, data: &str, pattern: &str) -> bool {
    if data.to_lowercase().starts_with(&pattern.to_lowercase()[..]) {
        *field = data[pattern.len()..].parse::<u64>().unwrap();
        return true;
    }
    false
}

fn generate_field_vec_u8(field: &mut Vec<u8>, data: &str) {
    *field = data.as_bytes().to_owned();
}

#[derive(Debug, Default, PartialEq)]
pub struct MultipartFormData {
    pub elements: Vec<MultipartFormElement>,
    pub other: Vec<String>,
}

impl MultipartFormData {
    pub fn add(&mut self, data: String) {
        let stripped_data = &data[28..];
        if stripped_data != "\r\n" {
            let content_disposition = "Content-Disposition: form-data; ";
            if stripped_data.starts_with(content_disposition) {
                self.elements.push(MultipartFormElement::new(
                    stripped_data[content_disposition.len()..]
                        .trim_end_matches("\r\n")
                        .to_string(),
                ));
            } else {
                self.other.push(stripped_data.to_string());
            }
        }
    }
}

//ToDO Actually parse form fields
#[derive(Debug, Default, PartialEq)]
pub struct MultipartFormElement {
    //pub data: Vec<String>,
}

impl MultipartFormElement {
    pub fn new(data: String) -> MultipartFormElement {
        let mut element = MultipartFormElement::default();
        //element.data = ;
        let data: Vec<&str> = data.split("\r\n\r\n").collect();
        for line in data {
            println!("New:{}", line);
        }
        element
    }
}
