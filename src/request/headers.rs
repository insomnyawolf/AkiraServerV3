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
    let header: Vec<&str> = current.rsplit(": ").collect();

    if header.len() == 2 {
        match header[0] {
            "A-IM" => {
                headers.acceptable_instance_manipulations = generate_field_string(&header[1]);
            },
            "Accept" => {
                //values = &values.replace(";", ",")[..];
                let arr: Vec<&str> = header[1].split(";").collect();

                for data in arr {
                    headers.accept.push(data.to_string());
                }
            },
            "Accept-Charset" => {
                headers.accept_charset = generate_field_string(&header[1]);
            },
            "Accept-Encoding" => {
                headers.accept_encoding = generate_field_string_vec(&header[1]);
            },
            "Accept-Language" => {
                headers.accept_language = generate_field_string(&header[1]);
            },
            "Accept-Datetime" => {
                headers.accept_datetime = generate_field_string(&header[1]);
            },
            "Access-Control-Request-Method" => {
                headers.access_control_request_method = generate_field_string(&header[1]);
            },
            "Authorization" => {
                headers.authorization = generate_field_string(&header[1]);
            },

            "Cache-Control" => {
                headers.cache_control = generate_field_string(&header[1]);
            },
            "Connection" => {
                headers.connection = generate_field_string(&header[1]);
            },
            "Content-Length" => {
                headers.content_length = generate_field_u64(&header[1]);
            },
            "Content-MD5" => {
                headers.content_md5 = generate_field_string(&header[1]);
            },
            "Content-Type" => {
                headers.content_type = generate_field_string(&header[1]);
                let t: Vec<&str> = current.split("; ").collect();
                if t.len() > 1 {
                    let z = t[1];
                    let bound_str = "boundary=";
                    if z.starts_with(bound_str) {
                        let bond_str_len = bound_str.len();
                        let bounds = &z[bond_str_len..];
                        headers.content_type = headers.content_type.trim_end_matches(&bounds).to_string();
                        headers.content_type = headers.content_type.trim_end_matches("; boundary=").to_string();
                        headers.content_bounds = bounds.to_string();
                    }
                }
            },
            "Cookie" => {
                headers.cookie = generate_field_string(&header[1]);
            },
            "Date" => {
                headers.date = generate_field_string(&header[1]);
            },
            "Expect" => {
                headers.expect = generate_field_string(&header[1]);
            },
            "Forwarded" => {
                headers.forwarded = generate_field_string(&header[1]);
            },
            "From" => {
                headers.from = generate_field_string(&header[1]);
            },
            "Host" => {
                headers.host = generate_field_string(&header[1]);
            },
            "Max-Forwards" => {
                headers.max_forwards = generate_field_string(&header[1]);
            },
            "Origin" => {
                headers.origin = generate_field_string(&header[1]);
            },
            "Pragma" => {
                headers.pragma = generate_field_string(&header[1]);
            },
            "Proxy-Authorization" => {
                headers.proxy_authorization = generate_field_string(&header[1]);
            },
            "Range" => {
                headers.range = generate_field_string(&header[1]);
            },
            "Referer" => {
                headers.referer = generate_field_string(&header[1]);
            },
            "TE" => {
                headers.transfer_encodings = generate_field_string(&header[1]);
            },
            "User-Agent" => {
                headers.user_agent = generate_field_string(&header[1]);
            },
            "Via" => {
                headers.via = generate_field_string(&header[1]);
            },
            "Warning" => {
                headers.warning = generate_field_string(&header[1]);
            },
            "Upgrade-Insecure-Requests" => {
                headers.upgrade_insecure_requests = generate_field_string(&header[1]);
            },
            "DNT" => {
                headers.dnt = generate_field_string(&header[1]);
            },
            _ => {
                headers.other.push(current.to_string());
            },
        }
    } else {
        if start_with(&current, "HTTP/") {
            headers.version = generate_field_string(current);
            return;
        }
        headers.other.push(current.to_string());
    }
}
