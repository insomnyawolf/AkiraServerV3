#[derive(Debug, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub client: Client,
}

impl Request {
    pub fn parse(& mut self, buffer: &[u8]) {

        let request_str = String::from_utf8_lossy(&buffer[..]);

        let request_arr:Vec<_> = request_str.splitn(3, ' ').collect();

        self.method = request_arr[0].to_string();
        self.path = percent_encoding::percent_decode(request_arr[1].as_bytes()).decode_utf8().unwrap().to_string();
        let mut client = Client::default();
        client.parse(request_arr[2]);
        self.client = client;
    }

    pub fn get_local_path(&self, root_folder:&String) -> String {
        root_folder.to_string() + &self.path
    }

}

#[derive(Debug, Default)]
pub struct Client {
    pub version: String,
    pub browser: String,
    pub connection: String,
    pub cache_control: String,
    pub upgrade_insecure_requests: String,
    pub user_agent: String,
    pub dnt: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_anguage: String,
    pub cookie: String,
    pub other: String,
}

impl Client {
    pub fn parse(& mut self, client_str: &str) {
        let client_arr:Vec<_> = client_str.splitn(12, "\r\n").collect();
        self.version = client_arr[0].to_string();
        self.browser = client_arr[1].to_string();
        self.connection = client_arr[2].to_string();
        self.cache_control = client_arr[3].to_string();
        self.upgrade_insecure_requests = client_arr[4].to_string();
        self.user_agent = client_arr[5].to_string();
        self.dnt = client_arr[6].to_string();
        self.accept = client_arr[7].to_string();
        self.accept_encoding = client_arr[8].to_string();
        self.accept_anguage = client_arr[9].to_string();
        if client_arr.len() > 10 {
            self.cookie = client_arr[10].to_string();
        }else{
            self.cookie = "".to_string();
        }
        if client_arr.len() > 11 {
            self.other = client_arr[11].to_string();
        }else{
            self.other = "".to_string();
        }
    }
}