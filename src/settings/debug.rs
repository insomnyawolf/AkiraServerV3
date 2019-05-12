//! Debug Settings
#[derive(Debug, Deserialize)]
/// Contains Debug Releated Config
pub struct Debug {
    pub active: bool,
    pub log_to_console: bool,
    pub log_to_file: bool,
}
