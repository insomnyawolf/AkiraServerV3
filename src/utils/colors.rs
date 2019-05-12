/// Ansi colors that can be used to print in the stdout
pub enum Color {
    Default,
    BlueDark,
    BlueLight,
    Green,
    Red,
}

impl Color {
    /// Converts the given value to a String
    pub fn to_string(&self) -> &str {
        match *self {
            Color::Default => "0",
            Color::BlueDark => "34",
            Color::BlueLight => "96",
            Color::Green => "32",
            Color::Red => "31",
        }
    }
}
