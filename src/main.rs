#![feature(proc_macro_hygiene)] // Enables procedural macros as expresions
                                // Url
extern crate percent_encoding;

// Parser
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derivative;

use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};

use num_cpus;
use threadpool::ThreadPool;

use std::time::Duration;

// Request Handlers
mod request_handlers;
use crate::request_handlers::get_handler::handle_get;

// Util
mod utils;
use crate::utils::log::*;

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

lazy_static! {
    pub static ref APP_CONFIG: Settings = load_settings();
}

fn load_settings() -> Settings {
    let mut settings: Settings = Settings::new().unwrap();
    settings.server.root_folder = add_string(&settings.server.root_folder, "/".to_string());
    settings
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
    let listener_option = TcpListener::bind(format!(
        "{host}:{port}",
        host = APP_CONFIG.server.host,
        port = APP_CONFIG.server.port
    ));
    let listener = match listener_option {
        Ok(value) => (value),
        Err(error) => {
            log_error_fatal(&error);
            panic!();
        }
    };

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

fn add_string(a: &String, b: String) -> String {
    a.to_string() + &b
}
