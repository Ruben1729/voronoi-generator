extern crate glfw;
use glfw::{WindowMode};

pub const CX_DISPLAY_CENTER: i32 = -1;

pub struct WindowConfig<'a> {
    pub window_mode: WindowMode<'a>,

    pub title: &'static str,

    // Position of the window's top left corner in screen pixels
    pub x: i32,
    pub y: i32,

    // Size of window in pixels
    pub width: i32,
    pub height: i32,

    // If true, show borders and controls
    pub decorated: bool,

    // If true, allow window resize
    pub resizable: bool,

    // V-sync enable
    pub vsync: bool,

    // Should the window be rendered on top of other windows
    pub always_on_top: bool,

    // Full screen mode
    pub fullscreen: bool
}

pub const DEFAULT: WindowConfig = WindowConfig{
    window_mode     : WindowMode::Windowed,

    title           : "Voronoi Generator",

    x               : CX_DISPLAY_CENTER,
    y               : CX_DISPLAY_CENTER,

    width           : 1280,
    height          : 720,

    decorated       : true,
    resizable       : false,
    vsync           : true,
    always_on_top   : false,
    fullscreen      : true
};
