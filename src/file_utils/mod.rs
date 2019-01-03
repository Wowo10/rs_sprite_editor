use std::fs;
use std::path::Path;

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

pub fn save_template(name: String, data: String) {
    let mut path = String::from("resources/definitions/");
    path += &name;
    path += ".csv";

    let path = Path::new(&path);

    fs::write(path, data).expect("Failed to initialise file write.");
}

pub fn load_file_by_lines(name: String) -> Vec<String> {
    let mut path = String::from("resources/definitions/");
    path += &name;
    path += ".csv";

    let path = Path::new(&path);

    let buffer = fs::read_to_string(path).expect("Failed to initialise file read.");

    let mut result = Vec::new();
    for line in buffer.lines() {
        result.push(line.to_owned());
    }

    result
}

pub fn split_line(data: &String, pattern: &str) -> Vec<String> {
    data.split(pattern).map(String::from).collect()
}
