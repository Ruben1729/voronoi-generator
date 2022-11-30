extern crate core;

mod display;

use crate::display::Display;
use crate::display::window_config::DEFAULT;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    let mut s_display = Display::create(DEFAULT);

    gl::load_with(|s| s_display.window.get_proc_address(s) as *const _);
    unsafe {
        println!("{}", gl::GetError());
    }


    while !s_display.window.should_close() {
        s_display.update();
    }
}
