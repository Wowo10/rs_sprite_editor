extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate sdl2;

mod lib;
use lib::*;

use sdl2::pixels::Color;
use sdl2::rect::Point;

use sdl2::image::{INIT_JPG, INIT_PNG};

mod fragment;
use fragment::*;

mod mymath;
use mymath::{check_rect2, rotate_rectangle};

mod file_utils;

mod ui_stuff;
use ui_stuff::*;

mod config;
use config::Config;

mod resource_manager;
use resource_manager::*;

fn draw_rectangle_around_active(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    points: [Point; 4],
) {
    canvas.set_draw_color(Color::RGB(200, 20, 20));
    canvas.set_scale(1.0, 1.0).unwrap();

    canvas.draw_line(points[0], points[1]).unwrap();
    canvas.draw_line(points[1], points[2]).unwrap();
    canvas.draw_line(points[2], points[3]).unwrap();
    canvas.draw_line(points[3], points[0]).unwrap();
}

fn main() {
    let mut config = Config::create("./usr/config.csv");

    let width = config.read("width").parse::<u32>().unwrap();
    let height = config.read("height").parse::<u32>().unwrap();

    let backgound_color = config.read_color("background_color");

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
        .window("rust-imgui-sdl2 demo", width, height)
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

    let imgui_renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    let mut event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(err) => panic!("SDL could not get event_pump: {}.", err),
    };

    let mut canvas = match window.into_canvas().accelerated().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("SDL could not convart into canvas: {}.", err),
    };

    //do not touch me
    let texture_creator = canvas.texture_creator();

    let mut manager = ResourceManager::new(&texture_creator);

    let texture = manager.get_spritesheet("animbg.png");

    let texture2 = manager.get_doodad("arrow.png");

    let texture3 = manager.get_doodad("foo.png");

    let mut spritesheet = Spritesheet::new(&texture, 400, 20, 6);

    let mut doodads: Vec<Doodad> = Vec::new();

    doodads.push(Doodad::new(&texture2, 100, 100, 6));
    doodads.push(Doodad::new(&texture3, 100, 100, 6));

    let mut active_fragment = 0;

    let mut holding_button = false;
    let mut holding_index: i32 = -1;

    let mut main_ui = MainInterface::new();
    let mut main_menu_ui = ui_stuff::MainMenuInterface::new();

    let mut frame = 0;

    while !main_menu_ui.exit {
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
                } => main_menu_ui.exit = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    main_ui.scale = 1.0;
                    main_ui.did_change = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    main_ui.scale = 2.0;
                    main_ui.did_change = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    main_ui.scale = 3.0;
                    main_ui.did_change = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    main_ui.scale = 4.0;
                    main_ui.did_change = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    println!("//////////////////////");
                    println!("serialize1");
                    println!("{}", spritesheet.serialize());
                    for doodad in &doodads {
                        println!(
                            "{}",
                            doodad.serialize(spritesheet.get_position().top_left())
                        );
                    }
                    println!("serialize2");
                    println!("{}", spritesheet.serialize());
                    for doodad in &doodads {
                        println!(
                            "{}",
                            doodad.serialize2(spritesheet.get_position().top_left())
                        );
                    }
                    println!("serialize3");
                    println!("{}", spritesheet.serialize());
                    for doodad in &doodads {
                        println!(
                            "{}",
                            doodad.serialize3(spritesheet.get_position().top_left())
                        );
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    for i in 0..doodads.len() {
                        //TODO: Add Checking if pixel is not (0,0,0,1)
                        //poor pixel perfect
                        let check = check_rect2(
                            rotate_rectangle(
                                doodads[i].real_position(),
                                doodads[i].get_rotation() as f32,
                                doodads[i].get_scale(),
                            ),
                            Point::new(x, y),
                        );

                        if check {
                            active_fragment = i;
                            main_ui.change_settings(
                                doodads[i].get_scale(),
                                doodads[i].get_rotation() as f32,
                            );
                            holding_index = i as i32;
                            break;
                        }
                    }

                    holding_button = true;
                }

                Event::MouseButtonUp { .. } => {
                    holding_button = false;
                    holding_index = -1;
                }

                Event::MouseMotion { xrel, yrel, .. } => {
                    if holding_button {
                        if holding_index != -1 {
                            doodads[holding_index as usize].change_position(xrel, yrel);
                        } else {
                            spritesheet.change_position(xrel, yrel);
                            for doodad in &mut doodads {
                                doodad.change_all_positions(xrel, yrel);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let check = main_ui.update_check();

        if check.0 {
            doodads[active_fragment].set_rotation(main_ui.get_rotation().into());
            doodads[active_fragment].set_scale(main_ui.get_scale());

            let frame = main_ui.get_frame();

            for fragment in &mut doodads {
                fragment.set_frame(frame);
            }
            spritesheet.set_frame(frame);
        }

        if check.1 {
            for fragment in &mut doodads {
                fragment.reset_frames();
            }
            spritesheet.reset_frames();
        }

        if main_ui.play() && frame != main_ui.frame() {
            for fragment in &mut doodads {
                fragment.next_frame();
            }
            spritesheet.next_frame();
            frame = main_ui.frame();
        }

        canvas.set_draw_color(backgound_color);
        canvas.clear();

        canvas
            .set_scale(spritesheet.get_scale(), spritesheet.get_scale())
            .unwrap();

        canvas
            .copy_ex(
                &spritesheet.get_texture(),
                Some(spritesheet.get_source_rect()),
                Some(spritesheet.draw_position()),
                spritesheet.get_rotation(),
                None,
                false,
                false,
            )
            .unwrap();

        for fragment in &doodads {
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

        draw_rectangle_around_active(
            &mut canvas,
            rotate_rectangle(
                doodads[active_fragment].real_position(),
                main_ui.get_rotation(),
                main_ui.get_scale(),
            ),
        );

        let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);

        main_ui.draw_window(&ui);
        main_menu_ui.draw_window(&ui);

        ui.show_demo_window(&mut true);

        canvas.window_mut().gl_make_current(&gl_context).unwrap();
        imgui_renderer.render(ui);

        &canvas.present();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
