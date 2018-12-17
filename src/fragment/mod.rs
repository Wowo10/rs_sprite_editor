enum ImageType {
    doodad,
    spritesheet,
    album,
}

pub struct Fragment {
    image_type: ImageType,

    texture: sdl2::render::Texture<'static>,
    source_rect: sdl2::rect::Rect,
    position: sdl2::rect::Rect,

    scale: f32,
    rotation: f32,
}

impl Fragment {
    pub fn new(texture: sdl2::render::Texture<'static>) -> Self {
        let width = texture.query().width;
        let heigth = texture.query().height;

        Fragment {
            image_type: ImageType::album,

            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
            position: sdl2::rect::Rect::new(0, 0, width, heigth),

            scale: 1.0,
            rotation: 0.0,
        }
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
