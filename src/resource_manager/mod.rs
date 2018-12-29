// use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
// use std::collections::HashMap;
// use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, Texture};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

// pub struct ResourceManager<'a> {
//     pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
//     buffer: HashMap<String, sdl2::render::Texture<'a>>,
// }

// impl<'a> ResourceManager<'a> {
//     pub fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
//         ResourceManager {
//             texture_creator: texture_creator,
//             buffer: HashMap::new(),
//         }
//     }

//     fn load_texture(&self, path: &Path) {
//         let texture = self.texture_creator.load_texture(path).unwrap();

//         self.buffer.insert(String::new(), texture);
//     }

//     pub fn get_spritesheet(&self, name: &'static str) {
//         self.load_texture(Path::new(&("resources/spritesheets/".to_string() + name)))
//     }

//     pub fn get_doodad(&self, name: &'static str) {
//         self.load_texture(Path::new(&("resources/doodads/".to_string() + name)))
//     }
// }


pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
    where K: Hash + Eq,
          L: 'l + ResourceLoader<'l, R>
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
    where K: Hash + Eq,
          L: ResourceLoader<'l, R>
{
    pub fn new(loader: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
        where L: ResourceLoader<'l, R, Args = D>,
              D: Eq + Hash + ?Sized,
              K: Borrow<D> + for<'a> From<&'a D>
    {
        self.cache
            .get(details)
            .cloned()
            .map_or_else(|| {
                             let resource = Rc::new(self.loader.load(details)?);
                             self.cache.insert(details.into(), resource.clone());
                             Ok(resource)
                         },
                         Ok)
    }
}

// TextureCreator knows how to load Textures
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;
    fn load(&'l self, path: &str) -> Result<Texture, String> {
        self.load_texture(path)
    }
}

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}