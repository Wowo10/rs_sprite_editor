extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate sdl2;

mod lib;
use lib::*;

use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::path::Path;

use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
use sdl2::render::TextureCreator;

use imgui::*;

enum ImageType {
    doodad,
    spritesheet,
    album,
}

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

    sdl2::image::init(INIT_PNG | INIT_JPG).expect("Counld not init SDL Image.");

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

    let texture = texture_creator
        .load_texture(Path::new("resources/spritesheets/anim.png"))
        .unwrap();

    let frames_per_anim = 6;
    let sprite_tile_size = (52, 76);

    let mut source_rect = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.1);

    let mut dest_rect = Rect::new(20, 20, sprite_tile_size.0, sprite_tile_size.1);

    println!("{:?}, {:?}", dest_rect, dest_rect.center());

    //dest_rect.center_on(Point::new(100,100));

    // dest_rect.set_x(100);
    // // dest_rect.set_y(100);

    // println!("{:?}, {:?}", dest_rect, dest_rect.center());

    // let kek = dest_rect.center();

    //println!("{:?}, {:?}", kek, texture.query());

    let mut scale = 1.0f32;

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

        source_rect.set_x(sprite_tile_size.0 as i32 * ((ticks / 1000) % frames_per_anim));

        canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 200, 20));
        canvas.clear();

        canvas.set_scale(scale, scale).unwrap();

        let tempx = if dest_rect.x != 0 { dest_rect.x } else { 1 } as f32 / scale;
        let tempy = if dest_rect.y != 0 { dest_rect.x } else { 1 } as f32 / scale;

        let mut temp_rect = dest_rect;

        temp_rect.set_x(tempx as i32);
        temp_rect.set_y(tempy as i32);
        canvas
            .copy_ex(
                &texture,
                Some(source_rect),
                Some(temp_rect),
                0.0,
                None,
                false,
                false,
            ).unwrap();

        // dest_rect.set_x(100);
        // canvas
        //     .copy_ex(
        //         &texture,
        //         Some(source_rect),
        //         Some(dest_rect),
        //         0.0,
        //         None,
        //         false,
        //         false,
        //     ).unwrap();

        // dest_rect.set_x(200);
        // canvas
        //     .copy_ex(
        //         &texture,
        //         Some(source_rect),
        //         Some(dest_rect),
        //         0.0,
        //         None,
        //         false,
        //         false,
        //     ).unwrap();

        // RED RECT
        canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 20, 20));
        canvas.draw_rect(temp_rect).unwrap();

        let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);

        let mut slide = 3;

        ui.window(im_str!("test"))
            .size((300.0, 500.0), ImGuiCond::Appearing)
            .position((600.0, 140.0), ImGuiCond::Appearing)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();

                ui.slider_int(im_str!("slider"), &mut slide, 0, 1).build();

                ui.separator();

                ui.slider_float(im_str!(""), &mut scale, 0.5, 6.0).build();

                ui.separator();
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));
            });

        canvas.window_mut().gl_make_current(&gl_context).unwrap();
        renderer.render(ui);

        &canvas.present();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
