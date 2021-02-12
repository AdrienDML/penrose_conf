use penrose::core::{
    layout::LayoutConf
};

pub const CONF_FF_GL_WON: LayoutConf = LayoutConf {
    floating: false,
    gapless: true,
    follow_focus: true,
    allow_wrapping: true,
};

pub const TERMINAL: &str = "alacritty";
pub const LAUNCHER: &str = "dmenu_run";

pub const FLOAT_CLASS: &str = "floating";
pub const N_MAIN: u32 = 1;
pub const RATIO: f32 = 0.6;
pub const FONT : &str = "mononokio Nerd Font";

pub const SB_HEIGHT : usize = 18;

pub const BLACK: &str = "#282828";
pub const WHITE: &str = "#ebdbb2";
pub const GREY: &str = "#3c3836";
pub const BLUE: &str = "#458588";