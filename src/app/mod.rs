use lib::ImguiSdl2;

use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::pixels::Color;
use sdl2::rect::Point;

use config::Config;
use fragment::{Doodad, Fragment, Spritesheet};
use mymath::{check_rect, rotate_rectangle};
use resource_manager::ResourceManager;
use ui_stuff::{
    MainInterface, MainInterfaceCommand, MainMenuCommand, MainMenuInterface, UserInterface,
};

use file_utils;

pub struct App {
    exit: bool,
    config: Config,
    sdl_ctx: sdl2::Sdl,
    video: sdl2::VideoSubsystem,

    main_ui: MainInterface,
    main_menu_ui: MainMenuInterface,

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

    fn bring_to_front(vec: &mut Vec<Doodad>, index: usize) {
        let el = vec.remove(index);
        vec.insert(0, el);
    }

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

        let default_x = self
            .config
            .read("starting_x_position")
            .parse::<i32>()
            .unwrap();
        let default_y = self
            .config
            .read("starting_y_position")
            .parse::<i32>()
            .unwrap();
        let default_frames = self
            .config
            .read("starting_frames")
            .parse::<usize>()
            .unwrap();
        let default_name = self.config.read("starting_filename") + ".png";

        let mut spritesheet = Spritesheet::new(
            default_name.clone(),
            manager.get_spritesheet(&default_name),
            default_x,
            default_y,
            default_frames,
        );
        self.main_ui.reset(default_frames as i32);

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
                                doodad.serialize(spritesheet.real_position().top_left())
                            );
                        }
                        // println!("serialize2");
                        // println!("{}", spritesheet.serialize());
                        // for doodad in &doodads {
                        //     println!(
                        //         "{}",
                        //         doodad.serialize2(spritesheet.real_position().top_left())
                        //     );
                        // }
                        // println!("serialize3");
                        // println!("{}", spritesheet.serialize());
                        // for doodad in &doodads {
                        //     println!(
                        //         "{}",
                        //         doodad.serialize3(spritesheet.real_position().top_left())
                        //     );
                        // }
                    }
                    Event::MouseButtonDown { x, y, .. } => {
                        for i in 0..doodads.len() {
                            let check = check_rect(
                                rotate_rectangle(
                                    doodads[i].real_position(),
                                    doodads[i].get_rotation() as f32,
                                    doodads[i].get_scale(),
                                ),
                                Point::new(x, y),
                            );

                            if check {
                                self.main_ui.change_settings(
                                    doodads[i].get_scale(),
                                    doodads[i].get_rotation() as f32,
                                );
                                self.holding_index = 0; //i as i32; also -> probably will move to bool
                                App::bring_to_front(&mut doodads, i);
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

            self.main_ui.update();

            match self.main_ui.check() {
                MainInterfaceCommand::Scale(scale) => {
                    doodads.first_mut().unwrap().set_scale(scale);
                }
                MainInterfaceCommand::Rotate(angle) => {
                    doodads.first_mut().unwrap().set_rotation(angle.into());
                }
                MainInterfaceCommand::Frame(frame) => {
                    for doodad in &mut doodads {
                        doodad.set_frame(frame as usize);
                    }
                    spritesheet.set_frame(frame as usize);
                    self.frame = frame;
                    if doodads.len() != 0 {
                        self.main_ui
                            .set_rotation(doodads.first().unwrap().get_rotation() as f32);
                    }
                }
                _ => {}
            }

            match self.main_menu_ui.check() {
                MainMenuCommand::New => {
                    doodads.clear();

                    spritesheet = Spritesheet::new(
                        default_name.clone(),
                        manager.get_spritesheet(&default_name),
                        default_x,
                        default_y,
                        default_frames,
                    );
                    self.main_ui.reset(default_frames as i32);
                }
                MainMenuCommand::Save(path) => {
                    let mut temp_string = String::new();

                    temp_string += &spritesheet.serialize();
                    temp_string += &(self.main_ui.get_framerate().to_string());

                    for doodad in &doodads {
                        temp_string += "\n";
                        temp_string += &doodad.serialize(spritesheet.real_position().top_left());
                    }

                    file_utils::save_template(path, temp_string);
                }
                MainMenuCommand::Load(path) => {
                    let lines = file_utils::load_file_by_lines(path);

                    let first = lines.first().unwrap();

                    let split = file_utils::split_line(first, ";"); //name;frames;framerate

                    let frames = split[1].parse::<usize>().unwrap();

                    spritesheet = Spritesheet::new(
                        split[0].clone(),
                        manager.get_spritesheet(&(split[0].clone() + ".png")),
                        default_x,
                        default_y,
                        frames,
                    );
                    
                    self.main_ui.set_framerate(split[2].parse::<i32>().unwrap());

                    self.main_ui.reset(frames as i32);

                    for line in lines.iter().skip(1){
                        println!("{}", line);

                        let split = file_utils::split_line(line, ";");//name;scale;pos_x,pos_y,rot/...

                        let doodad = Doodad::new(
                            split[0].clone(),
                            manager.get_doodad(&(split[0].clone() + ".png")),
                            default_x,
                            default_y,
                            frames as u32
                        );

                        doodads.push(doodad);
                    }
                }
                MainMenuCommand::Exit => {
                    self.exit = true;
                }
                MainMenuCommand::AddDoodad(name) => {
                    let name_clone = name.clone();

                    let texture = manager.get_doodad(&(name + ".png"));

                    doodads.push(Doodad::new(
                        name_clone,
                        texture,
                        default_x,
                        default_y,
                        spritesheet.get_frames_amount() as u32,
                    ));
                }
                MainMenuCommand::ClearDoodads => {
                    doodads.clear();
                }
                MainMenuCommand::ChangeSpritesheet(name, frames) => {
                    let name_clone = name.clone();

                    let texture = manager.get_spritesheet(&(name + ".png"));

                    let position = spritesheet.real_position();

                    spritesheet = Spritesheet::new(
                        name_clone,
                        texture,
                        position.x,
                        position.y,
                        frames as usize,
                    );

                    for doodad in &mut doodads {
                        doodad.set_frames_amount(frames.into());
                    }

                    self.main_ui.reset(frames as i32);
                }
                _ => {}
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

            for doodad in doodads.iter().rev() {
                canvas
                    .set_scale(doodad.get_scale(), doodad.get_scale())
                    .unwrap();

                canvas
                    .copy_ex(
                        doodad.get_texture(),
                        Some(doodad.get_source_rect()),
                        Some(doodad.draw_position()),
                        doodad.get_rotation(),
                        None,
                        false,
                        false,
                    )
                    .unwrap();
            }

            if doodads.len() != 0 {
                let first = doodads.first().unwrap();

                App::draw_rectangle_around_active(
                    &mut canvas,
                    rotate_rectangle(
                        first.real_position(),
                        first.get_rotation() as f32,
                        first.get_scale(),
                    ),
                );
            }

            let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);

            self.main_ui.draw_window(&ui);
            self.main_menu_ui.draw_window(&ui);

            canvas.window_mut().gl_make_current(&gl_context).unwrap();
            imgui_renderer.render(ui);

            &canvas.present();

            ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
