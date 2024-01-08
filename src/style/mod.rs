pub mod button;
pub mod container;
pub mod icons;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod theme;

pub const FONT_BYTES_REGULAR: &[u8] =
    include_bytes!("../../resources/fonts/Roboto/Roboto-Regular.ttf");
pub const FONT_BYTES_BOLD: &[u8] = include_bytes!("../../resources/fonts/Roboto/Roboto-Bold.ttf");

pub const FONT_NAME: &'static str = "Roboto";
pub const FONT_SIZE_DEFAULT: u16 = 16;
pub const FONT_SIZE_ICON: u16 = 18;
pub const FONT_SIZE_TABS: u16 = 18;

pub const SPACING_SMALL: u16 = 5;
pub const SPACING_NORMAL: u16 = 10;
#[allow(unused)]
pub const SPACING_LARGE: u16 = 20;

pub const BORDER_RADIUS: f32 = 8.0;
