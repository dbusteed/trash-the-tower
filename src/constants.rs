use bevy::prelude::Color;

pub const WIDTH: f32 = 900.0;
pub const HEIGHT: f32 = 600.0;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub const LAUNCH_FACTOR: f32 = 2.0;
pub const TARGET_FORCE_THRESH: f32 = 20.0;
pub const TARGET_COLOR: (Color, Color) = (Color::rgb(0.82, 0.16, 0.16), Color::rgb(0.65, 0.11, 0.11));

pub const GROUND_HEIGHT: f32 = 15.0;
pub const GROUND_COLOR: Color = Color::DARK_GREEN;

pub struct Material {
    pub density: f32,
    pub color1: Color,
    pub color2: Color,
}

pub const WOOD1: Material = Material {
    density: 0.5,
    color1: Color::rgb(0.6, 0.4, 0.2),
    color2: Color::rgb(0.525, 0.349, 0.176),
};

pub const WOOD2: Material = Material {
    density: 1.0,
    color1: Color::rgb(0.451, 0.302, 0.149),
    color2: Color::rgb(0.376, 0.251, 0.125),
};

pub const STONE1: Material = Material {
    density: 1.5,
    // color1: Color::rgb(0.510, 0.510, 0.490),
    // color2: Color::rgb(0.459, 0.459, 0.439),
    color1: Color::rgb(0.408, 0.408, 0.392),
    color2: Color::rgb(0.357, 0.357, 0.341),
};

pub const STONE2: Material = Material {
    density: 2.0,
    color1: Color::rgb(0.306, 0.306, 0.294),
    color2: Color::rgb(0.255, 0.255, 0.243),
};
