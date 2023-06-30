use crossterm::style::Color;

#[derive(PartialEq)]
pub enum Type {
    None,
    Number,
}

impl Type {
    pub fn to_color(&self) -> Color {
        match self {
            Type::Number => Color::Rgb { r: 192, g: 232, b: 127 },
            _ => Color::Rgb { r: 204, g: 204, b: 204 },
        }
    }
}
