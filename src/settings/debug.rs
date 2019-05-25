//! Debug Settings
#[derive(Debug, Deserialize)]
/// Contains Debug Releated Config
pub struct Debug {
    /// Define if debug will be enabled or disabled
    pub active: bool,
    // Todo Comments
    pub error: bool,

    pub warning: bool,

    pub verbose: bool,
    /// Define if the program will log data to the stdout
    pub log_to_console: bool,
    /// Define if the program will log data to a file
    pub log_to_file: bool,
}
