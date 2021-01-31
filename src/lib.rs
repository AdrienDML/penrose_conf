extern crate penrose;

use penrose::{
    core::layout::{
        LayoutConf,Layout
    }
};

pub mod consts;
#[macro_export]
macro_rules! layout {
    { $name:expr, $func:expr } => {
        Layout::new($name, LayoutConf::default(), $func, N_MAIN, RATIO)
    };
    { $name:expr, $conf:expr, $func:expr } => {
        Layout::new($name, $conf, $func, penrose_conf::N_MAIN, penrose_conf::RATIO)
    };
}