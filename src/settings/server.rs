//! Server Settings
#[derive(Debug, Deserialize)]
/// Contains Server Releated Config
pub struct Server {
    /// Wich ip will the server listen on
    ///
    /// # Example
    ///
    /// ```
    /// 0.0.0.0
    /// ```
    ///
    /// Will Listen on all available adresses
    pub host: String,
    /// Wich port will the server listen on
    ///
    /// # Example
    ///
    /// ```
    /// 80
    /// ```
    ///
    /// Will Listen on the default http port
    pub port: String,
    /// [TTL](https://en.wikipedia.org/wiki/Time_to_live)  
    ///
    /// # Example
    ///
    /// ```
    /// 128
    /// ```
    ///
    /// This will make the data be discarded after 128 hops
    pub ttl: u32,
    /// Wich directory will the server use as root
    ///
    /// # Example
    ///
    /// ```
    /// ./
    /// ```
    ///
    /// The server will serve it's own directory
    pub root_folder: String,
    /// Defines id the server will list directory content or show an error instead
    ///
    /// # Example
    ///
    /// ```
    /// true
    /// ```
    ///
    /// The will list directory content
    pub list_directories: bool,
    /// Defines the maximum number of works that will be running on each logical processor
    ///
    /// # Example
    ///
    /// ```
    /// 2
    /// ```
    ///
    /// On a 16 core cpu this will be max 32 total threads
    pub workers_per_thread: usize,

    pub index: Vec<String>,
}
