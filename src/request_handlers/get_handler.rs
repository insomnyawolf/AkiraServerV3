use maud::*;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use crate::request::request::Request;
use crate::response::headers::ResponseHeaders;
use crate::response::status::HttpStatus;
use crate::utils::log::log;
use crate::APP_CONFIG;
use termcolor::Color;

// Resources
const BOOTSTRAP_CSS: &'static str = include_str!("../../resources/bootstrap.css");
// const JQUERY_JS:&'static str = include_str!("../resources/jquery-3.4.1.js");

pub fn handle_get(mut stream: &TcpStream, request: &Request) {
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
                stream
                    .write(error_page(HttpStatus::Forbidden).as_bytes())
                    .unwrap();
            }
        }
    } else {
        let mut headers = ResponseHeaders::new(HttpStatus::NotFound);
        stream.write(headers.get_headers().as_bytes()).unwrap();
        stream
            .write(error_page(HttpStatus::NotFound).as_bytes())
            .unwrap();
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

    let template: Markup = html! {
        html{
            (header_template())
            body{
                div class="container"{
                    h1{
                        "Listing:"(&request.path)
                    }
                    @if (request_path.as_bytes()) != (APP_CONFIG.server.root_folder.as_bytes()) {
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

fn get_web_path(full_path: String, path: &String) -> String {
    full_path.trim_start_matches(&*path).to_string()
}
