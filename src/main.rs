#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate config;

use num_cpus;
use std::thread;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use threadpool::ThreadPool;

// For Config
mod settings;

// Request Module
mod request;
use request::*;

lazy_static! {
    #[derive(Debug)]
    static ref APP_CONFIG: settings::Settings = settings::Settings::new_unwrap();
}

// Resources
static ERROR_PAGE:&[u8] = include_bytes!("../404.html");
static HELLO_PAGE:&[u8] = include_bytes!("../hello.html");

// Http Headers
static HTTP_NOT_FOUND:&[u8] = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";
static HTTP_OK:&[u8] = b"HTTP/1.1 200 OK\r\n\r\n";

fn main() {
    //let path = env::current_dir().unwrap();
    //print!("Current server location {} \n", path.as_path().display());

    APP_CONFIG.show();

    server();
}


fn server(){
    // Obtiene numero procesadores logicos
    let core_count = num_cpus::get();
    // Calcula trabajos por procasador logico
    let n_workers = core_count * APP_CONFIG.server.workers_per_thread;
    // Inicia piscina de trabajos limitada
    let pool = ThreadPool::new(n_workers);
    // Echo
    print!("Starting server with {} thread max\n", core_count);
    // Bind de la direccion tcp
    let listener = TcpListener::bind(format!("{host}:{port}", host = APP_CONFIG.server.host, port = APP_CONFIG.server.port)).unwrap();
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
    let mut buffer = [0; 512];
    //Parse request data
    stream.read(&mut buffer).unwrap();
    //Prints request info in the

    let mut request = Request::default();
    request.parse(&buffer);

    print!("{:?}\n", &request);

    //"GET / HTTP/1.1\r\n"
    if &request.method == ("GET") {
        test1(stream, request);
    } else {
        println!("Unsupported Method: {}", request.method);
        //test2(stream, request);
    }
}

fn handle_get(mut stream: TcpStream, request: Request){

}

fn test1(mut stream: TcpStream, request: Request) {
    //Send response
    let mut response = Vec::new();
    response.extend_from_slice(HTTP_OK);
    response.extend_from_slice(read_dir().as_bytes());
    //response.extend_from_slice(request.as_bytes());
    stream.write(response.as_slice()).unwrap();
    stream.flush().unwrap();
}

/*
fn test2(mut stream: TcpStream, buffer: Request) {
    println!("Request: {}", request);
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (header, data) = if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        (HTTP_NOT_FOUND, HELLO_PAGE)
    } else {
        (HTTP_NOT_FOUND, ERROR_PAGE)
    };

    let mut response = Vec::new();
    response.extend_from_slice(header);
    response.extend_from_slice(data);

    stream.write(response.as_slice()).unwrap();
    stream.flush().unwrap();
}
*/
/*
//Opens file
    let mut file = File::open(filename).unwrap();
    //Create empty string
    let mut contents = String::new();
    //Load file content onto variable
    file.read_to_string(&mut contents).unwrap();
    //Format response
    let response = format!("{}{}{}", status_line, contents, request);
    println!("{}", response);
    //Send response
*/

fn read_dir() -> String {
    let paths = fs::read_dir(format!("{}", APP_CONFIG.server.root_folder)).unwrap();
    let mut result = String::new();
    for path in paths {
        result = format!(
            "{0} <a href='{1}'>{1}</a><br />",
            result,
            path.unwrap().path().display()
        )
    }
    result
}

