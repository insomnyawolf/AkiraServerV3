#[derive(Debug, Default)]
pub struct Request {
    pub is_valid_request: bool,
    pub method: String,
    pub path: String,
    pub request_headers: RequestHeaders,
}

impl Request {
    /** Parse request and headers from byte buffer **/
    pub fn parse(buffer: &[u8]) -> Request {
        let mut req = Request::default();

        let raw = String::from_utf8_lossy(&buffer)
            .to_string()
            .replace("\u{0}", "");

        let request_arr: Vec<_> = raw.splitn(3, ' ').collect();

        if request_arr.len() == 3 {
            req.method = request_arr[0].to_string();
            req.path = percent_encoding::percent_decode(request_arr[1].as_bytes())
                .decode_utf8()
                .unwrap()
                .to_string();
            let mut headers = RequestHeaders::default();
            headers.parse(request_arr[2]);
            req.request_headers = headers;
            req.is_valid_request = true;
        }
        req
    }

    pub fn get_method(&self) -> Option<Method> {
        Method::from_str(&self.method)
    }

    /** Obtains resource path relative to the specified location **/
    pub fn get_local_path(&self, root_folder: &String) -> String {
        root_folder.to_string() + &self.path
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
    pub fn parse(&mut self, client_str: &str) {
        let client_arr: Vec<&str> = client_str.rsplit("\r\n").collect();

        // ToDo Improove this loop
        for data in client_arr {
            let current = data.to_string();

            if current.starts_with("A-IM: ") {
                self.acceptable_instance_manipulations =
                    current.trim_start_matches("A-IM: ").to_string();
            } else if current.starts_with("Accept: ") {
                // Todo Check This
                let values = current.trim_start_matches("Accept: ").replace(";", ",");
                let arr: Vec<&str> = values.split(",").collect();

                for data in arr {
                    self.accept.push(data.to_string());
                }
            } else if current.starts_with("Accept-Charset: ") {
                self.accept_charset = current.trim_start_matches("Accept-Charset: ").to_string();
            } else if current.starts_with("Accept-Encoding: ") {
                let arr: Vec<&str> = current
                    .trim_start_matches("Accept-Encoding: ")
                    .split(" ")
                    .collect();
                for data in arr {
                    self.accept_encoding.push(data.to_string());
                }
            } else if current.starts_with("Accept-Language: ") {
                self.accept_language = current.trim_start_matches("Accept-Language: ").to_string();
            } else if current.starts_with("Accept-Datetime: ") {
                self.accept_datetime = current.trim_start_matches("Accept-Datetime: ").to_string();
            } else if current.starts_with("Access-Control-Request-Method: ") {
                self.access_control_request_method = current
                    .trim_start_matches("Access-Control-Request-Method: ")
                    .to_string();
            } else if current.starts_with("Authorization: ") {
                self.authorization = current.trim_start_matches("Authorization: ").to_string();
            } else if current.starts_with("Cache-Control: ") {
                self.cache_control = current.trim_start_matches("Cache-Control: ").to_string();
            } else if current.starts_with("Connection: ") {
                self.connection = current.trim_start_matches("Connection: ").to_string();
            } else if current.starts_with("Content-Length: ") {
                self.content_length = current
                    .trim_start_matches("Content-Length: ")
                    .to_string()
                    .parse::<u64>()
                    .unwrap();
            } else if current.starts_with("Content-MD5: ") {
                self.content_md5 = current.trim_start_matches("Content-MD5: ").to_string();
            } else if current.starts_with("Content-Type: ") {
                self.content_type = current.trim_start_matches("Content-Type: ").to_string();
            } else if current.starts_with("Cookie: ") {
                self.cookie = current.trim_start_matches("Cookie: ").to_string();
            } else if current.starts_with("Date: ") {
                self.date = current.trim_start_matches("Date: ").to_string();
            } else if current.starts_with("Expect: ") {
                self.expect = current.trim_start_matches("Expect: ").to_string();
            } else if current.starts_with("Forwarded: ") {
                self.forwarded = current.trim_start_matches("Forwarded: ").to_string();
            } else if current.starts_with("Host: ") {
                self.host = current.trim_start_matches("Host: ").to_string();
            } else if current.starts_with("Max-Forwards: ") {
                self.max_forwards = current.trim_start_matches("Max-Forwards: ").to_string();
            } else if current.starts_with("Origin: ") {
                self.origin = current.trim_start_matches("Origin: ").to_string();
            } else if current.starts_with("Pragma: ") {
                self.pragma = current.trim_start_matches("Pragma: ").to_string();
            } else if current.starts_with("Proxy-Authorization: ") {
                self.proxy_authorization = current
                    .trim_start_matches("Proxy-Authorization: ")
                    .to_string();
            } else if current.starts_with("Range: ") {
                self.range = current.trim_start_matches("Range: ").to_string();
            } else if current.starts_with("Referer: ") {
                self.referer = current.trim_start_matches("Referer: ").to_string();
            } else if current.starts_with("TE: ") {
                self.transfer_encodings = current.trim_start_matches("TE: ").to_string();
            } else if current.starts_with("User-Agent: ") {
                self.user_agent = current.trim_start_matches("User-Agent: ").to_string();
            } else if current.starts_with("Via: ") {
                self.via = current.trim_start_matches("Via: ").to_string();
            } else if current.starts_with("Warning: ") {
                self.warning = current.trim_start_matches("Warning: ").to_string();
            } else if current.starts_with("HTTP") {
                self.version = current;
            } else if current.starts_with("Upgrade-Insecure-Requests: ") {
                self.upgrade_insecure_requests = current
                    .trim_start_matches("Upgrade-Insecure-Requests: ")
                    .to_string();
            } else if current.starts_with("DNT: ") {
                self.dnt = current.trim_start_matches("DNT: ").to_string();
            } else {
                self.other.push(current);
            }
        }
    }
}
