use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::Seek;
use std::io::{BufRead, BufReader, SeekFrom};

pub struct Config {
    config_file: File,
    buffer: HashMap<String, String>,
}

impl Config {
    pub fn create(config_name: &'static str) -> Self {
        Config {
            config_file: File::open(config_name).unwrap(),
            buffer: HashMap::new(),
        }
    }

    pub fn read(&mut self, key: &'static str) -> String {
        self.config_file.seek(SeekFrom::Start(0)).unwrap();

        match self.buffer.get(key).cloned() {
            Some(review) => review.to_string(),
            None => {
                let reader = BufReader::new(&self.config_file);

                for line in reader.lines() {
                    let [data_key, data_value] = Config::split_data(line.unwrap());

                    if data_key == key.to_string() {
                        self.buffer.insert(data_key.clone(), data_value.clone());

                        return data_value;
                    }
                }

                "".to_string()
            }
        }
    }

    fn split_data(data: String) -> [String; 2] {
        let temp: Vec<String> = data.split(';').map(String::from).collect();

        [temp[0].clone(), temp[1].clone()]
    }

    pub fn read_color(&mut self, key: &'static str) -> sdl2::pixels::Color {
        let str_array = self.read(key);

        if str_array == "".to_string() {
            panic!(
                "BUG! Color that you are looking for is not defined: {}",
                key
            );
        }

        let [red, green, blue] = Config::split_color(str_array);

        sdl2::pixels::Color::RGB(red, green, blue)
    }

    fn split_color(data: String) -> [u8; 3] {
        let temp: Vec<String> = data.split(',').map(String::from).collect();

        [
            temp[0].clone().parse::<u8>().unwrap(),
            temp[1].clone().parse::<u8>().unwrap(),
            temp[2].clone().parse::<u8>().unwrap(),
        ]
    }
}
