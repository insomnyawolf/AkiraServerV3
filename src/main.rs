#![feature(proc_macro_hygiene)]  // Enables procedural macros as expresions
// Url
extern crate percent_encoding;
// Time
extern crate chrono;
// Config
extern crate config;
// Parser
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derivative;
extern crate maud;
use maud::*;

use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

use num_cpus;
use threadpool::ThreadPool;

use std::any::Any;
use std::time::Duration;

// Util
mod utils;

// For Config
mod settings;
use crate::settings::settings::Settings;

// Request Module
mod request;

use crate::request::method::Method;
use crate::request::request::Request;

// Response
mod response;
use crate::response::headers::ResponseHeaders;
use crate::response::status::HttpStatus;
use termcolor::Color;

// Resources
const BOOTSTRAP_CSS:&'static str = include_str!("../resources/bootstrap.css");
// const JQUERY_JS:&'static str = include_str!("../resources/jquery-3.4.1.js");

lazy_static! {
    static ref APP_CONFIG: Settings = Settings::new().unwrap();
    static ref SERVER_ROOT: String = add_string(&APP_CONFIG.server.root_folder, "/".to_string());
}

fn main() {
    APP_CONFIG.show();
    server();
}

fn server() {
    // Obtiene numero procesadores logicos
    let core_count = num_cpus::get();
    // Calcula trabajos por procasador logico
    let n_workers = core_count * APP_CONFIG.server.workers_per_thread;
    // Inicia piscina de trabajos limitada
    let pool = ThreadPool::new(n_workers);
    // Bind de la direccion tcp
    let listener = TcpListener::bind(format!(
        "{host}:{port}",
        host = APP_CONFIG.server.host,
        port = APP_CONFIG.server.port
    ))
    .unwrap();

    listener
        .set_ttl(APP_CONFIG.server.ttl)
        .expect("could not set TTL");

    // Bucle para cada peticion tcp
    for stream in listener.incoming() {
        // Canal de datos tcp
        let stream = stream.unwrap();
        // Inicia el trabajo en otro hilo su hay tareas disponibles, ni no, espera a que alguna finalize
        pool.execute(move || {
            // Hace cosas
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Create a Duration ans set is as timeout
    // That way the server doesnt keep waiting for more bytes
    let timeout = Some(Duration::new(0, APP_CONFIG.timeout.get_nanoseconds()));
    // Get a copy of TcpStream
    //Parse request data
    let request = Request::parse(stream.try_clone().unwrap(), timeout);

    if request.is_valid_request {
        log(&request, Color::Cyan);

        // Switch Equivalent
        match request.method {
            Method::GET => {
                handle_get(&stream, &request);
            }
            _ => {
                handle_unsupported(&stream);
            }
        }
    } else {
        log(&"Invalid Request", Color::Red);
    }

    // Avoid Dead Connections?
    stream.flush().ok().unwrap();
    stream.shutdown(Shutdown::Both).ok().unwrap();
}

fn handle_unsupported(mut stream: &TcpStream) {
    log(&"Unsupported Method", Color::Red);
    let mut headers = ResponseHeaders::new(HttpStatus::NotImplemented);
    stream.write(&headers.get_headers().as_bytes()).unwrap();
}

fn handle_get(mut stream: &TcpStream, request: &Request) {
    let path = request.get_local_path(&APP_CONFIG.server.root_folder);
    if std::path::Path::new(&path).exists() {
        let meta = fs::metadata(&path).unwrap();
        if meta.is_file() {
            // TODO Optimize this, hend filetipe headers and load file in chunks
            let mut file = File::open(&path).unwrap();
            let mut data: Vec<u8> = Vec::new();

            // Headers
            let mut headers = ResponseHeaders::new(HttpStatus::OK);
            headers.set_cross_origin_allow_all();
            headers.set_content_length(meta.len());
            let headers_processed = headers.get_headers();

            if APP_CONFIG.debug.active {
                log(&headers, Color::Cyan);
                log(&headers_processed, Color::Cyan);
            }

            stream.write(headers_processed.as_bytes()).unwrap();

            file.read_to_end(&mut data).unwrap();
            stream.write(data.as_slice()).unwrap();
        } else if meta.is_dir() {
            if APP_CONFIG.server.list_directories {
                let mut headers = ResponseHeaders::new(HttpStatus::OK);
                stream.write(headers.get_headers().as_bytes()).unwrap();
                stream.write(read_dir(&request).as_bytes()).unwrap();
            } else {
                let mut headers = ResponseHeaders::new(HttpStatus::Forbidden);
                stream.write(headers.get_headers().as_bytes()).unwrap();
                stream.write(error_page(HttpStatus::Forbidden).as_bytes()).unwrap();
            }
        }
    } else {
        let mut headers = ResponseHeaders::new(HttpStatus::NotFound);
        stream.write(headers.get_headers().as_bytes()).unwrap();
        stream.write(error_page(HttpStatus::NotFound).as_bytes()).unwrap();
    }
}

fn header_template() -> Markup {
    html!{
        head{
            meta name="viewport" content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0"{}
            meta http-equiv="X-UA-Compatible" content="ie=edge"{}
            meta charset="UTF-8" {}
            title{
                "AkiraServerV3!"
            }
            style{
                (PreEscaped(BOOTSTRAP_CSS))
            }
            /*
            script {
                (PreEscaped(JQUERY_JS))
            }
            */
        }
    }
}

fn error_page(error_code:HttpStatus) -> String {
    let template:Markup = html!{
        html{
            (header_template())
            body{
                div class="container"{
                    // Todo Fix this alert
                    div class="alert alert-danger" role="alert" {
                        h3{
                            "Oops! the request can not be processed"
                            br{}
                            "Error: "(error_code.to_int())
                        }
                        a href="/" class="btn btn-primary"{
                            "Safe Place!!!"
                        }
                    }
                }
            }
        }
    };
    template.into_string()
}

// ToDo Test Bytes instead of strings for better performance
fn read_dir(request: &Request) -> String {

    let request_path = request.get_local_path(&APP_CONFIG.server.root_folder);



    let paths = fs::read_dir(&request_path).unwrap();

    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for item_info in paths {
        let path = item_info.unwrap().path().display().to_string();
        let md = fs::metadata(&path).unwrap();
        if md.is_dir() {
            directories.push(get_web_path(path, &request_path));
        } else if md.is_file() {
            files.push(get_web_path(path, &request_path));
        }
    }

    let dir_len = directories.len();
    let file_len = files.len();

    let template:Markup = html! {
        html{
            (header_template())
            body{
                div class="container"{
                    h1{
                        "Listing:"(&request.path)
                    }
                    @if (request_path.as_bytes()) != (SERVER_ROOT.as_bytes()) {
                        a href=".." class="btn btn-primary" { "Upper Directory" }
                    }
                    @if dir_len > 0 {
                        h3 {
                            "Directories"
                        }
                        @for uri in &directories {
                            a href=(percent_encode(uri, true)) style="display:block;" { (uri) }
                        }
                    }
                    br{}
                    @if file_len > 0 {
                        h3 {
                            "Files"
                        }
                        @for uri in &files {
                            a href=(percent_encode(uri, false)) style="display:block;" { (uri) }
                        }
                    }
                }
            }
        }
    };
    template.into_string()
}

fn percent_encode(link: &String, is_dir:bool) -> String {
    let encoded = percent_encoding::utf8_percent_encode(
        &link.replace('%', "%25"),
        percent_encoding::DEFAULT_ENCODE_SET,
    )
    .to_string();
    if is_dir {
        encoded+"\\"
    } else {
        encoded
    }
}

fn get_web_path(full_path: String, path: &String) -> String {
    full_path.trim_start_matches(&*path).to_string()
}

fn add_string(a: &String, b: String) -> String {
    a.to_string() + &b
}

fn log<T: Any + Debug>(data: &T, color: Color) {
    // https://en.wikipedia.org/wiki/ANSI_escape_code
    if APP_CONFIG.debug.active {
        utils::log::log(
            data,
            color,
            true,
            APP_CONFIG.debug.log_to_console,
            APP_CONFIG.debug.log_to_file,
        );
    }
}
