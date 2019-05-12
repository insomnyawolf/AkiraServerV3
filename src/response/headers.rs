// Http Headers
// https://tools.ietf.org/html/rfc2616
// https://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html
use crate::response::status::*;

// https://en.wikipedia.org/wiki/List_of_HTTP_header_fields
#[derive(Debug, Default)]
/// Structure That can be used to easily add response headers to any http response
// ToDo Complete code for all fields
pub struct ResponseHeaders {
    /// Status Header
    pub status: HttpStatus,
    // Standard response fields
    /// Specifying which web sites can participate in cross-origin resource sharing ( * means any )
    pub access_control_allow_origin: String,
    /// Specifies which patch document formats this server supports
    pub accept_patch: String,
    /// What partial content range types this server supports via byte serving
    pub accept_ranges: String,
    /// The age the object has been in a proxy cache in seconds
    pub age: String,
    /// Valid methods for a specified resource. To be used for a 405 Method not allowed
    pub allow: String,
    /// Tells all caching mechanisms from server to client whether they may cache this object. It is measured in seconds
    pub cache_control: String,
    /// Control options for the current connection and list of hop-by-hop response fields.
    pub connection: String,
    /** An opportunity to raise a "File Download" dialogue box for a known MIME type with binary format or suggest a filename for dynamic content.
    Quotes are necessary with special characters. **/
    pub content_disposition: String,
    /// The type of encoding used on the data.
    pub content_encoding: String,
    /// The natural language or languages of the intended audience for the enclosed content
    pub content_language: String,
    /// The length of the response body in octets (8-bit bytes)
    pub content_length: u64,
    /// An alternate location for the returned data
    pub content_location: String,
    /// A Base64-encoded binary MD5 sum of the content of the response
    pub content_md5: String,
    /// Where in a full body message this partial message belongs
    pub content_range: String,
    /// The MIME type of this content
    pub content_type: String,
    /// The date and time that the message was sent
    pub date: String,
    /// Specifies the delta-encoding entity tag of the response
    pub delta_base: String,
    /// An identifier for a specific version of a resource, often a message digest
    pub e_tag: String,
    /// Gives the date/time after which the response is considered stale
    pub expires: String,
    /// Instance-manipulations applied to the response
    pub instance_manipulations: String,
    /// The last modified date for the requested object
    pub last_modified: String,
    /// Used to express a typed relationship with another resource
    pub link: String,
    /// Used in redirection, or when a new resource has been created.
    pub location: String,
    /** This field is supposed to set P3P policy, in the form of P3P:CP="your_compact_policy".
    However, P3P did not take off, most browsers have never fully implemented it,
    a lot of websites set this field with fake policy text,
    that was enough to fool browsers the existence of P3P policy and grant permissions for third party cookies. **/
    pub p3p: String,
    /// Implementation-specific fields that may have various effects anywhere along the request-response chain.
    pub pragma: String,
    /// Request authentication to access the proxy
    pub proxy_authenticate: String,
    /// HTTP Public Key Pinning, announces hash of website's authentic TLS certificate
    pub public_key_pins: String,
    /** If an entity is temporarily unavailable,
    this instructs the client to try again later.
    Value could be a specified period of time (in seconds) or a HTTP-date **/
    pub retry_after: String,
    /// A name for the server
    pub server: String,
    /// An HTTP cookie
    pub set_cookie: String,
    /// A HSTS Policy informing the HTTP client how long to cache the HTTPS only policy and whether this applies to subdomains
    pub strict_transport_security: String,
    /// The Trailer general field value indicates that the given set of header fields is present in the trailer of a message encoded with chunked transfer coding
    pub trailer: String,
    /// The form of encoding used to safely transfer the entity to the user. Currently defined methods are: chunked, compress, deflate, gzip, identity
    pub transfer_encoding: TransferEncoding,
    /// Tracking Status header, value suggested to be sent in response to a DNT(do-not-track)
    pub tracking_status: String,
    /// Ask the client to upgrade to another protocol
    pub upgrade: String,
    /// Tells downstream proxies how to match future request headers to decide whether the cached response can be used rather than requesting a fresh one from the origin server
    pub vary: String,
    /// Informs the client of proxies through which the response was sent.
    pub via: String,
    /// A general warning about possible problems with the entity body.
    pub warning: String,
    /// Indicates the authentication scheme that should be used to access the requested entity.
    pub www_authenticate: String,
    /** Clickjacking protection:
    deny - no rendering within a frame,
    sameorigin - no rendering if origin mismatch,
    allow-from - allow from specified location,
    allowall - non-standard, allow from any location **/
    pub x_frame_options: String,
    // Common non-standard response fields
}

#[allow(dead_code)]
impl ResponseHeaders {
    pub fn new(status: HttpStatus) -> ResponseHeaders {
        let mut response = ResponseHeaders::default();
        response.status = status;
        response
    }
    pub fn set_cross_origin_allow_all(&mut self) {
        self.access_control_allow_origin = "*".to_string();
    }
    pub fn set_cross_origin_allow_host(&mut self, host: String) {
        self.access_control_allow_origin = host;
    }
    pub fn set_content_length(&mut self, content_lenght: u64) {
        self.content_length = content_lenght;
    }
    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = content_type;
    }
    pub fn get_headers(&mut self) -> String {
        let mut headers: Vec<u8> = Vec::new();
        headers.extend_from_slice(&self.status.as_bytes());
        // Cors
        if self.access_control_allow_origin != "" {
            let s = format!(
                "Access-Control-Allow-Origin: {}\r\n",
                &self.access_control_allow_origin
            );
            headers.extend_from_slice(s.as_bytes());
        }
        // Content Lenght
        if self.content_length != 0 {
            let s = format!("Content-Length: {}\r\n", &self.content_length);
            headers.extend_from_slice(s.as_bytes());
        }
        headers.extend_from_slice(b"\r\n");
        String::from_utf8_lossy(headers.as_slice()).to_string()
    }
}

#[allow(dead_code)] // Remove unused code warnings on compile time
#[derive(Debug)]
pub enum TransferEncoding {
    /// ""
    Undefined,
    /// "!"
    UnderConstruction,
    /// "?"
    Dynamic,
    /// "G"
    GatewayToMultipleParties,
    /// "N"
    NotTracking,
    /// "T"
    Tracking,
    /// "C"
    TrackingWithConsent,
    /// "P"
    TrackingOnlyIfConsented,
    /// "D"
    DisregardingDnt,
    /// "U"
    Updated,
}

impl Default for TransferEncoding {
    fn default() -> TransferEncoding {
        TransferEncoding::Undefined
    }
}
