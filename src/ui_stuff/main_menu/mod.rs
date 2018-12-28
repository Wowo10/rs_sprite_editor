use ui_stuff::*; //{im_str, ImGuiCond, Ui, UserInterface, ImString, ImVec2};

enum WindowVisible {
    None,
    New,
    Load,
    Save,
    AddDoodad,
    ChangeSpritesheet,
}

pub struct MainMenuInterface {
    window: WindowVisible,
    pub exit: bool,

    text_input: ImString,
}

impl MainMenuInterface {
    pub fn new() -> Self {
        MainMenuInterface {
            window: WindowVisible::None,
            exit: false,

            text_input: ImString::with_capacity(32),
        }
    }

    pub fn reset(&mut self) {
        self.text_input.clear();
    }
}

impl UserInterface for MainMenuInterface {
    fn draw_window(&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File")).build(|| {
                if ui.menu_item(im_str!("New")).build() {
                    self.reset();
                    self.window = WindowVisible::New;
                }
                if ui.menu_item(im_str!("Load")).build() {
                    self.reset();
                    self.window = WindowVisible::Load;
                }
                if ui.menu_item(im_str!("Save")).build() {
                    self.reset();
                    self.window = WindowVisible::Save;
                }
                if ui.menu_item(im_str!("Exit")).build() {
                    self.exit = true;
                }
            });

            ui.menu(im_str!("Fragments")).build(|| {
                if ui.menu_item(im_str!("Add Doodad")).build() {
                    self.reset();
                    self.window = WindowVisible::AddDoodad;
                }
                if ui.menu_item(im_str!("Change SpriteSheet")).build() {
                    self.reset();
                    self.window = WindowVisible::ChangeSpritesheet;
                }
            });
        });

        match self.window {
            WindowVisible::New => {
                ui.window(im_str!("Warning!"))
                    .size((300.0, 100.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.text("No Autosave, are you sure?");

                        ui.separator();

                        if ui.button(im_str!("Yes!!"), ImVec2::new(0.0, 0.0)) {
                            println!("New: ");
                        }
                    });
            }

            WindowVisible::Load => {
                ui.window(im_str!("Load File"))
                    .size((300.0, 100.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.text("Here will be File Browser -> WiP");

                        ui.separator();

                        if ui.button(im_str!("Load!"), ImVec2::new(0.0, 0.0)) {
                            println!("Load: ");
                        }
                    });
            }

            WindowVisible::Save => {
                ui.window(im_str!("Save File"))
                    .size((300.0, 100.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.input_text(im_str!("Filename"), &mut self.text_input)
                            .build();

                        if ui.button(im_str!("Save!"), ImVec2::new(0.0, 0.0)) {
                            println!("Save: {:?}", self.text_input);
                        }
                    });
            }

            WindowVisible::AddDoodad => {
                ui.window(im_str!("Doodad choose"))
                    .size((300.0, 100.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.text("Here will be File Browser -> WiP");

                        ui.separator();

                        if ui.button(im_str!("Add!"), ImVec2::new(0.0, 0.0)) {
                            println!("Add: ");
                        }
                    });
            }

            WindowVisible::ChangeSpritesheet => {
                ui.window(im_str!("SpriteSheet choose"))
                    .size((300.0, 100.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.text("Here will be File Browser -> WiP");

                        ui.separator();

                        ui.input_text(im_str!("frames"), &mut self.text_input)
                            .chars_decimal(true)
                            .build();

                        ui.separator();

                        if ui.button(im_str!("Change!"), ImVec2::new(0.0, 0.0)) {
                            println!("Change: {:?}", self.text_input);
                        }
                    });
            }

            _ => {}
        }
    }
}