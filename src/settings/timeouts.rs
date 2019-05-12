//! Timeout Settings
#[derive(Debug, Deserialize)]
/// Contains Timeouts Releated Config
pub struct Timeouts {
    /// How much time will pass before the server consider the request as done
    pub request_miliseconds: u32,
}

impl Timeouts {
    /// Converts miliseconds to nanoseconds, unsed creating Duratin type
    pub fn get_nanoseconds(&self) -> u32 {
        self.request_miliseconds * 1000000
    }
}
