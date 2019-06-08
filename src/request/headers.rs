use crate::request::utils::*;
use std::env::current_exe;

// https://en.wikipedia.org/wiki/List_of_HTTP_header_fields

#[derive(Debug, Default)]
/// Struct that have all the request headers
pub struct RequestHeaders {
    // Standard request fields
    /// Acceptable instance-manipulations for the request
    pub acceptable_instance_manipulations: String,
    /// Media type(s) that is/are acceptable for the response
    pub accept: Vec<String>,
    /// Character sets that are acceptable
    pub accept_charset: String,
    /// List of acceptable encodings
    pub accept_encoding: Vec<String>,
    /// List of acceptable human languages for response
    pub accept_language: String,
    /// Acceptable version in time
    pub accept_datetime: String,
    /// Initiates a request for cross-origin resource sharing with Origin
    pub access_control_request_method: String,
    /// Authentication credentials for HTTP authentication
    pub authorization: String,
    /// Used to specify directives that must be obeyed by all caching mechanisms along the
    /// request-response chain
    pub cache_control: String,
    /// Control options for the current connection and list of hop-by-hop request fields
    pub connection: String,
    /// The length of the request body in octets (8-bit bytes)
    pub content_length: u64,
    /// A Base64-encoded binary MD5 sum of the content of the request body
    pub content_md5: String,
    /// The Media type of the body of the request (used with POST and PUT requests)
    pub content_type: String,
    /// Multipart Forms Bounds
    pub content_bounds: String,
    /// An HTTP cookie previously sent by the server with Set-Cookie (below)
    pub cookie: String,
    /// The date and time at which the message was originated
    pub date: String,
    /// Indicates that particular server behaviors are required by the client
    pub expect: String,
    /// Disclose original information of a client connecting to a web server through an HTTP proxy
    pub forwarded: String,
    /// The email address of the user making the request
    pub from: String,
    /// The domain name of the server (for virtual hosting), and the TCP port number on which the
    /// server is listening
    pub host: String,
    /// Limit the number of times the message can be forwarded through proxies or gateways
    pub max_forwards: String,
    /// Initiates a request for cross-origin resource sharing (asks server for Access-Control-*
    /// response fields)
    pub origin: String,
    /// Implementation-specific fields that may have various effects anywhere along the
    /// request-response chain
    pub pragma: String,
    /// Authorization credentials for connecting to a proxy
    pub proxy_authorization: String,
    /// Request only part of an entity. Bytes are numbered from 0
    pub range: String,
    /// This is the address of the previous web page from which a link to the currently requested
    /// page was followed (misspelled in the RFC)
    pub referer: String,
    /** The transfer encodings the user agent is willing to accept:
    the same values as for the response header field Transfer-Encoding can be used,
    plus the "trailers" value (related to the "chunked" transfer method)
    to notify the server it expects to receive additional fields in the trailer after the last, zero-sized, chunk **/
    pub transfer_encodings: String,
    /// user agent string **/
    pub user_agent: String,

    // ToDo Omitted Upgrade
    /// Informs the server of proxies through which the request was sent
    pub via: String,
    /// A general warning about possible problems with the entity body
    pub warning: String,

    // End Of Std Fields

    // Common non-standard request fields
    /// HTTP Protocol used version
    pub version: String,
    /// Tells a server which (presumably in the middle of a HTTP -> HTTPS migration) hosts mixed
    /// content that the client would prefer redirection to HTTPS
    pub upgrade_insecure_requests: String,
    /// Requests a web application to disable their tracking of a user **/
    pub dnt: String,

    /// Undefined headers
    pub other: Vec<String>,
}

impl RequestHeaders {
    /// Parse Headers From Raw Request String
    pub fn parse(raw: &str) -> RequestHeaders {
        let mut headers = RequestHeaders::default();

        let client_arr: Vec<&str> = raw.rsplit("\r\n").collect();

        // ToDo Improove this loop
        for current in client_arr {
            parse_header(&mut headers, current);
        }

        headers
    }
}

