use imgui::*;
pub mod timer;
use self::timer::Timer;

pub mod main_menu;
pub use self::main_menu::{MainMenuCommand, MainMenuInterface};

#[derive(Clone)]
pub enum MainInterfaceCommand {
    None,
    Scale(f32),
    Rotate(f32),
    Frame(i32),
}

pub trait UserInterface {
    fn draw_window(&mut self, ui: &Ui);
}

pub struct MainInterface {
    pub scale: f32,
    rotation: f32,
    current_frame: i32,
    play: bool,

    frame_timer: Timer,
    frames_per_anim: i32,
    framerate: i32,

    pub did_change: bool,

    command: MainInterfaceCommand,
}

impl MainInterface {
    pub fn new() -> Self {
        MainInterface {
            scale: 1.0,
            rotation: 0.0,
            current_frame: 0,
            play: false,

            frame_timer: Timer::create(),
            frames_per_anim: 6,
            framerate: 1000,

            did_change: false,

            command: MainInterfaceCommand::None,
        }
    }

    pub fn update(&mut self) {
        if self.play {
            if self.frame_timer.did_pass(self.framerate as u64) {
                self.current_frame = (self.current_frame + 1) % self.frames_per_anim;
                self.frame_timer.reset();
            }
            self.command = MainInterfaceCommand::Frame(self.current_frame);
        }
    }

    pub fn change_settings(&mut self, scale: f32, rotation: f32) {
        self.scale = scale;
        self.rotation = rotation;
    }

    pub fn reset(&mut self, frames: i32) {
        self.frame_timer.reset();
        self.play = false;
        self.current_frame = 0;
        self.frames_per_anim = frames;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_framerate(&mut self, framerate: i32) {
        self.framerate = framerate;
    }

    pub fn get_framerate(&self) -> i32 {
        self.framerate
    }

    pub fn check(&mut self) -> MainInterfaceCommand {
        let command = self.command.clone();

        self.command = MainInterfaceCommand::None;

        command
    }
}

impl UserInterface for MainInterface {
    fn draw_window(&mut self, ui: &Ui) {
        ui.window(im_str!("Main Panel"))
            .size((300.0, 500.0), ImGuiCond::Once)
            .position((400.0, 140.0), ImGuiCond::Once)
            .build(|| {
                if ui
                    .slider_float(im_str!("scale"), &mut self.scale, 0.5, 6.0) //needs parametrization
                    .build()
                {
                    self.did_change = true;
                    self.command = MainInterfaceCommand::Scale(self.scale);
                }

                ui.separator();

                if ui
                    .slider_float(im_str!("rotation"), &mut self.rotation, 0.0, 360.0)
                    .build()
                {
                    self.did_change = true;
                    self.command = MainInterfaceCommand::Rotate(self.rotation);
                }

                ui.separator();

                if ui
                    .slider_int(
                        im_str!("Frame:"),
                        &mut self.current_frame,
                        0,
                        self.frames_per_anim - 1,
                    )
                    .build()
                {
                    self.frame_timer.reset();
                    self.did_change = true;
                    self.play = false;
                    self.command = MainInterfaceCommand::Frame(self.current_frame);
                }

                if ui
                    .input_int(im_str!("framerate"), &mut self.framerate)
                    .chars_decimal(true)
                    .build()
                {
                    self.frame_timer.reset(); //TODO: Check if necessary
                }

                if ui.checkbox(im_str!("play"), &mut self.play) {
                    self.frame_timer.reset();
                    self.current_frame = 0;

                    self.command = MainInterfaceCommand::Frame(0);
                }
            });
    }
}
