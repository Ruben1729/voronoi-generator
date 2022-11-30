extern crate glfw;
use glfw::{Action, Context, Key, WindowEvent};

pub mod window_config;
use window_config::WindowConfig;

use std::sync::mpsc::Receiver;

pub struct Display {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>
}

impl Display {
    pub fn create(window_config: WindowConfig) -> Display {
        let mut _glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW Window.");

        _glfw.window_hint(glfw::WindowHint::Visible(false));
        _glfw.window_hint(glfw::WindowHint::Decorated(window_config.decorated));
        _glfw.window_hint(glfw::WindowHint::Floating(window_config.always_on_top));
        _glfw.window_hint(glfw::WindowHint::Resizable(window_config.resizable));

        // Create window
        let (mut window, events) = _glfw.create_window(
            window_config.width as u32,
            window_config.height as u32,
            window_config.title,
            window_config.window_mode)
            .expect("Failed to create GLFW window.");

        _glfw.with_primary_monitor(|_: &mut _, m: Option<&glfw::Monitor> | {
            let monitor = m.expect("Unable to find Monitor.");
            let mode = monitor.get_video_mode().expect("Unable to initialize Monitor Mode.");

            let mut pos_x = window_config.x;
            let mut pos_y = window_config.y;

            if window_config.x == window_config::CX_DISPLAY_CENTER {
                pos_x = (mode.width / 2) as i32 - (window_config.width / 2) as i32;
            }

            if window_config.y == window_config::CX_DISPLAY_CENTER {
                pos_y = (mode.height / 2) as i32 - (window_config.height / 2) as i32;
            }

            window.set_pos(pos_x, pos_y);
        });

        window.show();

        // Poll for all events
        window.set_all_polling(true);
        window.make_current();

        return Display{glfw: _glfw, window, events};
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(& self.events) {
            Self::handle_window_event(&mut self.window, event);
        }
    }

    fn handle_window_event(window: &mut glfw::Window, event: WindowEvent) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
