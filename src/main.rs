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
use mymath::{check_rect2, rotate_rectangle};

//use ui_stuff::timer::Timer;

mod ui_stuff;
use ui_stuff::*;

fn draw_rectangle_around_active(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    points: [Point; 4],
) {
    canvas.set_scale(1.0, 1.0).unwrap();

    canvas.draw_line(points[0], points[1]).unwrap();
    canvas.draw_line(points[1], points[2]).unwrap();
    canvas.draw_line(points[2], points[3]).unwrap();
    canvas.draw_line(points[3], points[0]).unwrap();
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
    imgui.set_ini_filename(Some(imgui::ImString::new("imgui.ini")));
    imgui.set_log_filename(Some(imgui::ImString::new("imgui.log")));

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

    let texture2 = texture_creator
        .load_texture(Path::new("resources/doodads/arrow.png"))
        .unwrap();

    let texture3 = texture_creator
        .load_texture(Path::new("resources/doodads/foo.png"))
        .unwrap();

    let mut fragments: Vec<Box<Fragment>> = Vec::new();

    fragments.push(Box::new(Spritesheet::new(&texture, 400, 20, 6)));
    fragments.push(Box::new(Doodad::new(&texture2, 100, 100, 6)));
    fragments.push(Box::new(Doodad::new(&texture3, 100, 100, 6)));

    let active = &mut fragments[0].draw_position();

    //let active_fragment = &fragments.first();
    let active_fragment = 0;

    let mut holding_button = false;

    let mut main_ui = UserInterface::new();

    let mut frame = 0;

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
                    println!("(x, y): ({}, {})", x, y);

                    for fragment in &fragments {
                        let check = check_rect2(
                            rotate_rectangle(
                                fragment.draw_position(),
                                fragment.get_rotation() as f32,
                                fragment.get_scale(),
                            ),
                            Point::new(x, y),
                        );

                        println!("check {}", check);
                    }

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
            for fragment in fragments.iter_mut() {
                fragment.next_frame();
            }
            frame = main_ui.frame();
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(20, 200, 20));
        canvas.clear();

        canvas.set_scale(main_ui.scale, main_ui.scale).unwrap();

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
        draw_rectangle_around_active(
            &mut canvas,
            rotate_rectangle(
                fragments[active_fragment].draw_position(),
                main_ui.rotation,
                main_ui.scale,
            ),
        );

        let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);

        main_ui.draw_window(&ui);

        canvas.window_mut().gl_make_current(&gl_context).unwrap();
        renderer.render(ui);

        &canvas.present();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
