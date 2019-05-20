// https://tools.ietf.org/html/rfc2616#section-5.1.1

#[derive(Debug, PartialEq)]

/// Contains references to all possible request methods  
/// Can be parsed from string  
/// Can be converted to string  
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
    /// Defines default value for the method enum, wich is "Unsupported"
    fn default() -> Method {
        Method::Unsupported
    }
}

impl Method {
    /// Parse Method from String
    pub fn from_str(s: &String) -> Method {
        let string: &str = &s[..];
        match string {
            "CONNECT" => Method::CONNECT,
            "DELETE" => Method::DELETE,
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "TRACE" => Method::TRACE,
            _ => Method::Unsupported,
        }
    }
    ///  Converts the given value to a String
    pub fn to_str(&self) -> &str {
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
