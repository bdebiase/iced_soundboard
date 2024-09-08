use include_dir::{include_dir, Dir};

pub mod button;
pub mod container;
pub mod icons;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod svg;
pub mod theme;

pub const FONT_BYTES_REGULAR: &[u8] =
    include_bytes!("../../assets/fonts/Roboto/Roboto-Regular.ttf");
pub const FONT_BYTES_BOLD: &[u8] = include_bytes!("../../assets/fonts/Roboto/Roboto-Bold.ttf");
pub const ICON_DIR: Dir = include_dir!("assets/adwaita-icons/");

pub const FONT_NAME: &'static str = "Roboto";
pub const FONT_SIZE_DEFAULT: u16 = 16;
pub const FONT_SIZE_TABS: u16 = 18;

pub const ICON_SIZE: f32 = 18.0;

pub const SPACING_SMALL: u16 = 5;
pub const SPACING_NORMAL: u16 = 10;
#[allow(unused)]
pub const SPACING_LARGE: u16 = 20;

pub const BORDER_RADIUS: f32 = 8.0;
