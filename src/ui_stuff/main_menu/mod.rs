use ui_stuff::*; //{im_str, ImGuiCond, Ui, UserInterface, ImString, ImVec2};

pub struct MainMenuInterface {
    pub exit: bool,
    pub open: bool,
    pub load: bool,
    pub save: bool,
    pub add: bool,

    save_input: ImString,
}

impl MainMenuInterface {
    pub fn new() -> Self {
        MainMenuInterface {
            exit: false,
            open: false,
            load: false,
            save: false,
            add: false,

            save_input: ImString::new(""),
        }
    }
}

impl UserInterface for MainMenuInterface {
    fn draw_window(&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File")).build(|| {
                if ui.menu_item(im_str!("New")).build() {
                    //Some file menu to choose basic texture
                }
                if ui.menu_item(im_str!("Load")).build() {
                    //Some file menu to choose other config.csv
                }
                if ui.menu_item(im_str!("Save")).build() {
                    self.save = true;
                }
                if ui.menu_item(im_str!("Exit")).build() {
                    self.exit = true;
                }
            });

            ui.menu(im_str!("Add")).build(|| {
                if ui.menu_item(im_str!("Doodad")).build() {
                    //Some file menu to choose doodad texture
                }
            });
        });

        if self.save {
            ui.window(im_str!("Save File"))
                .size((300.0, 100.0), ImGuiCond::Once)
                .position((100.0, 100.0), ImGuiCond::Once)
                .title_bar(true)
                .movable(true)
                .resizable(false)
                .save_settings(false)
                .inputs(true) // interacting with buttons.
                .collapsible(false)
                .scroll_bar(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.input_text(im_str!("Filename"), &mut self.save_input)
                        .auto_select_all(true)
                        .chars_noblank(true)
                        .chars_uppercase(true)
                        .build();

                    ui.input_text_multiline(
                        im_str!("Filename2"),
                        &mut self.save_input,
                        ImVec2::new(0.0, 0.0),
                    )
                    .build();

                    if ui.button(im_str!("Save!"), ImVec2::new(0.0, 0.0)) {
                        println!("Save: {:?}", self.save_input);
                    }
                });
        }
    }
}
