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
    pub form_data: FormData,
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

        let request_arr: Vec<_> = req.raw.splitn(3, ' ').collect();

        let request_arr: Vec<&str> = req.raw.splitn(3, ' ').collect();

        if request_arr.len() >= 3 {
            req.method = Method::from_str(&request_arr[0].to_string()).unwrap();
            req.path = percent_encoding::percent_decode(request_arr[1].as_bytes())
                .decode_utf8()
                .unwrap()
                .to_string();

            let data: Vec<&str> = request_arr[2]
                .split("content-type: multipart/form-data; ")
                .collect();
            let data_lenght = data.len();
            if data_lenght > 0 {
                req.request_headers = RequestHeaders::parse(data[0]);
            }
            if data_lenght > 1 {
                req.form_data = FormData::parse(data[1])
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
            } else if RequestHeaders::generate_field_string(
                &mut headers.accept_charset,
                &current,
                "Accept-Charset: ",
            ) {
            } else if current.starts_with("Accept-Encoding: ") {
                let arr: Vec<&str> = current
                    .trim_start_matches("Accept-Encoding: ")
                    .split(" ")
                    .collect();
                for data in arr {
                    headers.accept_encoding.push(data.to_string());
                }
            } else if RequestHeaders::generate_field_string(
                &mut headers.accept_language,
                &current,
                "Accept-Language: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.accept_datetime,
                &current,
                "Accept-Datetime: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.access_control_request_method,
                &current,
                "Access-Control-Request-Method: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.authorization,
                &current,
                "Authorization: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.cache_control,
                &current,
                "Cache-Control: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.connection,
                &current,
                "Connection: ",
            ) {
            } else if current.starts_with("Content-Length: ") {
                headers.content_length = current
                    .trim_start_matches("Content-Length: ")
                    .to_string()
                    .parse::<u64>()
                    .unwrap();
            } else if RequestHeaders::generate_field_string(
                &mut headers.content_md5,
                &current,
                "Content-MD5: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.content_type,
                &current,
                "Content-Type: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.cookie,
                &current,
                "Cookie: ",
            ) {
            } else if RequestHeaders::generate_field_string(&mut headers.date, &current, "Date: ") {
            } else if RequestHeaders::generate_field_string(
                &mut headers.expect,
                &current,
                "Expect: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.forwarded,
                &current,
                "Forwarded: ",
            ) {
            } else if RequestHeaders::generate_field_string(&mut headers.from, &current, "From: ") {
            } else if RequestHeaders::generate_field_string(&mut headers.host, &current, "Host: ") {
            } else if RequestHeaders::generate_field_string(
                &mut headers.max_forwards,
                &current,
                "Max-Forwards: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.origin,
                &current,
                "Origin: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.pragma,
                &current,
                "Pragma: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.proxy_authorization,
                &current,
                "Proxy-Authorization: ",
            ) {
            } else if RequestHeaders::generate_field_string(&mut headers.range, &current, "Range: ")
            {
            } else if RequestHeaders::generate_field_string(
                &mut headers.referer,
                &current,
                "Referer: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.transfer_encodings,
                &current,
                "TE: ",
            ) {
            } else if RequestHeaders::generate_field_string(
                &mut headers.user_agent,
                &current,
                "User-Agent: ",
            ) {
            } else if RequestHeaders::generate_field_string(&mut headers.via, &current, "Via: ") {
            } else if RequestHeaders::generate_field_string(
                &mut headers.warning,
                &current,
                "Warning: ",
            ) {
            } else if RequestHeaders::generate_field_string(&mut headers.version, &current, "HTTP/")
            {
            } else if RequestHeaders::generate_field_string(
                &mut headers.upgrade_insecure_requests,
                &current,
                "Upgrade-Insecure-Requests: ",
            ) {
            } else if RequestHeaders::generate_field_string(&mut headers.dnt, &current, "DNT: ") {
            } else {
                headers.other.push(current);
            }
        }
        headers
    }

    fn generate_field_string(field: &mut String, data: &String, pattern: &str) -> bool {
        if data.starts_with(pattern) {
            *field = data.trim_start_matches(pattern).to_string();
            return true;
        }
        false
    }
}

#[derive(Debug, Default)]
pub struct FormData {
    pub data: String,
}

impl FormData {
    pub fn parse(raw: &str) -> FormData {
        let mut form_data = FormData::default();
        /*let data: Vec<&str> = raw.trim_start_matches("boundary=").rsplit("").collect();
        for thing in data {
            println!("Dat\n{}\nDat", thing);
        }*/
        println!("{}", raw);
        form_data.data = raw.to_string();
        form_data
    }
}
