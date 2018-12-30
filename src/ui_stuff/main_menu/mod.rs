use file_utils;
use ui_stuff::*;

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

    selected: usize,
    list_directory: Vec<ImString>,
}

impl MainMenuInterface {
    pub fn new() -> Self {
        MainMenuInterface {
            window: WindowVisible::None,
            exit: false,

            text_input: ImString::with_capacity(32),

            selected: 0,
            list_directory: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.text_input.clear();
        self.selected = 0;

        match self.window {
            WindowVisible::AddDoodad => {
                self.list_directory = file_utils::get_imgui_directory("./resources/doodads")
            }
            WindowVisible::ChangeSpritesheet => {
                self.list_directory = file_utils::get_imgui_directory("./resources/spritesheets")
            }
            WindowVisible::Load | WindowVisible::Save => {
                self.list_directory = file_utils::get_imgui_directory("./resources/definitions")
            }
            _ => {}
        }
    }
}

impl UserInterface for MainMenuInterface {
    fn draw_window(&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File")).build(|| {
                if ui.menu_item(im_str!("New")).build() {
                    self.window = WindowVisible::New;
                    self.reset();
                }
                if ui.menu_item(im_str!("Load")).build() {
                    self.window = WindowVisible::Load;
                    self.reset();
                }
                if ui.menu_item(im_str!("Save")).build() {
                    self.window = WindowVisible::Save;
                    self.reset();
                }
                if ui.menu_item(im_str!("Exit")).build() {
                    self.exit = true;
                }
            });

            ui.menu(im_str!("Fragments")).build(|| {
                if ui.menu_item(im_str!("Add Doodad")).build() {
                    self.window = WindowVisible::AddDoodad;
                    self.reset();
                }
                if ui.menu_item(im_str!("Change SpriteSheet")).build() {
                    self.window = WindowVisible::ChangeSpritesheet;
                    self.reset();
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
                            self.window = WindowVisible::None;
                        }
                    });
            }

            WindowVisible::Load => {
                ui.window(im_str!("Load File"))
                    .size((300.0, 400.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.child_frame(im_str!("child frame"), (280.0, 200.0))
                            .show_borders(true)
                            .always_show_vertical_scroll_bar(true)
                            .build(|| {
                                for i in 0..self.list_directory.len() {
                                    if ui.selectable(
                                        &self.list_directory[i],
                                        i == self.selected,
                                        ImGuiSelectableFlags::empty(),
                                        ImVec2::new(0.0, 0.0),
                                    ) {
                                        self.selected = i;
                                    }
                                }
                            });

                        ui.separator();

                        if ui.button(im_str!("Load!"), ImVec2::new(0.0, 0.0)) {
                            if self.selected < self.list_directory.len() {
                                println!("Load: ");
                                self.window = WindowVisible::None;
                            }
                        }
                    });
            }

            WindowVisible::Save => {
                ui.window(im_str!("Save File"))
                    .size((300.0, 400.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.child_frame(im_str!("child frame"), (280.0, 200.0))
                            .show_borders(true)
                            .always_show_vertical_scroll_bar(true)
                            .build(|| {
                                for i in 0..self.list_directory.len() {
                                    if ui.selectable(
                                        &self.list_directory[i],
                                        false,
                                        ImGuiSelectableFlags::empty(),
                                        ImVec2::new(0.0, 0.0),
                                    ){
                                        self.text_input = self.list_directory[i].clone();
                                    }
                                }
                            });

                        ui.input_text(im_str!("Filename"), &mut self.text_input)
                            .build();

                        if ui.button(im_str!("Save!"), ImVec2::new(0.0, 0.0)) {
                            if self.text_input != ImString::new("") {
                                println!("Save: {:?}", self.text_input);
                                self.window = WindowVisible::None;
                            }
                        }
                    });
            }

            WindowVisible::AddDoodad => {
                ui.window(im_str!("Doodad choose"))
                    .size((300.0, 400.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.child_frame(im_str!("child frame"), (280.0, 200.0))
                            .show_borders(true)
                            .always_show_vertical_scroll_bar(true)
                            .build(|| {
                                for i in 0..self.list_directory.len() {
                                    if ui.selectable(
                                        &self.list_directory[i],
                                        i == self.selected,
                                        ImGuiSelectableFlags::empty(),
                                        ImVec2::new(0.0, 0.0),
                                    ) {
                                        self.selected = i;
                                    }
                                }
                            });

                        ui.separator();

                        if ui.button(im_str!("Add!"), ImVec2::new(0.0, 0.0)) {
                            println!(
                                "Add to: {:?}, frames: {:?}",
                                self.list_directory[self.selected], self.text_input
                            );
                            self.window = WindowVisible::None;
                        }
                    });
            }

            WindowVisible::ChangeSpritesheet => {
                ui.window(im_str!("SpriteSheet choose"))
                    .size((300.0, 400.0), ImGuiCond::Once)
                    .position((100.0, 100.0), ImGuiCond::Once)
                    .build(|| {
                        ui.child_frame(im_str!("child frame"), (280.0, 200.0))
                            .show_borders(true)
                            .always_show_vertical_scroll_bar(true)
                            .build(|| {
                                for i in 0..self.list_directory.len() {
                                    if ui.selectable(
                                        &self.list_directory[i],
                                        i == self.selected,
                                        ImGuiSelectableFlags::empty(),
                                        ImVec2::new(0.0, 0.0),
                                    ) {
                                        self.selected = i;
                                    }
                                }
                            });

                        ui.separator();

                        ui.input_text(im_str!("frames"), &mut self.text_input)
                            .chars_decimal(true)
                            .build();

                        ui.separator();

                        if ui.button(im_str!("Change!"), ImVec2::new(0.0, 0.0)) {
                            if self.text_input != ImString::new("") {
                                println!(
                                    "Change to: {:?}, frames: {:?}",
                                    self.list_directory[self.selected], self.text_input
                                );
                                self.window = WindowVisible::None;
                            }
                        }
                    });
            }

            _ => {}
        }
    }
}
