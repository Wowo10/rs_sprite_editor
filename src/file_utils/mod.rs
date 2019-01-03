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
    
    //let dupa=OsStr::new(&name);

    let mut path = String::from("resources/definitions/");
    path += &name;
    path += ".csv";

    //let path = Path::new(&(("resources/definitions/".to_string() + &name) + ".csv"));
    let path = Path::new(&path);//(&(("resources/definitions/".to_string() + &name) + ".csv"));

    // let mut file = OpenOptions::new().write(true).create(!path.exists()).open(path).unwrap();

    // file.write("");

    fs::write(path, data).expect("bug");

    println!(
        "{}, {:?}",
        path.exists(),
        path
    );
}
// pub fn save_template(path: &'static str) {
//     println!("{}", Path::new(path).exists());
// }
