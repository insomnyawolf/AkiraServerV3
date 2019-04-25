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

// https://en.wikipedia.org/wiki/List_of_HTTP_header_fields
#[derive(Debug, Default)]
pub struct ResponseHeaders {
    // Standard response fields
    /** Specifying which web sites can participate in cross-origin resource sharing ( * means any ) **/
    pub access_control_allow_origin: String,
    /** Specifies which patch document formats this server supports **/
    pub accept_patch: String,
    /** What partial content range types this server supports via byte serving **/
    pub accept_ranges: String,
    /** The age the object has been in a proxy cache in seconds **/
    pub age: String,
    /** Valid methods for a specified resource. To be used for a 405 Method not allowed **/
    pub allow: String,

    // Todo Omited Alt-Svc
    /** Tells all caching mechanisms from server to client whether they may cache this object. It is measured in seconds **/
    pub cache_control: String,
    /** Control options for the current connection and list of hop-by-hop response fields. **/
    pub connection: String,
    /** An opportunity to raise a "File Download" dialogue box for a known MIME type with binary format or suggest a filename for dynamic content.
    Quotes are necessary with special characters. **/
    pub content_disposition: String,
    /** The type of encoding used on the data. **/
    pub content_encoding: String,
    /** The natural language or languages of the intended audience for the enclosed content **/
    pub content_language: String,
    /** The length of the response body in octets (8-bit bytes) **/
    pub content_length: String,
    /** An alternate location for the returned data **/
    pub content_location: String,
    /** A Base64-encoded binary MD5 sum of the content of the response **/
    pub content_md5: String,
    /** Where in a full body message this partial message belongs **/
    pub content_range: String,
    /** The MIME type of this content **/
    pub content_type: String,
    /** The date and time that the message was sent **/
    pub date: String,
    /** Specifies the delta-encoding entity tag of the response **/
    pub delta_base: String,
    /** An identifier for a specific version of a resource, often a message digest **/
    pub e_tag: String,
    /** Gives the date/time after which the response is considered stale **/
    pub expires: String,
    /** Instance-manipulations applied to the response **/
    pub instance_manipulations: String,
    /** The last modified date for the requested object **/
    pub last_modified: String,
    /** Used to express a typed relationship with another resource **/
    pub link: String,
    /** Used in redirection, or when a new resource has been created. **/
    pub location: String,
    /** This field is supposed to set P3P policy, in the form of P3P:CP="your_compact_policy".
    However, P3P did not take off, most browsers have never fully implemented it,
    a lot of websites set this field with fake policy text,
    that was enough to fool browsers the existence of P3P policy and grant permissions for third party cookies. **/
    pub p3p: String,
    /** Implementation-specific fields that may have various effects anywhere along the request-response chain. **/
    pub pragma: String,
    /** Request authentication to access the proxy **/
    pub proxy_authenticate: String,
    /** HTTP Public Key Pinning, announces hash of website's authentic TLS certificate **/
    pub public_key_pins: String,
    /** If an entity is temporarily unavailable,
    this instructs the client to try again later.
    Value could be a specified period of time (in seconds) or a HTTP-date **/
    pub retry_after: String,
    /** A name for the server **/
    pub server: String,
    /** An HTTP cookie **/
    pub set_cookie: String,
    /** A HSTS Policy informing the HTTP client how long to cache the HTTPS only policy and whether this applies to subdomains **/
    pub strict_transport_security: String,
    /** The Trailer general field value indicates that the given set of header fields is present in the trailer of a message encoded with chunked transfer coding **/
    pub trailer: String,
    /** The form of encoding used to safely transfer the entity to the user. Currently defined methods are: chunked, compress, deflate, gzip, identity **/
    pub transfer_encoding: String,
    /** Tracking Status header, value suggested to be sent in response to a DNT(do-not-track), possible values:
    "!" — under construction
    "?" — dynamic
    "G" — gateway to multiple parties
    "N" — not tracking
    "T" — tracking
    "C" — tracking with consent
    "P" — tracking only if consented
    "D" — disregarding DNT
    "U" — updated **/
    pub tracking_status: String,
    /** Ask the client to upgrade to another protocol **/
    pub upgrade: String,
    /** Tells downstream proxies how to match future request headers to decide whether the cached response can be used rather than requesting a fresh one from the origin server **/
    pub vary: String,
    /** Informs the client of proxies through which the response was sent. **/
    pub via: String,
    /** A general warning about possible problems with the entity body. **/
    pub warning: String,
    /** Indicates the authentication scheme that should be used to access the requested entity. **/
    pub www_authenticate: String,
    /** Clickjacking protection:
    deny - no rendering within a frame,
    sameorigin - no rendering if origin mismatch,
    allow-from - allow from specified location,
    allowall - non-standard, allow from any location **/
    pub x_frame_options: String,
    // Common non-standard response fields
}
