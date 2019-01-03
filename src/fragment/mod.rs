use sdl2::render::Texture;
use std::rc::Rc;

pub struct Spritesheet<'a> {
    name: String,
    texture: Rc<Texture<'a>>,
    source_rect: sdl2::rect::Rect,
    frame_width: i32,
    position: sdl2::rect::Rect,

    scale: f32,
    rotation: f64,

    frame_count: usize,

    current: usize,
}

impl<'a> Spritesheet<'a> {
    pub fn new(
        name: String,
        texture: Rc<Texture<'a>>,
        x_pos: i32,
        y_pos: i32,
        frame_count: usize,
    ) -> Self {
        let width = texture.query().width / frame_count as u32;
        let heigth = texture.query().height;

        Spritesheet {
            name: name,
            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
            frame_width: width as i32,

            position: sdl2::rect::Rect::new(x_pos, y_pos, width, heigth),

            scale: 1.0,
            rotation: 0.0,

            frame_count: frame_count,

            current: 0,
        }
    }

    // pub fn _load(texture: &'a sdl2::render::Texture<'a>, frame_count: usize) -> Self {
    //     let width = texture.query().width;
    //     let heigth = texture.query().height;

    //     Spritesheet {
    //         texture: texture,
    //         source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
    //         frame_width: width as i32,

    //         position: sdl2::rect::Rect::new(0, 0, width, heigth),

    //         scale: 1.0,
    //         rotation: 0.0,

    //         frame_count: frame_count,

    //         current: 0,
    //     }
    // }

    fn update_frame(&mut self) {
        self.source_rect
            .set_x(self.current as i32 * self.frame_width);
    }

    pub fn serialize(&self) -> String {
        let mut temp = String::new();
        temp += &self.name;
        temp += ";";

        temp += &self.frame_count.to_string();
        temp += ";"; //fetch framerate

        temp
    }

    pub fn get_frames_amount(&self) -> usize {
        self.frame_count
    }
}

pub struct Doodad<'a> {
    name: String,
    texture: Rc<Texture<'a>>,
    source_rect: sdl2::rect::Rect,
    positions: Vec<sdl2::rect::Rect>,

    scale: f32,
    rotations: Vec<f64>,

    current: usize,
}

impl<'a> Doodad<'a> {
    pub fn new(
        name: String,
        texture: Rc<Texture<'a>>,
        x_pos: i32,
        y_pos: i32,
        frame_count: u32,
    ) -> Self {
        let width = texture.query().width;
        let heigth = texture.query().height;

        let mut positions: Vec<sdl2::rect::Rect> = Vec::new();
        let mut rotations: Vec<f64> = Vec::new();

        for _ in 0..frame_count {
            positions.push(sdl2::rect::Rect::new(x_pos, y_pos, width, heigth));
        }

        for _ in 0..frame_count {
            rotations.push(0.0);
        }

        Doodad {
            name: name,
            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
            positions: positions,

            scale: 1.0,
            rotations: rotations,

            current: 0,
        }
    }

    pub fn set_frames_amount(&mut self, frames: usize) {
        while frames != self.positions.len() {
            if frames > self.positions.len() {
                let cloned_first = self.positions.first().cloned().unwrap();
                self.positions.push(cloned_first);
            } else {
                self.positions.pop();
            }
        }
    }

    // pub fn _load(
    //     texture: &'a sdl2::render::Texture<'a>,
    //     positions: Vec<sdl2::rect::Rect>,
    //     rotations: Vec<f64>,
    // ) -> Self {
    //     let width = texture.query().width;
    //     let heigth = texture.query().height;

    //     Doodad {
    //         texture: texture,
    //         source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
    //         positions: positions,

    //         scale: 1.0,
    //         rotations: rotations,

    //         current: 0,
    //     }
    // }

    pub fn serialize(&self, origin: sdl2::rect::Point) -> String {
        let mut temp = String::new();

        temp += &self.name;
        temp += ";";
        for position in &self.positions {
            let temp_point = position.top_left() - origin;
            temp += "(";
            temp += &temp_point.x.to_string();
            temp += ",";
            temp += &temp_point.y.to_string();
            temp += ").";
        }

        temp += ";";
        for rotation in &self.rotations {
            temp += &rotation.to_string();
            temp += ".";
        }

        temp
    }

