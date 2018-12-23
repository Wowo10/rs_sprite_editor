enum FragmentType{
    spritesheet,
    doodad
}

pub struct Fragment<'a> {
    fragment_type: FragmentType,

    pub texture: sdl2::render::Texture<'a>,
    pub source_rect: sdl2::rect::Rect,
    pub position: sdl2::rect::Rect,

    pub scale: f32,
    pub rotation: f32,

    pub frame_count: u32
}

impl<'a> Fragment<'a> {
    pub fn new_doodad(texture: sdl2::render::Texture<'a>, x_pos: i32, y_pos: i32 ) -> Self {
        
        let width = texture.query().width;
        let heigth = texture.query().height;

        Fragment {
            fragment_type: FragmentType::doodad,

            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width, heigth),
            position: sdl2::rect::Rect::new(x_pos, y_pos, width, heigth),

            scale: 1.0,
            rotation: 0.0,

            frame_count: 0
        }
    }

    pub fn new_spritesheet(texture: sdl2::render::Texture<'a>, frame_count: u32) -> Self {
        let width = texture.query().width;
        let heigth = texture.query().height;

        Fragment {
            fragment_type: FragmentType::spritesheet,

            texture: texture,
            source_rect: sdl2::rect::Rect::new(0, 0, width/frame_count, heigth),
            position: sdl2::rect::Rect::new(0, 0, width/frame_count, heigth),

            scale: 1.0,
            rotation: 0.0,
            frame_count: frame_count
        }
    }

    pub fn next_step(){
        //source_rect
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
