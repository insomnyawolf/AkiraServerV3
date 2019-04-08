use num_cpus;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    let core_count = num_cpus::get();
    print!("Starting server with {} thread max\n", core_count);
    let path = env::current_dir().unwrap();
    print!("Current server location {} \n", path.as_path().display());
    let n_workers = core_count;
    let pool = ThreadPool::new(n_workers);

    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            //Launch handle_conection in new thread
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    //Parse request data
    stream.read(&mut buffer).unwrap();
    //Prints request info in the
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        test1(stream, buffer);
    } else {
        test2(stream, buffer);
    }
}

fn test1(mut stream: TcpStream, buffer: [u8; 512]) {
    let request = String::from_utf8_lossy(&buffer[..]);
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("{}{}{}", status_line, read_dir(), request);
    println!("{}", response);
    //Send response
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn test2(mut stream: TcpStream, buffer: [u8; 512]) {
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
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
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn read_dir() -> String {
    let paths = fs::read_dir("./").unwrap();
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
