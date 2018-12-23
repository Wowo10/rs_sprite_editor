pub struct Spritesheet<'a> {
    pub texture: &'a sdl2::render::Texture<'a>,
    pub source_rect: sdl2::rect::Rect,
    pub position: sdl2::rect::Rect,

    pub scale: f32,
    pub rotation: f64,

    pub frame_count: u32,

    pub current: usize,
}

impl<'a> Spritesheet<'a> {
    pub fn new(texture: &'a sdl2::render::Texture<'a>, x_pos: i32, y_pos: i32, frame_count: u32) -> Self {
        let width = texture.query().width;
        let heigth = texture.query().height;

        Spritesheet {
            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
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
    pub fn new(texture: &'a sdl2::render::Texture<'a>, x_pos: i32, y_pos: i32, frame_count: u32) -> Self {
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
    //fn new(texture: sdl2::render::Texture<'a>, x_pos: i32, y_pos: i32, frame_count: u32) -> Self;

    fn set_position(&mut self, x: i32, y: i32);
    fn draw_position(&self) -> sdl2::rect::Rect;

    fn get_texture(&self) -> &sdl2::render::Texture;
    fn get_source_rect(&self) -> sdl2::rect::Rect;
    fn get_rotation(&self) -> f64;
    fn get_scale(&self) -> f32;
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
}
/*
impl<'a> Fragment<'a> {
    pub fn new_doodad(texture: sdl2::render::Texture<'a>, x_pos: i32, y_pos: i32) -> Self {
        let width = texture.query().width;
        let heigth = texture.query().height;

        Fragment {
            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
            position: sdl2::rect::Rect::new(x_pos, y_pos, width, heigth),

            scale: 1.0,
            rotation: 0.0,

            frame_count: 0,
        }
    }

    pub fn new_spritesheet(texture: sdl2::render::Texture<'a>, frame_count: u32) -> Self {
        let width = texture.query().width;
        let heigth = texture.query().height;

        Fragment {
            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width / frame_count, heigth),
            position: sdl2::rect::Rect::new(0, 0, width / frame_count, heigth),

            scale: 1.0,
            rotation: 0.0,
            frame_count: frame_count,
        }
    }

    pub fn next_step() {
        //source_rect
    }

    pub fn draw_position(&self) -> sdl2::rect::Rect {
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
}
*/
