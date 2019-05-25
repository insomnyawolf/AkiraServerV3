use crate::utils::log::log_error;
use std::net::TcpStream;

#[derive(Debug, Default)]
pub struct Other {
    pub remote_ip: String,
    pub remote_port: String,
}

impl Other {
    pub fn parse(stream: &TcpStream) -> Other {
        match stream.peer_addr() {
            Ok(remote_addr) => Other {
                remote_ip: remote_addr.ip().to_string(),
                remote_port: remote_addr.port().to_string(),
            },
            Err(error) => {
                log_error(&error);
                Other::default()
            }
        }
    }
}
