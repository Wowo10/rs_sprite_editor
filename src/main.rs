extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
//extern crate imgui_sdl2;
extern crate sdl2;

mod lib;

use lib::*;

use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::path::Path;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
    }

    let window = video
        .window("rust-imgui-sdl2 demo", 1000, 1000)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window
        .gl_create_context()
        .expect("Couldn't create GL context");
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let mut imgui = imgui::ImGui::init();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui);

    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut timer = sdl_context.timer().unwrap();

    let temp_surface =
        sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .unwrap();

    let frames_per_anim = 4;
    let sprite_tile_size = (32, 32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_0.center_on(Point::new(-64, 120));

    // King - walk animation
    let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_1.center_on(Point::new(0, 240));

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_2.center_on(Point::new(440, 360));

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

        source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

        source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

        //draw shit
        canvas.window_mut().gl_make_current(&_gl_context).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 200, 20));
        canvas.clear();

        canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).unwrap();
        canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false).unwrap();
        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false).unwrap();

        //ui render
        let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);
        ui.show_demo_window(&mut true);

        // unsafe {
        //     gl::ClearColor(0.9, 0.2, 0.2, 1.0);
        //     gl::Clear(gl::COLOR_BUFFER_BIT);
        // }
        unsafe { gl::Flush() };

        canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 20, 20));
        canvas
            .draw_rect(sdl2::rect::Rect::new(100, 100, 200, 200))
            .unwrap();
        unsafe { gl::Flush() };

        canvas.window_mut().gl_make_current(&_gl_context).unwrap();
        renderer.render(ui);
        unsafe { gl::Flush() };

        &canvas.present();
        //&canvas.window().gl_swap_window();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
