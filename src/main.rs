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

use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use num_cpus;
use threadpool::ThreadPool;

// For Config
mod settings;

// Request Module
mod request;
mod response;

use request::*;
use response::*;
use std::any::Any;
use std::time::Duration;

lazy_static! {
    #[derive(Debug)]
    static ref APP_CONFIG: settings::Settings = settings::Settings::new_unwrap();
    static ref SERVER_ROOT: String = add_string(&APP_CONFIG.server.root_folder , "/".to_string());
}

// Resources
static HTML_HEADER: &[u8] = include_bytes!("../resources/html_header.html");
static HTML_CLOSE: &[u8] = include_bytes!("../resources/html_close.html");
static HTML_ERROR_PAGE: &[u8] = include_bytes!("../resources/404.html");

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

fn handle_connection(stream: TcpStream) {
    // Create a Duration ans set is as timeout
    // That way the server doesnt keep waiting for more bytes
    let timeout = Some(Duration::new(
        APP_CONFIG.timeout.request_seconds,
        APP_CONFIG.timeout.get_nanoseconds(),
    ));
    // Get a copy of TcpStream
    //Parse request data
    let request = Request::parse(stream.try_clone().unwrap(), timeout);

    if request.is_valid_request {
        log(&request);

        // Switch Equivalent
        match request.method {
            Method::GET => {
                handle_get(stream, &request);
            }
            _ => {
                println!("Unsupported Method\n");
                //test2(stream, request);
            }
        }
    } else {
        log(&"Invalid Request");
    }
}

fn handle_get(mut stream: TcpStream, request: &Request) {
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

            if APP_CONFIG.debug.active {
                log(&headers);
            }

            stream.write(&headers.get_headers().as_bytes()).unwrap();

            file.read_to_end(&mut data).unwrap();
            stream.write(data.as_slice()).unwrap();
        } else if meta.is_dir() {
            if APP_CONFIG.server.list_directories {
                let mut headers = ResponseHeaders::new(HttpStatus::OK);
                stream.write(headers.get_headers().as_bytes()).unwrap();
                stream.write(HTML_HEADER).unwrap();
                stream.write(read_dir(&request).as_bytes()).unwrap();
                stream.write(HTML_CLOSE).unwrap();
            } else {
                let mut headers = ResponseHeaders::new(HttpStatus::Forbidden);
                stream.write(headers.get_headers().as_bytes()).unwrap();
                stream.write(HTML_HEADER).unwrap();
                stream.write(HTML_ERROR_PAGE).unwrap();
                stream.write(HTML_CLOSE).unwrap();
            }
        }
    } else {
        let mut headers = ResponseHeaders::new(HttpStatus::NotFound);
        stream.write(headers.get_headers().as_bytes()).unwrap();
        stream.write(HTML_HEADER).unwrap();
        stream.write(HTML_ERROR_PAGE).unwrap();
        stream.write(HTML_CLOSE).unwrap();
    }
}

// ToDo Test Bytes instead of strings for better performance
fn read_dir(request: &Request) -> String {
    let mut result = String::new();

    result = add_string(&result, format!("<h1>Listing:{}</h1><br />", &request.path));

    let request_path = request.get_local_path(&APP_CONFIG.server.root_folder);

    if request_path.as_bytes() != SERVER_ROOT.as_bytes() {
        result = add_string(
            &result,
            "<a href=\"..\">Upper Directory</a><br />".to_string(),
        );
    }

    let paths = fs::read_dir(&request_path).unwrap();

    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for item_info in paths {
        let path = item_info.unwrap().path().display().to_string();
        let md = fs::metadata(&path).unwrap();
        if md.is_dir() {
            directories.push(path);
        } else if md.is_file() {
            files.push(path);
        }
    }

    if directories.len() > 0 {
        result = add_string(&result, "<h2>Directories</h2><br />".to_string());
    }

    for path in directories {
        let link = get_web_path(path, &request_path);
        result = add_string(
            &result,
            format!(
                "<a href=\"{1}/\">{0}</a><br />",
                link,
                percent_encoding::utf8_percent_encode(&link, percent_encoding::DEFAULT_ENCODE_SET)
            ),
        );
    }

    if files.len() > 0 {
        result = add_string(&result, "<h2>Files</h2><br />".to_string());
    }

    for path in files {
        let link = get_web_path(path, &request_path);
        result = add_string(
            &result,
            format!("<a href=\"{1}\">{0}</a><br />", link, percent_encode(&link)),
        );
    }
    result
}

fn percent_encode(link: &String) -> String {
    percent_encoding::utf8_percent_encode(
        &link.replace('%', "%25"),
        percent_encoding::DEFAULT_ENCODE_SET,
    )
    .to_string()
}

fn get_web_path(full_path: String, path: &String) -> String {
    full_path.trim_start_matches(&*path).to_string()
}

fn add_string(a: &String, b: String) -> String {
    a.to_string() + &b
}

fn log<T: Any + Debug>(data: &T) {
    if APP_CONFIG.debug.active {
        let s = format!(
            "Debug:\t{time}\n\t{dat:?}\n",
            time = chrono::Local::now(),
            dat = data,
        );
        if APP_CONFIG.debug.log_to_file {
            // ToDo
        }
        if APP_CONFIG.debug.log_to_console {
            println!("{}", s);
        }
    }
}