    pub fn serialize2(&self, origin: sdl2::rect::Point) -> String {
        let mut temp = String::new();

        temp += &self.name;
        temp += ";";
        for i in 0..self.positions.len() {
            let temp_point = self.positions[i].top_left() - origin;
            temp += "[";
            temp += &temp_point.x.to_string();
            temp += ",";
            temp += &temp_point.y.to_string();
            temp += ",";
            temp += &self.rotations[i].to_string();
            temp += "].";
        }

        temp
    }

    pub fn serialize3(&self, origin: sdl2::rect::Point) -> String {
        let mut temp = String::new();

        temp += &self.name;
        temp += ";";

        let mut tempx = String::new();
        let mut tempy = String::new();
        for position in &self.positions {
            let temp_point = position.top_left() - origin;

            tempx += &temp_point.x.to_string();
            tempx += ".";
            tempy += &temp_point.y.to_string();
            tempy += ".";
        }

        temp += &tempx.to_string();
        temp += ";";
        temp += &tempy.to_string();

        temp += ";";
        for rotation in &self.rotations {
            temp += &rotation.to_string();
            temp += ".";
        }

        temp
    }

    pub fn change_all_positions(&mut self, diff_x: i32, diff_y: i32) {
        for position in &mut self.positions {
            if position.x > 0 || diff_x > 0 {
                position.x += diff_x;
            }
            if position.y > 0 || diff_y > 0 {
                position.y += diff_y;
            }
        }
    }
}

pub trait Fragment<'a> {
    fn draw_position(&self) -> sdl2::rect::Rect;
    fn real_position(&self) -> sdl2::rect::Rect;
    fn change_position(&mut self, diff_x: i32, diff_y: i32);
    fn set_rotation(&mut self, rotation: f64);
    fn set_scale(&mut self, scale: f32);

    fn get_texture(&self) -> &sdl2::render::Texture;
    fn get_source_rect(&self) -> sdl2::rect::Rect;
    fn get_rotation(&self) -> f64;
    fn get_scale(&self) -> f32;

    fn set_frame(&mut self, frame_number: usize);
}

impl<'a> Fragment<'a> for Spritesheet<'a> {
    fn draw_position(&self) -> sdl2::rect::Rect {
        let tempx = if self.position.x != 0 {
            self.position.x
        } else {
            1
        } as f32
            / self.scale;

        let tempy = if self.position.y != 0 {
            self.position.y
        } else {
            1
        } as f32
            / self.scale;

        sdl2::rect::Rect::new(
            tempx as i32,
            tempy as i32,
            self.position.width(),
            self.position.height(),
        )
    }
    fn real_position(&self) -> sdl2::rect::Rect {
        self.position
    }

    fn change_position(&mut self, diff_x: i32, diff_y: i32) {
        if self.position.x > 0 || diff_x > 0 {
            self.position.x += diff_x;
        }
        if self.position.y > 0 || diff_y > 0 {
            self.position.y += diff_y;
        }
    }
    fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
    }
    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn get_texture(&self) -> &sdl2::render::Texture<'_> {
        &self.texture
    }
    fn get_source_rect(&self) -> sdl2::rect::Rect {
        self.source_rect
    }
    fn get_rotation(&self) -> f64 {
        self.rotation
    }
    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn set_frame(&mut self, frame_number: usize) {
        self.current = frame_number % self.frame_count;
        self.update_frame();
    }
}

impl<'a> Fragment<'a> for Doodad<'a> {
    fn draw_position(&self) -> sdl2::rect::Rect {
        let tempx = if self.positions[self.current].x != 0 {
            self.positions[self.current].x
        } else {
            1
        } as f32
            / self.scale;

        let tempy = if self.positions[self.current].y != 0 {
            self.positions[self.current].y
        } else {
            1
        } as f32
            / self.scale;

        sdl2::rect::Rect::new(
            tempx as i32,
            tempy as i32,
            self.positions[self.current].width(),
            self.positions[self.current].height(),
        )
    }
    fn real_position(&self) -> sdl2::rect::Rect {
        self.positions[self.current]
    }
    fn change_position(&mut self, diff_x: i32, diff_y: i32) {
        self.positions[self.current].x += diff_x;
        self.positions[self.current].y += diff_y;
    }
    fn set_rotation(&mut self, rotation: f64) {
        self.rotations[self.current] = rotation;
    }
    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn get_texture(&self) -> &sdl2::render::Texture<'_> {
        &self.texture
    }
    fn get_source_rect(&self) -> sdl2::rect::Rect {
        self.source_rect
    }
    fn get_rotation(&self) -> f64 {
        self.rotations[self.current]
    }
    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn set_frame(&mut self, frame_number: usize) {
        self.current = frame_number % self.positions.len();
    }
}
