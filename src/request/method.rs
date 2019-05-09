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
            _ => Some(Method::Unsupported),
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