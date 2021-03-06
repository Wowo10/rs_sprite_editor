use lib::ImguiSdl2;

use sdl2::image::{InitFlag};
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
    holding_index: bool,
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
            holding_index: false,
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

        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("Counld not init SDL Image.");

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
        let default_name = self.config.read("starting_filename");
        let default_filename = default_name.clone() + ".png";

        let mut spritesheet = Spritesheet::new(
            default_name.clone(),
            manager.get_spritesheet(&default_filename),
            default_x,
            default_y,
            default_frames,
        );
        self.main_ui.reset(default_frames as i32);

        let mut doodads: Vec<Doodad> = Vec::new();

        while !self.exit {
            use sdl2::event::Event;

            for event in event_pump.poll_iter() {
                imgui_sdl2.handle_event(&mut imgui, &event);
                if imgui_sdl2.ignore_event(&event) {
                    continue;
                }

                match event {
                    Event::Quit { .. } => self.exit = true,
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
                                self.holding_index = true; //i as i32; also -> probably will move to bool
                                App::bring_to_front(&mut doodads, i);
                                break;
                            }
                        }

                        self.holding_button = true;
                    }

                    Event::MouseButtonUp { .. } => {
                        self.holding_button = false;
                        self.holding_index = false;
                    }

                    Event::MouseMotion { xrel, yrel, .. } => {
                        if self.holding_button {
                            if self.holding_index {
                                doodads.first_mut().unwrap().change_position(xrel, yrel);
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
                    match doodads.first_mut() {
                        Some(first_doodad) => {
                            first_doodad.set_scale(scale);
                        }
                        None => {}
                    };
                }
                MainInterfaceCommand::Rotate(angle) => {
                    match doodads.first_mut() {
                        Some(first_doodad) => {
                            first_doodad.set_rotation(angle.into());
                        }
                        None => {}
                    };
                }
                MainInterfaceCommand::Frame(frame) => {
                    for doodad in &mut doodads {
                        doodad.set_frame(frame as usize);
                    }
                    spritesheet.set_frame(frame as usize);
                    self.frame = frame;

                    match doodads.first() {
                        Some(first_doodad) => {
                            self.main_ui
                                .set_rotation(first_doodad.get_rotation() as f32);
                        }
                        None => {}
                    };
                }
                _ => {}
            }

            match self.main_menu_ui.check() {
                MainMenuCommand::New => {
                    doodads.clear();

                    spritesheet = Spritesheet::new(
                        default_name.clone(),
                        manager.get_spritesheet(&default_filename),
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
                    doodads.clear();

                    for line in lines.iter().skip(1) {
                        let split = file_utils::split_line(line, ";"); //name;scale;posx,posy,rot/...

                        let texture = manager.get_doodad(&(split[0].clone() + ".png"));
                        let width = texture.query().width;
                        let height = texture.query().height;

                        let mut pos_vec: Vec<sdl2::rect::Rect> = Vec::new();
                        let mut rot_vec: Vec<f64> = Vec::new();

                        let split_positions = file_utils::split_line(&split[2], "/"); //data/data/data

                        let split_positions: Vec<&String> = split_positions
                            .iter()
                            .filter(|position| position.len() != 0)
                            .collect();

                        for split_position in split_positions {
                            let pos_data = file_utils::split_line(&split_position, ","); //posx,posy,rot

                            let temp_x = default_x + pos_data[0].parse::<i32>().unwrap();
                            let temp_y = default_y + pos_data[1].parse::<i32>().unwrap();
                            let temp_rot = pos_data[2].parse::<f64>().unwrap();

                            pos_vec.push(sdl2::rect::Rect::new(temp_x, temp_y, width, height));
                            rot_vec.push(temp_rot);
                        }

                        let doodad = Doodad::load(
                            split[0].clone(),
                            texture,
                            pos_vec,
                            rot_vec,
                            split[1].parse::<f32>().unwrap(),
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
