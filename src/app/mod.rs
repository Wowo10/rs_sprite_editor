use lib::*;

use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::pixels::Color;
use sdl2::rect::Point;

use fragment::*;

use mymath::{check_rect2, rotate_rectangle};

use ui_stuff::*;

use config::Config;

use resource_manager::*;

pub struct App {
    exit: bool,
    config: Config,
    sdl_ctx: sdl2::Sdl,
    video: sdl2::VideoSubsystem,

    main_ui: MainInterface,
    main_menu_ui: MainMenuInterface,

    active_doodad: usize,
    holding_button: bool,
    holding_index: i32,
    frame: i32,
}

impl App {
    pub fn new() -> Self {
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

        App {
            exit: false,
            config: Config::create("./usr/config.csv"),
            sdl_ctx: sdl_context,
            video: video,

            main_ui: MainInterface::new(),
            main_menu_ui: MainMenuInterface::new(),

            active_doodad: 0,
            holding_button: false,
            holding_index: -1,
            frame: 0,
        }
    }

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

    // fn handle_main_menu_command(
    //     &mut self,
    //     command: MainMenuCommand,
    //     manager: &ResourceManager,
    //     spritesheet: &Spritesheet,
    //     doodads: &Vec<Doodad>,
    // ) {
    //     match command {
    //         MainMenuCommand::Exit => {
    //             self.exit = true;
    //         }
    //         MainMenuCommand::AddDoodad(name) => {

    //             let texture = manager.get_doodad("");

    //             doodads.push(Doodad::new(texture, 100, 100, 10));
    //         }
    //         _ => {}
    //     }
    // }

    pub fn run(&mut self) {
        let width = self.config.read("width").parse::<u32>().unwrap();
        let height = self.config.read("height").parse::<u32>().unwrap();

        let window = match self
            .video
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

        gl::load_with(|s| self.video.gl_get_proc_address(s) as _);

        let mut imgui = imgui::ImGui::init();
        imgui.set_ini_filename(Some(imgui::ImString::new("imgui.ini")));
        imgui.set_log_filename(Some(imgui::ImString::new("imgui.log")));

        let mut imgui_sdl2 = ImguiSdl2::new(&mut imgui);

        let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            self.video.gl_get_proc_address(s) as _
        });

        let mut event_pump = match self.sdl_ctx.event_pump() {
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

        let texture = manager.get_spritesheet("dummy.png");

        let mut spritesheet = Spritesheet::new(texture, 400, 20, 10);

        let mut doodads: Vec<Doodad> = Vec::new();

        while !self.exit {
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
                    } => self.exit = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num1),
                        ..
                    } => {
                        self.main_ui.scale = 1.0;
                        self.main_ui.did_change = true;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num2),
                        ..
                    } => {
                        self.main_ui.scale = 2.0;
                        self.main_ui.did_change = true;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num3),
                        ..
                    } => {
                        self.main_ui.scale = 3.0;
                        self.main_ui.did_change = true;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num4),
                        ..
                    } => {
                        self.main_ui.scale = 4.0;
                        self.main_ui.did_change = true;
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
                                self.active_doodad = i;
                                self.main_ui.change_settings(
                                    doodads[i].get_scale(),
                                    doodads[i].get_rotation() as f32,
                                );
                                self.holding_index = i as i32;
                                break;
                            }
                        }

                        self.holding_button = true;
                    }

                    Event::MouseButtonUp { .. } => {
                        self.holding_button = false;
                        self.holding_index = -1;
                    }

                    Event::MouseMotion { xrel, yrel, .. } => {
                        if self.holding_button {
                            if self.holding_index != -1 {
                                doodads[self.holding_index as usize].change_position(xrel, yrel);
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

            let check = self.main_ui.update_check();

            if check.0 && doodads.len() != 0 {
                doodads[self.active_doodad].set_rotation(self.main_ui.get_rotation().into());
                doodads[self.active_doodad].set_scale(self.main_ui.get_scale());

                let frame = self.main_ui.get_frame();

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

            if self.main_ui.play() && self.frame != self.main_ui.frame() {
                for fragment in &mut doodads {
                    fragment.next_frame();
                }
                spritesheet.next_frame();
                self.frame = self.main_ui.frame();
            }

            canvas.set_draw_color(self.config.read_color("background_color"));
            canvas.clear();

            canvas
                .set_scale(spritesheet.get_scale(), spritesheet.get_scale())
                .unwrap();

            canvas
                .copy_ex(
                    spritesheet.get_texture(),
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
                        fragment.get_texture(),
                        Some(fragment.get_source_rect()),
                        Some(fragment.draw_position()),
                        fragment.get_rotation(),
                        None,
                        false,
                        false,
                    )
                    .unwrap();
            }

            if doodads.len() != 0 {
                App::draw_rectangle_around_active(
                    &mut canvas,
                    rotate_rectangle(
                        doodads[self.active_doodad].real_position(),
                        self.main_ui.get_rotation(),
                        self.main_ui.get_scale(),
                    ),
                );
            }

            let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);

            self.main_ui.draw_window(&ui);
            self.main_menu_ui.draw_window(&ui);

            let command = self.main_menu_ui.check();

            // self.handle_main_menu_command(command, &manager, &spritesheet, &doodads);

            match command {
                MainMenuCommand::Exit => {
                    self.exit = true;
                }
                MainMenuCommand::AddDoodad(name) => {
                    let texture = manager.get_doodad(&(name + ".png"));

                    doodads.push(Doodad::new(texture, 100, 100, 10));
                }
                _ => {}
            }

            ui.show_demo_window(&mut true);

            canvas.window_mut().gl_make_current(&gl_context).unwrap();
            imgui_renderer.render(ui);

            &canvas.present();

            ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
