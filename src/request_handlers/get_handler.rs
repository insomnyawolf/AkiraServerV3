// Template System
extern crate maud;
// Mime
extern crate mime_guess;

use maud::*;
use std::fs;
use std::fs::{File, Metadata};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::request::request::Request;
use crate::response::headers::ResponseHeaders;
use crate::response::status::HttpStatus;
use crate::utils::check_stream_write;
use crate::utils::log::*;
use crate::APP_CONFIG;
use std::path::Path;

// Resources
const BOOTSTRAP_CSS: &'static str = include_str!("../../resources/bootstrap.css");
// const JQUERY_JS:&'static str = include_str!("../resources/jquery-3.4.1.js");

pub fn handle_get(mut stream: &TcpStream, request: &Request) {
    let path_str = request.get_local_path(&APP_CONFIG.server.root_folder);
    let path: &Path = std::path::Path::new(&path_str);
    if path.exists() {
        match fs::metadata(&path) {
            Ok(value) => {
                if value.is_file() {
                    serve_file(stream, value, path);
                } else if value.is_dir() {
                    serve_directory(stream, request);
                } else {
                    log_error(&"The target is neither a file or a directory.");
                }
            }
            Err(err) => {
                log_warning(&err);
            }
        }
    } else {
        let mut headers = ResponseHeaders::new(HttpStatus::NotFound);
        check_stream_write(stream.write(headers.get_headers().as_bytes()));
        check_stream_write(stream.write(error_page(HttpStatus::NotFound).as_bytes()));
    }
}

fn serve_file(mut stream: &TcpStream, meta: Metadata, path: &Path) {
    // Headers
    let mut headers = ResponseHeaders::new(HttpStatus::OK);
    headers.set_cross_origin_allow_all();
    headers.set_content_length(meta.len());
    // https://docs.rs/mime_guess/2.0.0-alpha.6/mime_guess/fn.octet_stream.html
    let mime = mime_guess::guess_mime_type_opt(path);
    match mime {
        Some(value) => {
            headers.set_content_type(value.to_string());
        }
        None => {
            log_error(&"No mime found");
        }
    };
    let headers_processed = headers.get_headers();

    if APP_CONFIG.debug.active {
        log_verbose(&headers);
        log_verbose(&headers_processed);
    }

    check_stream_write(stream.write(headers_processed.as_bytes()));
    // Max buffer Read
    const CAP: usize = 8192;
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::with_capacity(CAP, file);
    // Chunked Transfer WORKS!!! \:D/
    loop {
        let length = {
            let buffer = reader.fill_buf().unwrap();
            // do stuff with buffer here
            match stream.write(buffer) {
                Err(err) => {
                    log_error(&err);
                    break;
                }
                Ok(_value) => {}
            }
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length);
    }
}

fn serve_directory(mut stream: &TcpStream, request: &Request) {
    let request_path = &request.get_local_path(&APP_CONFIG.server.root_folder);

    let content: DirContent = DirContent::read_dir(request_path);

    for file in &content.files {
        for name in &APP_CONFIG.server.index {
            if file == name {
                let file: String = request_path.to_string() + &file;

                let p: &Path = std::path::Path::new(&file);

                let meta: Metadata = fs::metadata(p).unwrap();
                serve_file(stream, meta, p);
                return;
            }
        }
    }

    if APP_CONFIG.server.list_directories {
        let mut headers = ResponseHeaders::new(HttpStatus::OK);
        check_stream_write(stream.write(headers.get_headers().as_bytes()));
        check_stream_write(stream.write(list_directory(content, &request.path).as_bytes()));
    } else {
        let mut headers = ResponseHeaders::new(HttpStatus::Forbidden);
        check_stream_write(stream.write(headers.get_headers().as_bytes()));
        check_stream_write(stream.write(error_page(HttpStatus::Forbidden).as_bytes()));
    }
}

fn header_template() -> Markup {
    html! {
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

fn error_page(error_code: HttpStatus) -> String {
    let template: Markup = html! {
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

fn list_directory(content: DirContent, path: &String) -> String {
    let dir_len = content.directories.len();
    let file_len = content.files.len();
    // get_web_path(path, &request_path)

    let template: Markup = html! {
        html{
            (header_template())
            body{
                div class="container"{
                    h1{
                        "Listing:"(path)
                    }
                    @if (path) != ("/") {
                        a href=".." class="btn btn-primary" { "Upper Directory" }
                    }
                    @if dir_len > 0 {
                        h3 {
                            "Directories"
                        }
                        @for uri in &content.directories {
                            a href=(percent_encode(uri, true)) style="display:block;" { (uri) }
                        }
                    }
                    br{}
                    @if file_len > 0 {
                        h3 {
                            "Files"
                        }
                        @for uri in &content.files {
                            a href=(percent_encode(uri, false)) style="display:block;" { (uri) }
                        }
                    }
                }
            }
        }
    };
    template.into_string()
}

fn percent_encode(link: &String, is_dir: bool) -> String {
    let encoded = percent_encoding::utf8_percent_encode(
        &link.replace('%', "%25"),
        percent_encoding::DEFAULT_ENCODE_SET,
    )
    .to_string();
    if is_dir {
        encoded + "\\"
    } else {
        encoded
    }
}

fn get_web_path(full_path: String, current_dir: &String) -> String {
    let temp = full_path.replace(current_dir, "");
    temp.trim_start_matches(&APP_CONFIG.server.root_folder)
        .to_string()
}

#[derive(Default, Debug)]
struct DirContent {
    directories: Vec<String>,
    files: Vec<String>,
}

impl DirContent {
    // ToDo Test Bytes instead of strings for better performance
    fn read_dir(path: &String) -> DirContent {
        let mut content: DirContent = DirContent::default();

        let paths = fs::read_dir(&path).unwrap();

        for item_info in paths {
            let item_path = item_info.unwrap().path().display().to_string();
            let md = fs::metadata(&item_path).unwrap();
            let web_path = get_web_path(item_path, path);
            if md.is_dir() {
                content.directories.push(web_path);
            } else if md.is_file() {
                content.files.push(web_path);
            }
        }
        content
    }
}
