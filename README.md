# AkiraServerV3
( Experimental )
## WebServer Project
Reimplementacion de servidor web por 3º vez , esta vez en rust!
### ToDo
* Update This Documment
* Chunkear los archivos Enviados
* Config
   * CORS

### Hecho
* Threadpool para requests
* Configuraciones desde archivo
* Functional base
    * Enviar archivos solicitados
* Config
    * Puerto
    * Listado de archivos
    * 404 Por defecto
    * Rewrites ?
* Paginas auto generadas  
    *Directory listing
* Headers  
    *Content Type  
    *Content Size  
### Mejoras Pendientes  
* Http Request Client -> Valor sin cabecera
* Escuchar cambios en el archivo de configuracion
### Bug

## Docs

### Tecnologias

#### Rust

[Rust](https://www.rust-lang.org/)  
[Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

#### Protocolo http

[Mozilla MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview)


### Librerias Usadas

#### Sample Name

### Funcionamiento
Se basa en las librerias:
* std::net::TcpListener;
* std::net::TcpStream;  

Con ellas creamos un servicio que escucha en una direccion y puerto determinados, 
cuando se recive una conexion, 
esta se pasa a un hilo secundario para que el programa pueda seguir atendiendo peticiones.  

En este, lo primero que se realiza es un parseo de la request http obteniendo 2 structs,
los cuales nos proporcionaran toda la informacion necesaria para procesar la peticion.

Request:

```rust
#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Default)]
pub struct Request {
    #[derivative(Debug = "ignore")]
    raw: String,
    pub is_valid_request: bool,
    pub method: Method,
    pub path: String,
    pub request_headers: RequestHeaders,
}
```

```rust
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
```


WIP ↓
```rust
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
```
