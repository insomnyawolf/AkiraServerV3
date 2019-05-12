pub enum Color {
    Default,
    BlueDark,
    BlueLight,
    Green,
    Red,
}

impl Color {
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
