use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};

use std::collections::HashMap;
use std::rc::Rc;

pub struct ResourceManager<'l> {
    loader: &'l TextureCreator<sdl2::video::WindowContext>,
    cache: HashMap<String, Rc<Texture<'l>>>,
}

impl<'l> ResourceManager<'l> {
    pub fn new(loader: &'l TextureCreator<sdl2::video::WindowContext>) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    pub fn load(&mut self, details: String) -> Result<Rc<Texture<'l>>, String> {
        self.cache.get(&details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load_texture(format!("resources/{}", details))?);
                self.cache.insert(details, resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }

    pub fn get_spritesheet(&mut self, name: &str) -> Result<Rc<Texture<'l>>, String> {
        self.load("spritesheets/".to_string() + name)
    }

    pub fn get_doodad(&mut self, name: &str) -> Result<Rc<Texture<'l>>, String> {
        self.load("doodads/".to_string() + name)
    }
}
