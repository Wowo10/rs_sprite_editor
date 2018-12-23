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

mod fragment;
use fragment::*;

mod mymath;
use mymath::{check_rect2, rotate_point};

//use ui_stuff::timer::Timer;

mod ui_stuff;
use ui_stuff::*;

fn draw_rectangle_around_active(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    active_rect: Rect,
    rotation: f32,
    scale: f32,
) -> [Point; 4] {
    let temp_center = active_rect.top_left()
        + Point::new(
            (active_rect.width() as f32 * scale / 2.0) as i32,
            (active_rect.height() as f32 * scale / 2.0) as i32,
        );

    let top_left = rotate_point(active_rect.top_left(), temp_center, rotation);
    let top_right = rotate_point(
        active_rect.top_left() + Point::new((active_rect.width() as f32 * scale) as i32, 0),
        temp_center,
        rotation,
    );
    let bottom_left = rotate_point(
        active_rect.top_left() + Point::new(0, (active_rect.height() as f32 * scale) as i32),
        temp_center,
        rotation,
    );
    let bottom_right = rotate_point(
        active_rect.top_left()
            + Point::new(
                (active_rect.width() as f32 * scale) as i32,
                (active_rect.height() as f32 * scale) as i32,
            ),
        temp_center,
        rotation,
    );

    canvas.set_scale(1.0, 1.0).unwrap();

    canvas.draw_line(top_left, top_right).unwrap();
    canvas.draw_line(bottom_left, top_left).unwrap();
    canvas.draw_line(bottom_right, bottom_left).unwrap();
    canvas.draw_line(top_right, bottom_right).unwrap();

    [top_left, top_right, bottom_right, bottom_left] //order is very important
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
        .window("rust-imgui-sdl2 demo", 1000, 700)
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

    let texture = texture_creator
        .load_texture(Path::new("resources/spritesheets/animbg.png"))
        .unwrap();

    let sprite_tile_size = (52, 76);
    let mut source_rect = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.1);
    let mut dest_rect = Rect::new(30, 300, sprite_tile_size.0, sprite_tile_size.1);

    let texture2 = texture_creator
        .load_texture(Path::new("resources/doodads/arrow.png"))
        .unwrap();

    let texture3 = texture_creator
        .load_texture(Path::new("resources/doodads/foo.png"))
        .unwrap();

    let active = &mut dest_rect;
    let mut array = [
        Point::new(0, 0),
        Point::new(0, 0),
        Point::new(0, 0),
        Point::new(0, 0),
    ];

    let mut fragments: Vec<Box<Fragment>> = Vec::new();

    fragments.push(Box::new(Spritesheet::new(&texture, 400, 20, 6)));
    fragments.push(Box::new(Doodad::new(&texture2, 100, 100, 6)));
    fragments.push(Box::new(Doodad::new(&texture3, 100, 100, 6)));

    let mut holding_button = false;

    let mut main_ui = UserInterface::new();

    let mut frame = 0;

    'running: loop {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        let tempx = if active.x != 0 { active.x } else { 1 } as f32 / main_ui.scale;
        let tempy = if active.y != 0 { active.y } else { 1 } as f32 / main_ui.scale;

        let temp_rect = Rect::new(tempx as i32, tempy as i32, active.width(), active.height());

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
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    main_ui.scale = 1.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    main_ui.scale = 2.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    main_ui.scale = 3.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    main_ui.scale = 4.0;
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let check = check_rect2(array, Point::new(x, y));
                    println!("check: {}, (x, y): ({}, {})", check, x, y);

                    holding_button = true;
                }

                Event::MouseButtonUp { .. } => {
                    holding_button = false;
                }

                Event::MouseMotion { xrel, yrel, .. } => {
                    if holding_button {
                        let current_x = active.x;
                        let current_y = active.y;

                        active.set_x(current_x + xrel);
                        active.set_y(current_y + yrel);
                    }
                }
                _ => {}
            }
        }

        if frame != main_ui.frame() {
            for x in fragments.iter_mut() {
                x.next_frame();
            }
            frame = main_ui.frame();
        }

        if main_ui.play {
            source_rect.set_x(
                sprite_tile_size.0 as i32
                    * ((main_ui.frame_timer.get_elapsed() as i32 / main_ui.frame_time)
                        % main_ui.frames_per_anim),
            );
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 200, 20));
        canvas.clear();

        canvas.set_scale(main_ui.scale, main_ui.scale).unwrap();

        canvas
            .copy_ex(
                &texture,
                Some(source_rect),
                Some(temp_rect),
                main_ui.rotation.into(),
                None,
                false,
                false,
            )
            .unwrap();

        for fragment in &fragments {
            canvas
                .set_scale(fragment.get_scale(), fragment.get_scale())
                .unwrap();

            canvas
                .copy_ex(
                    &fragment.get_texture(),
                    Some(fragment.get_source_rect()),
                    Some(fragment.draw_position()),
                    fragment.get_rotation(),
                    None,
                    false,
                    false,
                )
                .unwrap();
        }

        // RED RECT
        canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 20, 20));
        array = draw_rectangle_around_active(&mut canvas, *active, main_ui.rotation, main_ui.scale);

        let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);

        main_ui.draw_window(&ui);

        canvas.window_mut().gl_make_current(&gl_context).unwrap();
        renderer.render(ui);

        &canvas.present();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
