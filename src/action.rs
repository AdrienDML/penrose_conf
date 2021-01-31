use penrose::{
    contrib::{actions::update_monitors_via_xrandr, extensions::dmenu::*},
    core::{bindings::KeyEventHandler, data_types::RelativePosition},
};


pub fn redetect_monitors() -> KeyEventHandler<Conn> {
    Box::new(move |_: &mut Wm| update_monitors_via_xrandr(MON_1, MON_2, RelativePosition::Right))
}

