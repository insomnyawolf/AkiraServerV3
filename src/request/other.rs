use std::net::TcpStream;

#[derive(Debug, Default)]
pub struct Other {
    pub remore_adress: String,
}

impl Other {
    pub fn parse(stream: TcpStream) -> Other {
        Other {
            remore_adress: stream.peer_addr().unwrap().to_string(),
        }
    }
}
