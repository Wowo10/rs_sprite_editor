pub struct Spritesheet<'a> {
    pub texture: &'a sdl2::render::Texture<'a>,
    pub source_rect: sdl2::rect::Rect,
    pub frame_width: i32,
    pub position: sdl2::rect::Rect,

    pub scale: f32,
    pub rotation: f64,

    pub frame_count: u32,

    pub current: usize,
}

impl<'a> Spritesheet<'a> {
    pub fn new(
        texture: &'a sdl2::render::Texture<'a>,
        x_pos: i32,
        y_pos: i32,
        frame_count: u32,
    ) -> Self {
        let width = texture.query().width/frame_count;
        let heigth = texture.query().height;

        Spritesheet {
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
}

pub struct Doodad<'a> {
    pub texture: &'a sdl2::render::Texture<'a>,
    pub source_rect: sdl2::rect::Rect,
    pub positions: Vec<sdl2::rect::Rect>,

    pub scale: f32,
    pub rotations: Vec<f64>,

    pub current: usize,
}

impl<'a> Doodad<'a> {
    pub fn new(
        texture: &'a sdl2::render::Texture<'a>,
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
            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
            positions: positions,

            scale: 1.0,
            rotations: rotations,

            current: 0,
        }
    }
}

pub trait Fragment<'a> {
    fn set_position(&mut self, x: i32, y: i32);
    fn draw_position(&self) -> sdl2::rect::Rect;
    fn real_position(&self) -> sdl2::rect::Rect;
    fn change_position(&mut self, diff_x: i32, diff_y: i32);
    fn set_rotation(&mut self,rotation: f64);
    fn set_scale(&mut self, scale: f32);

    fn get_texture(&self) -> &sdl2::render::Texture;
    fn get_source_rect(&self) -> sdl2::rect::Rect;
    fn get_rotation(&self) -> f64;
    fn get_scale(&self) -> f32;
    fn get_position(&self) -> sdl2::rect::Rect;

    fn next_frame(&mut self);
}

impl<'a> Fragment<'a> for Spritesheet<'a> {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
    }

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
    fn real_position(&self) -> sdl2::rect::Rect{
        self.position
    }
    
    fn change_position(&mut self, diff_x: i32, diff_y: i32){
        self.position.x += diff_x;
        self.position.y += diff_y;
    }
    fn set_rotation(&mut self, rotation: f64){
        self.rotation = rotation;
    }
    fn set_scale(&mut self, scale: f32){
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
    fn get_position(&self) -> sdl2::rect::Rect{
        self.position
    }

    fn next_frame(&mut self) {
        self.current += 1;

        if self.current as u32 >= self.frame_count {
            self.current = 0;
        }

        self.source_rect.set_x(self.current as i32 * self.frame_width);
    }
}

impl<'a> Fragment<'a> for Doodad<'a> {
    fn set_position(&mut self, x: i32, y: i32) {
        self.positions[self.current].x = x;
        self.positions[self.current].y = y;
    }

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
    fn change_position(&mut self, diff_x: i32, diff_y: i32){
        self.positions[self.current].x += diff_x;
        self.positions[self.current].y += diff_y;
    }
    fn set_rotation(&mut self, rotation: f64){
        self.rotations[self.current] = rotation;
    }
    fn set_scale(&mut self, scale: f32){
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
    fn get_position(&self) -> sdl2::rect::Rect{
        self.positions[self.current]
    }

    fn next_frame(&mut self) {
        self.current += 1;

        if self.current >= self.positions.len() {
            self.current = 0;
        }
    }
}