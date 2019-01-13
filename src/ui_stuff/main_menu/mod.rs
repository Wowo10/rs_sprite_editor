use file_utils;
use ui_stuff::{im_str, ImGuiCond, ImGuiSelectableFlags, ImString, ImVec2, Ui, UserInterface};

enum WindowVisible {
    None,
    New,
    Load,
    Save,
    AddDoodad,
    ChangeSpritesheet,
}

#[derive(Clone)]
pub enum MainMenuCommand {
    None,
    Exit,
    New,
    ClearDoodads,
    Load(String),
    Save(String),
    AddDoodad(String),
    ChangeSpritesheet(String, u8),
}

pub struct MainMenuInterface {
    window: WindowVisible,
    command: MainMenuCommand,
    pub exit: bool,

    text_input: ImString,
    frames_input: i32,

    selected: usize,
    list_directory: Vec<ImString>,
}

impl MainMenuInterface {
    pub fn new() -> Self {
        MainMenuInterface {
            window: WindowVisible::None,
            command: MainMenuCommand::None,

            exit: false,

            text_input: ImString::with_capacity(64),
            frames_input: 0,

            selected: 0,
            list_directory: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.text_input.clear();
        self.frames_input = 0;

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

        self.list_directory.sort();
    }

    pub fn check(&mut self) -> MainMenuCommand {
        let temp = self.command.clone();

        self.command = MainMenuCommand::None;

        temp
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
                    self.command = MainMenuCommand::Exit;
                }
            });

            ui.menu(im_str!("Fragments")).build(|| {
                if ui.menu_item(im_str!("Add Doodad")).build() {
                    self.window = WindowVisible::AddDoodad;
                    self.reset();
                }
                if ui.menu_item(im_str!("Clear Doodads")).build() {
                    self.window = WindowVisible::None;
                    self.command = MainMenuCommand::ClearDoodads;
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
                            self.window = WindowVisible::None;
                            self.command = MainMenuCommand::New;
                        }
                    });
            }

            WindowVisible::Load => {
                ui.window(im_str!("Load File"))
                    .size((300.0, 300.0), ImGuiCond::Once)
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
                                self.window = WindowVisible::None;
                                self.command = MainMenuCommand::Load(
                                    self.list_directory[self.selected].to_str().to_owned(),
                                );
                            }
                        }
                    });
            }

            WindowVisible::Save => {
                ui.window(im_str!("Save File"))
                    .size((300.0, 300.0), ImGuiCond::Once)
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
                                    ) {
                                        self.text_input = self.list_directory[i].clone();
                                    }
                                }
                            });

                        ui.input_text(im_str!("Filename"), &mut self.text_input)
                            .build();

                        if ui.button(im_str!("Save!"), ImVec2::new(0.0, 0.0)) {
                            if self.text_input != ImString::new("") {
                                self.window = WindowVisible::None;
                                self.command =
                                    MainMenuCommand::Save(self.text_input.to_str().to_owned());
                            }
                        }
                    });
            }

            WindowVisible::AddDoodad => {
                ui.window(im_str!("Doodad choose"))
                    .size((300.0, 300.0), ImGuiCond::Once)
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
                            if self.selected < self.list_directory.len() {
                                self.window = WindowVisible::None;
                                self.command = MainMenuCommand::AddDoodad(
                                    self.list_directory[self.selected].to_str().to_owned(),
                                );
                            }
                        }
                    });
            }

            WindowVisible::ChangeSpritesheet => {
                ui.window(im_str!("SpriteSheet choose"))
                    .size((300.0, 300.0), ImGuiCond::Once)
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

                        ui.input_int(im_str!("frames"), &mut self.frames_input)
                            .chars_decimal(true)
                            .build();

                        ui.separator();

                        if ui.button(im_str!("Change!"), ImVec2::new(0.0, 0.0)) {
                            if self.frames_input > 0 && self.frames_input < 127 {
                                self.window = WindowVisible::None;
                                self.command = MainMenuCommand::ChangeSpritesheet(
                                    self.list_directory[self.selected].to_str().to_owned(),
                                    self.frames_input as u8,
                                );
                            }
                        }
                    });
            }

            _ => {}
        }
    }
}
