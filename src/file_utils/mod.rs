use std::fs;

fn list_directory(path: &'static str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();

    let mut vec: Vec<String> = Vec::new();

    for path in paths {
        let mut temp = path.unwrap().file_name().into_string().unwrap();

        vec.push(temp.chars().take(temp.len() - 4).collect());
    }

    vec
}

fn imgui_list_directory(vec: Vec<String>) -> Vec<imgui::ImString> {
    vec.into_iter().map(|st| imgui::ImString::new(st)).collect()
}

pub fn get_imgui_directory(path: &'static str) -> Vec<imgui::ImString> {
    imgui_list_directory(list_directory(path))
}
