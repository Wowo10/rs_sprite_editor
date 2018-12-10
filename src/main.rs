extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate sdl2;

mod lib;
use lib::*;

use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::path::Path;

fn main() {
    let sdl_context = match sdl2::init() {
        Ok(sdl_context) => sdl_context,
        Err(err) => panic!("SDL could not initialize!  SDL_Error: {}", err),
    };
    let video = match sdl_context.video() {
        Ok(video) => video,
        Err(err) => panic!(
            "Could not obtain handle to the video subsystem! SDL_Error: {}",
            err
        ),
    };

    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
    }

    let window = match video
        .window("rust-imgui-sdl2 demo", 1000, 1000)
        .position_centered()
        .resizable()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("SDL could not create a window! SDL_Error: {}.", err),
    };

    let gl_context = match window.gl_create_context() {
        Ok(gl_context) => gl_context,
        Err(err) => panic!("SDL could not create Gl Context: {}.", err),
    };

    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let mut imgui = imgui::ImGui::init();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui);

    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    let mut event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err) => panic!("SDL could not get event_pump: {}.", err),
    };

    let mut canvas = match window.into_canvas().accelerated().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("SDL could not convart into canvas: {}.", err),
    };

    let texture_creator = canvas.texture_creator();

    let mut timer = sdl_context.timer().expect("Could not create timer.");

    let temp_surface = match sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")) {
        Ok(temp_surface) => temp_surface,
        Err(err) => panic!("SDL could not create surface: {}.", err),
    };
    let texture = match texture_creator.create_texture_from_surface(&temp_surface) {
        Ok(texture) => texture,
        Err(err) => panic!("SDL could not create texture: {}.", err),
    };

    let frames_per_anim = 4;
    let sprite_tile_size = (32, 32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_0.center_on(Point::new(-64, 120));

    'running: loop {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let ticks = timer.ticks() as i32;

        source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);
        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 200, 20));
        canvas.clear();

        canvas
            .copy_ex(
                &texture,
                Some(source_rect_0),
                Some(dest_rect_0),
                0.0,
                None,
                false,
                false,
            ).unwrap();

        //ui render
        let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);
        ui.show_demo_window(&mut true);

        canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 20, 20));
        canvas
            .draw_rect(sdl2::rect::Rect::new(100, 100, 200, 200))
            .unwrap();

        canvas.window_mut().gl_make_current(&gl_context).unwrap();
        renderer.render(ui);

        &canvas.present();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
