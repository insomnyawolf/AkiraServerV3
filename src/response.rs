// Http Headers

// https://tools.ietf.org/html/rfc2616
// https://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html
#[allow(dead_code)] // Remooves unused code warnings on compile time
pub enum HttpStatus {
    // Informational 1xx
    Continue,           // 100
    SwitchingProtocols, // 101
    // Successful 2xx
    OK,                          // 200
    Created,                     // 201
    Accepted,                    // 202
    NonAuthoritativeInformation, // 203
    NoContent,                   // 204
    ResetContent,                // 205
    PartialContent,              // 206
    // Redirection 3xx
    MultipleChoices,   // 300
    MovedPermanently,  // 301
    Found,             // 302
    SeeOther,          // 303
    NotModified,       // 304
    UseProxy,          // 305
    Unused306,         // 306 used in previous version, reserved, no longer used.
    TemporaryRedirect, // 307
    // Client Error 4xx
    BadRequest,                   // 400
    Unauthorized,                 // 401
    PaymentRequired,              // 402
    Forbidden,                    // 403
    NotFound,                     // 404
    MethodNotAllowed,             // 405
    NotAcceptable,                // 406
    ProxyAuthenticationRequired,  // 407
    RequestTimeout,               // 408
    Conflict,                     // 409
    Gone,                         // 410
    LengthRequired,               // 411
    PreconditionFailed,           // 412
    RequestEntityTooLarge,        // 413
    RequestURITooLong,            // 414
    UnsupportedMediaType,         // 415
    RequestedRangeNotSatisfiable, // 416
    ExpectationFailed,            // 417
    // Server Error 5xx
    InternalServerError,     // 500
    NotImplemented,          // 501
    BadGateway,              // 502
    ServiceUnavailable,      // 503
    GatewayTimeout,          // 504
    HTTPVersionNotSupported, // 505
}

impl HttpStatus {
    /* ToDO Low Priority
    pub fn from_str(s: &[u8]) -> Option<MediaType> {
        match s {
            b"HTTP/1.1 200 OK\r\n\r\n" => Some(HTTP_STATUS::OK),
            _ => None,
        }
    }
    */

    pub fn as_bytes(&self) -> &[u8] {
        match *self {
            // Informational 1xx
            HttpStatus::Continue => b"HTTP/1.1 100 CONTINUE\r\n\r\n",
            HttpStatus::SwitchingProtocols => b"HTTP/1.1 101 SWITCHING PROTOCOLS\r\n\r\n",
            // Successful 2xx
            HttpStatus::OK => b"HTTP/1.1 200 OK\r\n\r\n",
            HttpStatus::Created => b"HTTP/1.1 201 CREATED\r\n\r\n",
            HttpStatus::Accepted => b"HTTP/1.1 202 ACCEPTED\r\n\r\n",
            HttpStatus::NonAuthoritativeInformation => {
                b"HTTP/1.1 203 NON AUTHORITATIVE INFORMATION\r\n\r\n"
            }
            HttpStatus::NoContent => b"HTTP/1.1 204 NO CONTENT\r\n\r\n",
            HttpStatus::ResetContent => b"HTTP/1.1 205 RESET CONTENT\r\n\r\n",
            HttpStatus::PartialContent => b"HTTP/1.1 206 PARTIAL CONTENT\r\n\r\n",
            // Redirection 3xx
            HttpStatus::MultipleChoices => b"HTTP/1.1 300 MULTIPLE CHOICES\r\n\r\n",
            HttpStatus::MovedPermanently => b"HTTP/1.1 301 MOVED PERMANENTLY\r\n\r\n",
            HttpStatus::Found => b"HTTP/1.1 302 FOUND\r\n\r\n",
            HttpStatus::SeeOther => b"HTTP/1.1 303 SEE OTHER\r\n\r\n",
            HttpStatus::NotModified => b"HTTP/1.1 304 NOT MODIFIED\r\n\r\n",
            HttpStatus::UseProxy => b"HTTP/1.1 305 USE PROXY\r\n\r\n",
            HttpStatus::Unused306 => b"HTTP/1.1 306 \r\n\r\n",
            HttpStatus::TemporaryRedirect => b"HTTP/1.1 307 TEMPORARY REDIRECT\r\n\r\n",
            // Client Error 4xx
            HttpStatus::BadRequest => b"HTTP/1.1 400 BAD REQUEST\r\n\r\n",
            HttpStatus::Unauthorized => b"HTTP/1.1 401 UNAUTHORIZED\r\n\r\n",
            HttpStatus::PaymentRequired => b"HTTP/1.1 402 PAYMENT REQUIERED\r\n\r\n",
            HttpStatus::Forbidden => b"HTTP/1.1 403 FORBIDDEN\r\n\r\n",
            HttpStatus::NotFound => b"HTTP/1.1 404 NOT FOUND\r\n\r\n",
            HttpStatus::MethodNotAllowed => b"HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n",
            HttpStatus::NotAcceptable => b"HTTP/1.1 406 NOT ACCEPTABLE\r\n\r\n",
            HttpStatus::ProxyAuthenticationRequired => {
                b"HTTP/1.1 407 PROXY AUTHENTICATION REQUIERED\r\n\r\n"
            }
            HttpStatus::RequestTimeout => b"HTTP/1.1 408 REQUEST TIMEOUT\r\n\r\n",
            HttpStatus::Conflict => b"HTTP/1.1 409 CONFLICT\r\n\r\n",
            HttpStatus::Gone => b"HTTP/1.1 410 GONE\r\n\r\n",
            HttpStatus::LengthRequired => b"HTTP/1.1 411 LENGHT REQUIRED\r\n\r\n",
            HttpStatus::PreconditionFailed => b"HTTP/1.1 412 PRECONDITION FAILED\r\n\r\n",
            HttpStatus::RequestEntityTooLarge => b"HTTP/1.1 413 REQUEST ENTITY TOO LARGE\r\n\r\n",
            HttpStatus::RequestURITooLong => b"HTTP/1.1 414 REQUEST URI TOO LONG\r\n\r\n",
            HttpStatus::UnsupportedMediaType => b"HTTP/1.1 415 UNSUPPORTED MEDIA TYPE\r\n\r\n",
            HttpStatus::RequestedRangeNotSatisfiable => {
                b"HTTP/1.1 416 REQUEST RANGE NOT SATISFIABLE\r\n\r\n"
            }
            HttpStatus::ExpectationFailed => b"HTTP/1.1 417 EXPECTATION FAILED\r\n\r\n",
            // Server Error 5xx
            HttpStatus::InternalServerError => b"HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n",
            HttpStatus::NotImplemented => b"HTTP/1.1 500 NOT IMPLEMENTED\r\n\r\n",
            HttpStatus::BadGateway => b"HTTP/1.1 500 BAD GATEWAY\r\n\r\n",
            HttpStatus::ServiceUnavailable => b"HTTP/1.1 500 SERVICE UNAVAILABLE\r\n\r\n",
            HttpStatus::GatewayTimeout => b"HTTP/1.1 500 GATEWAY TIMEOUT\r\n\r\n",
            HttpStatus::HTTPVersionNotSupported => {
                b"HTTP/1.1 500 HTTP VERSION NOT SUPPORTED\r\n\r\n"
            }
        }
    }
}
