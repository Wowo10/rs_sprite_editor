use ui_stuff::{im_str, Ui, UserInterface};

pub struct MainMenuInterface {
    pub exit: bool,
}

impl MainMenuInterface {
    pub fn new() -> Self {
        MainMenuInterface { exit: false }
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
                    //Some file menu to choose
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
    }
}