fn parse_header(headers: &mut RequestHeaders, current: &str) {
    let header: Vec<&str> = current.rsplit("\r\n").collect();

    let mut pattern = "A-IM: ";
    if start_with(&current, &pattern) {
        headers.acceptable_instance_manipulations = generate_field_string(&current, &pattern);
        return;
    }
    pattern = "Accept: ";
    if start_with(&current, &pattern) {
        // Todo Check This
        let values = current.trim_start_matches(pattern);
        //values = &values.replace(";", ",")[..];
        let arr: Vec<&str> = values.split(";").collect();

        for data in arr {
            headers.accept.push(Cow::Owned(data.to_string()));
        }
        return;
    }
    pattern = "Accept-Charset: ";
    if start_with(&current, &pattern) {
        headers.accept_charset = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Accept-Encoding: ";
    if start_with(&current, &pattern) {
        headers.accept_encoding = generate_field_string_vec(&current, pattern);
        return;
    }
    pattern = "Accept-Language: ";
    if start_with(&current, &pattern) {
        headers.accept_language = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Accept-Datetime: ";
    if start_with(&current, &pattern) {
        headers.accept_datetime = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Access-Control-Request-Method: ";
    if start_with(&current, &pattern) {
        headers.access_control_request_method = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Authorization: ";
    if start_with(&current, &pattern) {
        headers.authorization = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Cache-Control: ";
    if start_with(&current, &pattern) {
        headers.cache_control = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Connection: ";
    if start_with(&current, &pattern) {
        headers.connection = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Content-Length: ";
    if start_with(&current, &pattern) {
        headers.content_length = generate_field_u64(&current, pattern);
        return;
    }
    pattern = "Content-MD5: ";
    if start_with(&current, &pattern) {
        headers.content_md5 = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Content-Type: ";
    if start_with(&current, &pattern) {
        headers.content_type = generate_field_string(&current, pattern);
        let t: Vec<&str> = current.split("; ").collect();
        if t.len() > 1 {
            let z = t[1];
            let bound_str = "boundary=";
            if z.starts_with(bound_str) {
                let bond_str_len = bound_str.len();
                let bounds = &z[bond_str_len..];
                headers.content_type = Cow::Owned(headers.content_type.trim_end_matches(&bounds).to_string());
                headers.content_type = Cow::Owned(headers.content_type.trim_end_matches("; boundary=").to_string());
                headers.content_bounds = Cow::Owned(bounds.to_string());
            }
        }
        return;
    }
    pattern = "Cookie: ";
    if start_with(&current, &pattern) {
        headers.cookie = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Date: ";
    if start_with(&current, &pattern) {
        headers.date = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Expect: ";
    if start_with(&current, &pattern) {
        headers.expect = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Forwarded: ";
    if start_with(&current, &pattern) {
        headers.forwarded = generate_field_string(&current, pattern);
        return;
    }
    pattern = "From: ";
    if start_with(&current, &pattern) {
        headers.from = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Host: ";
    if start_with(&current, &pattern) {
        headers.host = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Max-Forwards: ";
    if start_with(&current, &pattern) {
        headers.max_forwards = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Origin: ";
    if start_with(&current, &pattern) {
        headers.origin = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Pragma: ";
    if start_with(&current, &pattern) {
        headers.pragma = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Proxy-Authorization: ";
    if start_with(&current, &pattern) {
        headers.proxy_authorization = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Range: ";
    if start_with(&current, &pattern) {
        headers.range = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Referer: ";
    if start_with(&current, &pattern) {
        headers.referer = generate_field_string(&current, pattern);
        return;
    }
    pattern = "TE: ";
    if start_with(&current, &pattern) {
        headers.transfer_encodings = generate_field_string(&current, pattern);
        return;
    }
    pattern = "User-Agent: ";
    if start_with(&current, &pattern) {
        headers.user_agent = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Via: ";
    if start_with(&current, &pattern) {
        headers.via = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Warning: ";
    if start_with(&current, &pattern) {
        headers.warning = generate_field_string(&current, pattern);
        return;
    }
    pattern = "HTTP/";
    if start_with(&current, &pattern) {
        headers.version = generate_field_string(&current, pattern);
        return;
    }
    pattern = "Upgrade-Insecure-Requests: ";
    if start_with(&current, &pattern) {
        headers.upgrade_insecure_requests = generate_field_string(&current, pattern);
        return;
    }
    pattern = "DNT: ";
    if start_with(&current, &pattern) {
        headers.dnt = generate_field_string(&current, pattern);
        return;
    }
    headers.other.push(Cow::Owned(current.to_string()));
}
