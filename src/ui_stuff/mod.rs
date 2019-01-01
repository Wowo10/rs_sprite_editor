use imgui::*;
pub mod timer;
use self::timer::*;

pub mod main_menu;
pub use self::main_menu::{MainMenuCommand, MainMenuInterface};

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
    did_change_play: bool,
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
            did_change_play: false,
        }
    }

    pub fn frame(&mut self) -> i32 {
        if self.frame_timer.did_pass(self.framerate as u64){
            self.current_frame = (self.current_frame+1)% self.frames_per_anim;
            self.frame_timer.reset();
        }

        self.current_frame
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

    pub fn update_check(&mut self) -> (bool, bool) {
        let did_change: (bool, bool) = (self.did_change, self.did_change_play);

        if self.did_change {
            self.did_change = false;
        }

        if self.did_change_play {
            self.did_change_play = false;
        }

        did_change
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
    pub fn get_frame(&self) -> usize {
        self.current_frame as usize
    }
    pub fn play(&self) -> bool {
        self.play
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
                }

                ui.separator();

                if ui
                    .slider_float(im_str!("rotation"), &mut self.rotation, 0.0, 360.0)
                    .build()
                {
                    self.did_change = true;
                }

                ui.separator();

                let mouse_pos = ui.imgui().mouse_pos(); //Debug Purposes
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));

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
                }

                if ui
                    .input_int(im_str!("framerate"), &mut self.framerate)
                    .chars_decimal(true)
                    .build()
                {
                    self.frame_timer.reset(); //TODO: Check if necessary
                    self.did_change = true;
                }

                if ui.checkbox(im_str!("play"), &mut self.play) {
                    self.frame_timer.reset();
                    self.current_frame = 0;
                    self.did_change_play = true;
                }
            });
    }
}
