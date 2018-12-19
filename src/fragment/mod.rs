pub struct Fragment<'a> {
    pub texture: sdl2::render::Texture<'a>,
    pub source_rect: sdl2::rect::Rect,
    pub position: sdl2::rect::Rect,

    pub scale: f32,
    pub rotation: f32,
}

impl<'a> Fragment<'a> {
    pub fn new(texture: sdl2::render::Texture<'a>) -> Self {
        let width = texture.query().width;
        let heigth = texture.query().height;

        Fragment {
            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
            position: sdl2::rect::Rect::new(0, 0, width, heigth),

            scale: 1.0,
            rotation: 0.0,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32){
        self.position.set_x(x);
        self.position.set_y(y);
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
