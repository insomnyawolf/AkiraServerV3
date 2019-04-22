#[derive(Debug, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub client: Client,
}

impl Request {
    pub fn parse(buffer: &[u8]) -> Request {
        let mut req = Request::default();

        let raw = String::from_utf8_lossy(&buffer)
            .to_string()
            .replace("\u{0}", "");

        let request_arr: Vec<_> = raw.splitn(3, ' ').collect();

        req.method = request_arr[0].to_string();
        req.path = percent_encoding::percent_decode(request_arr[1].as_bytes())
            .decode_utf8()
            .unwrap()
            .to_string();
        let mut client = Client::default();
        client.parse(request_arr[2]);
        req.client = client;
        req
    }

    pub fn get_local_path(&self, root_folder: &String) -> String {
        root_folder.to_string() + &self.path
    }
}

#[derive(Debug, Default)]
pub struct Client {
    pub version: String,
    pub host: String,
    pub connection: String,
    pub cache_control: String,
    pub upgrade_insecure_requests: String,
    pub user_agent: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_language: String,
    pub cookie: String,
    pub dnt: String,
    pub pragma: String,
    pub referer: String,
    pub other: Vec<String>,
}

impl Client {
    pub fn parse(&mut self, client_str: &str) {
        let client_arr: Vec<_> = client_str.rsplit("\r\n").collect();

        for data in client_arr {
            let current = data.to_string();

            if current.starts_with("HTTP") {
                self.version = current;
            } else if current.starts_with("Host: ") {
                self.host = current.trim_start_matches("Host: ").to_string();
            } else if current.starts_with("Connection: ") {
                self.connection = current.trim_start_matches("Connection: ").to_string();
            } else if current.starts_with("Cache-Control: ") {
                self.cache_control = current.trim_start_matches("Cache-Control: ").to_string();
            } else if current.starts_with("Upgrade-Insecure-Requests: ") {
                self.upgrade_insecure_requests = current
                    .trim_start_matches("Upgrade-Insecure-Requests: ")
                    .to_string();
            } else if current.starts_with("User-Agent: ") {
                self.user_agent = current.trim_start_matches("User-Agent: ").to_string();
            } else if current.starts_with("Accept: ") {
                self.accept = current.trim_start_matches("Accept: ").to_string();
            } else if current.starts_with("Accept-Encoding: ") {
                self.accept_encoding = current.trim_start_matches("Accept-Encoding: ").to_string();
            } else if current.starts_with("Accept-Language: ") {
                self.accept_language = current.trim_start_matches("Accept-Language: ").to_string();
            } else if current.starts_with("Cookie: ") {
                self.cookie = current.trim_start_matches("Cookie: ").to_string();
            } else if current.starts_with("DNT: ") {
                self.dnt = current.trim_start_matches("DNT: ").to_string();
            } else if current.starts_with("Pragma: ") {
                self.pragma = current.trim_start_matches("Pragma: ").to_string();
            } else if current.starts_with("Referer: ") {
                self.referer = current.trim_start_matches("Referer: ").to_string();
            } else {
                self.other.push(current);
            }
        }
    }
}
