
pub const MAX_VALUE: u8 = 255;
const MAX_VALUE_F: f32 = MAX_VALUE as f32;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32
}


impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: (r as f32) / MAX_VALUE_F,
            green: (g as f32) / MAX_VALUE_F,
            blue: (b as f32) / MAX_VALUE_F
        }
    }

    pub fn red(self) -> u8 {
        (self.red * MAX_VALUE_F) as u8
    }

    pub fn green(self) -> u8 {
        (self.green * MAX_VALUE_F) as u8
    }

    pub fn blue(self) -> u8 {
        (self.blue * MAX_VALUE_F) as u8
    }
}
