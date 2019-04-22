#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate config;

use num_cpus;
//use std::thread;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// use std::time::Duration;
use threadpool::ThreadPool;

// For Config
mod settings;

// Request Module
mod request;
use request::*;

// Url
extern crate percent_encoding;

lazy_static! {
    #[derive(Debug)]
    static ref APP_CONFIG: settings::Settings = settings::Settings::new_unwrap();
    static ref SERVER_ROOT: String = add_string(&APP_CONFIG.server.root_folder , "/".to_string());
}

// Resources
static HTML_HEADER:&[u8] = include_bytes!("../resources/html_header.html");
static HTML_CLOSE:&[u8] = include_bytes!("../resources/html_close.html");
static HTML_ERROR_PAGE:&[u8] = include_bytes!("../resources/404.html");
static HTML_HELLO_PAGE:&[u8] = include_bytes!("../resources/hello.html");

// Http Headers
static HTTP_NOT_FOUND:&[u8] = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";
static HTTP_OK:&[u8] = b"HTTP/1.1 200 OK\r\n\r\n";

fn main() {
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
    // If request buffer is too small it may fail
    let mut buffer = [0;1024];
    //Parse request data
    stream.read(&mut buffer).unwrap();
    //Prints request info in the

    let request = Request::parse(&buffer);

    if APP_CONFIG.debug.active {
        println!("{:?}", &request);
    }

    //"GET / HTTP/1.1\r\n"
    if &request.method == ("GET") {
        handle_get(stream, &request);
    } else {
        println!("Unsupported Method: {}", request.method);
        //test2(stream, request);
    }
}

fn handle_get(mut stream: TcpStream, request:&Request){
    let path = request.get_local_path(&APP_CONFIG.server.root_folder);
    if std::path::Path::new(&path).exists() {
        if fs::metadata(&path).unwrap().is_file(){
            stream.write(HTTP_OK).unwrap();

            // TODO Optimize this, hend filetipe headers and load file in chunks
            let mut file = File::open(path).unwrap();
            let mut data: Vec<u8> = Vec::new();
            file.read_to_end(&mut data).unwrap();
            stream.write(data.as_slice()).unwrap();

        } else {
            if request.path.ends_with("/") || request.path.ends_with("\\"){
                if APP_CONFIG.server.list_directories {
                    stream.write(HTTP_OK).unwrap();
                    stream.write(HTML_HEADER).unwrap();
                    stream.write(read_dir(&request).as_bytes()).unwrap();
                    stream.write(HTML_CLOSE).unwrap();
                } else {
                    stream.write(HTTP_NOT_FOUND).unwrap();
                    stream.write(HTML_HEADER).unwrap();
                    stream.write(HTML_ERROR_PAGE).unwrap();
                    stream.write(HTML_CLOSE).unwrap();
                }
            }
        }
    } else {
        stream.write(HTTP_NOT_FOUND).unwrap();
        stream.write(HTML_HEADER).unwrap();
        stream.write(HTML_ERROR_PAGE).unwrap();
        stream.write(HTML_CLOSE).unwrap();
    }
}

/*
fn test1(mut stream: TcpStream, request: Request) {
    //Send response
    let mut response = Vec::new();
    response.extend_from_slice(HTTP_OK);
    response.extend_from_slice(read_dir().as_bytes());
    //response.extend_from_slice(request.as_bytes());
    stream.write(response.as_slice()).unwrap();
    stream.flush().unwrap();
}
*/
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


// ToDo Test Bytes instead of strings for better performance
fn read_dir(request:&Request) -> String {
    let mut result = String::new();
    result = add_string(& result,format!("<h1>Listing:{}</h1><br />", &request.path));

    let request_path = request.get_local_path(&APP_CONFIG.server.root_folder);
    if request_path.as_bytes() != SERVER_ROOT.as_bytes() {
        result = add_string(& result,"<a href='..'>Upper Directory</a><br />".to_string());
    }

    if APP_CONFIG.debug.active {
        println!("{}",&request_path);
    }

    let paths = fs::read_dir(&request_path).unwrap();

    let mut directories:Vec<String> = Vec::new();
    let mut files:Vec<String> = Vec::new();

    for item_info in paths {
        let path = item_info.unwrap().path().display().to_string();
        let md = fs::metadata(&path).unwrap();
        if md.is_dir() {
            directories.push(path);
        }else if md.is_file() {
            files.push(path);
        }
    }

    if directories.len() > 0 {
        result = add_string(& result,"<h2>Directories</h2><br />".to_string());
    }

    for path in directories {
        let link = get_web_path(path, &request_path);
        result = add_string(& result,format!("<a href='{1}/'>{0}</a><br />", link, percent_encoding::utf8_percent_encode(&link, percent_encoding::DEFAULT_ENCODE_SET)));
    }

    if files.len() > 0 {
        result = add_string(&result, "<h2>Files</h2><br />".to_string());
    }

    for path in files {
        let link = get_web_path(path, &request_path);
        result = add_string(& result,format!("<a href='{1}'>{0}</a><br />", link, percent_encoding::utf8_percent_encode(&link, percent_encoding::DEFAULT_ENCODE_SET)));
    }
    result
}

fn get_web_path(full_path:String, path:&String) -> String {
    full_path.trim_start_matches(&*path).to_string()
}

fn add_string(a:&String, b:String) -> String {
    a.to_string() + &b
}

