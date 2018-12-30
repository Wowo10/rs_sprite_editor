extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate sdl2;

mod lib;

mod fragment;

mod mymath;

mod file_utils;

mod ui_stuff;

mod config;

mod resource_manager;

mod app;

fn main() {
    let mut app = app::App::new();

    app.run();
}
