use std::net::TcpStream;

#[derive(Debug, Default)]
pub struct Other {
    pub remote_ip: String,
    pub remote_port: String,
}

impl Other {
    pub fn parse(stream: TcpStream) -> Other {
        let remote_addr = stream.peer_addr().unwrap();
        Other {
            remote_ip: remote_addr.ip().to_string(),
            remote_port: remote_addr.port().to_string(),
        }
    }
}
