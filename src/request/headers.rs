use crate::request::utils::*;

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
    /// Used to specify directives that must be obeyed by all caching mechanisms along the request-response chain
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
    /// The domain name of the server (for virtual hosting), and the TCP port number on which the server is listening
    pub host: String,
    /// Limit the number of times the message can be forwarded through proxies or gateways
    pub max_forwards: String,
    /// Initiates a request for cross-origin resource sharing (asks server for Access-Control-* response fields)
    pub origin: String,
    /// Implementation-specific fields that may have various effects anywhere along the request-response chain
    pub pragma: String,
    /// Authorization credentials for connecting to a proxy
    pub proxy_authorization: String,
    /// Request only part of an entity. Bytes are numbered from 0
    pub range: String,
    /// This is the address of the previous web page from which a link to the currently requested page was followed (misspelled in the RFC)
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
    /// Tells a server which (presumably in the middle of a HTTP -> HTTPS migration) hosts mixed content that the client would prefer redirection to HTTPS
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
            if generate_field_string(
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
                let t: Vec<&str> = current.split("; ").collect();
                if t.len() > 1 {
                    let z = t[1];
                    let bound_str = "boundary=";
                    if z.starts_with(bound_str) {
                        let bond_str_len = bound_str.len();
                        let bounds = z[bond_str_len..].to_string();
                        headers.content_type =
                            headers.content_type.trim_end_matches(&bounds).to_string();
                        headers.content_type = headers
                            .content_type
                            .trim_end_matches("; boundary=")
                            .to_string();
                        headers.content_bounds = bounds;
                    }
                }
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
