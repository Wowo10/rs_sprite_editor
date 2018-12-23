use imgui::*;
pub mod timer;
use self::timer::*;

pub struct UserInterface {
    pub scale: f32,
    pub rotation: f32,
    pub current_frame: i32,
    pub play: bool,

    pub frame_timer: Timer,
    pub frame_time: i32,
    pub frames_per_anim: i32,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            scale: 1.0,
            rotation: 0.0,
            current_frame: 0,
            play: false,

            frame_timer: Timer::create(),
            frame_time: 1000,
            frames_per_anim: 6,
        }
    }

    pub fn draw_window(&mut self, ui: &Ui) {
        ui.window(im_str!("Main Panel"))
            .size((300.0, 500.0), ImGuiCond::Appearing)
            .position((400.0, 140.0), ImGuiCond::Appearing)
            .build(|| {
                ui.text(im_str!("A Panel wow!"));
                ui.separator();

                ui.slider_float(im_str!("scale"), &mut self.scale, 0.5, 6.0)
                    .build();

                ui.separator();

                ui.slider_float(im_str!("rotation"), &mut self.rotation, 0.0, 360.0)
                    .build();

                ui.separator();
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));

                ui.slider_int(
                    im_str!("Frame:"),
                    &mut self.current_frame,
                    0,
                    self.frames_per_anim - 1,
                )
                .build();

                if ui.checkbox(im_str!("play"), &mut self.play) {
                    self.frame_timer.reset();
                }
            });
    }

    pub fn frame(&mut self) -> i32 {
        self.current_frame = (self.frame_timer.get_elapsed() / self.frame_time as u64) as i32 % self.frames_per_anim;

        self.current_frame
    }
}
