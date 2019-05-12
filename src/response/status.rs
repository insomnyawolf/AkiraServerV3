#[derive(Debug)]
#[allow(dead_code)] // Removes unused code warnings on compile time
/// Enum with most common HttpStatus Possible codes
pub enum HttpStatus {
    /// Dont use, Parse Fallback
    Undefined,
    // Informational 1xx
    /// 100
    Continue,
    /// 101
    SwitchingProtocols,
    // Successful 2xx
    /// 200
    OK,
    /// 201
    Created,
    /// 202
    Accepted,
    /// 203
    NonAuthoritativeInformation,
    /// 204
    NoContent,
    /// 205
    ResetContent,
    /// 206
    PartialContent,
    // Redirection 3xx
    /// 300
    MultipleChoices,
    /// 301
    MovedPermanently,
    /// 302
    Found,
    /// 303
    SeeOther,
    /// 304
    NotModified,
    /// 305
    UseProxy,
    /// 306 used in previous version, reserved, no longer used.
    Unused306,
    /// 307
    TemporaryRedirect,
    // Client Error 4xx
    /// 400
    BadRequest,
    /// 401
    Unauthorized,
    /// 402
    PaymentRequired,
    /// 403
    Forbidden,
    /// 404
    NotFound,
    /// 405
    MethodNotAllowed,
    /// 406
    NotAcceptable,
    /// 407
    ProxyAuthenticationRequired,
    /// 408
    RequestTimeout,
    /// 409
    Conflict,
    /// 410
    Gone,
    /// 411
    LengthRequired,
    /// 412
    PreconditionFailed,
    /// 413
    RequestEntityTooLarge,
    /// 414
    RequestURITooLong,
    /// 415
    UnsupportedMediaType,
    /// 416
    RequestedRangeNotSatisfiable,
    /// 417
    ExpectationFailed,
    // Server Error 5xx
    /// 500
    InternalServerError,
    /// 501
    NotImplemented,
    /// 502
    BadGateway,
    /// 503
    ServiceUnavailable,
    /// 504
    GatewayTimeout,
    /// 505
    HTTPVersionNotSupported,
}

impl Default for HttpStatus {
    /// Default value for the enum
    fn default() -> HttpStatus {
        HttpStatus::Undefined
    }
}

impl HttpStatus {
    /* ToDO Low Priority Parse HttpStatus From String
    pub fn from_str(s: &[u8]) -> Option<MediaType> {
        match s {
            b"HTTP/1.1 200 OK\r\n" => Some(HTTP_STATUS::OK),
            _ => None,
        }
    }
    */

    /// Convert HttpStatus to bytes that can be sent with the response
    pub fn as_bytes(&self) -> &[u8] {
        match *self {
            HttpStatus::Undefined => b"",
            // Informational 1xx
            HttpStatus::Continue => b"HTTP/1.1 100 CONTINUE\r\n",
            HttpStatus::SwitchingProtocols => b"HTTP/1.1 101 SWITCHING PROTOCOLS\r\n",
            // Successful 2xx
            HttpStatus::OK => b"HTTP/1.1 200 OK\r\n",
            HttpStatus::Created => b"HTTP/1.1 201 CREATED\r\n",
            HttpStatus::Accepted => b"HTTP/1.1 202 ACCEPTED\r\n",
            HttpStatus::NonAuthoritativeInformation => {
                b"HTTP/1.1 203 NON AUTHORITATIVE INFORMATION\r\n"
            }
            HttpStatus::NoContent => b"HTTP/1.1 204 NO CONTENT\r\n",
            HttpStatus::ResetContent => b"HTTP/1.1 205 RESET CONTENT\r\n",
            HttpStatus::PartialContent => b"HTTP/1.1 206 PARTIAL CONTENT\r\n",
            // Redirection 3xx
            HttpStatus::MultipleChoices => b"HTTP/1.1 300 MULTIPLE CHOICES\r\n",
            HttpStatus::MovedPermanently => b"HTTP/1.1 301 MOVED PERMANENTLY\r\n",
            HttpStatus::Found => b"HTTP/1.1 302 FOUND\r\n",
            HttpStatus::SeeOther => b"HTTP/1.1 303 SEE OTHER\r\n",
            HttpStatus::NotModified => b"HTTP/1.1 304 NOT MODIFIED\r\n",
            HttpStatus::UseProxy => b"HTTP/1.1 305 USE PROXY\r\n",
            HttpStatus::Unused306 => b"HTTP/1.1 306 \r\n",
            HttpStatus::TemporaryRedirect => b"HTTP/1.1 307 TEMPORARY REDIRECT\r\n",
            // Client Error 4xx
            HttpStatus::BadRequest => b"HTTP/1.1 400 BAD REQUEST\r\n",
            HttpStatus::Unauthorized => b"HTTP/1.1 401 UNAUTHORIZED\r\n",
            HttpStatus::PaymentRequired => b"HTTP/1.1 402 PAYMENT REQUIERED\r\n",
            HttpStatus::Forbidden => b"HTTP/1.1 403 FORBIDDEN\r\n",
            HttpStatus::NotFound => b"HTTP/1.1 404 NOT FOUND\r\n",
            HttpStatus::MethodNotAllowed => b"HTTP/1.1 405 METHOD NOT ALLOWED\r\n",
            HttpStatus::NotAcceptable => b"HTTP/1.1 406 NOT ACCEPTABLE\r\n",
            HttpStatus::ProxyAuthenticationRequired => {
                b"HTTP/1.1 407 PROXY AUTHENTICATION REQUIERED\r\n"
            }
            HttpStatus::RequestTimeout => b"HTTP/1.1 408 REQUEST TIMEOUT\r\n",
            HttpStatus::Conflict => b"HTTP/1.1 409 CONFLICT\r\n",
            HttpStatus::Gone => b"HTTP/1.1 410 GONE\r\n",
            HttpStatus::LengthRequired => b"HTTP/1.1 411 LENGHT REQUIRED\r\n",
            HttpStatus::PreconditionFailed => b"HTTP/1.1 412 PRECONDITION FAILED\r\n",
            HttpStatus::RequestEntityTooLarge => b"HTTP/1.1 413 REQUEST ENTITY TOO LARGE\r\n",
            HttpStatus::RequestURITooLong => b"HTTP/1.1 414 REQUEST URI TOO LONG\r\n",
            HttpStatus::UnsupportedMediaType => b"HTTP/1.1 415 UNSUPPORTED MEDIA TYPE\r\n",
            HttpStatus::RequestedRangeNotSatisfiable => {
                b"HTTP/1.1 416 REQUEST RANGE NOT SATISFIABLE\r\n"
            }
            HttpStatus::ExpectationFailed => b"HTTP/1.1 417 EXPECTATION FAILED\r\n",
            // Server Error 5xx
            HttpStatus::InternalServerError => b"HTTP/1.1 500 INTERNAL SERVER ERROR\r\n",
            HttpStatus::NotImplemented => b"HTTP/1.1 501 NOT IMPLEMENTED\r\n",
            HttpStatus::BadGateway => b"HTTP/1.1 502 BAD GATEWAY\r\n",
            HttpStatus::ServiceUnavailable => b"HTTP/1.1 503 SERVICE UNAVAILABLE\r\n",
            HttpStatus::GatewayTimeout => b"HTTP/1.1 504 GATEWAY TIMEOUT\r\n",
            HttpStatus::HTTPVersionNotSupported => b"HTTP/1.1 505 HTTP VERSION NOT SUPPORTED\r\n",
        }
    }
}
