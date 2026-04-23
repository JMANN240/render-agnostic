use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use macroquad::texture::Texture2D;

#[derive(Debug, Default, Clone)]
pub struct MacroquadImageRegistry {
    images: HashMap<String, Texture2D>,
}

impl MacroquadImageRegistry {
    pub fn register_image(&mut self, image_name: String, image: Texture2D) {
        self.images.insert(image_name, image);
    }

    pub fn get_image<Q: Hash + Eq + ?Sized>(&self, image_name: &Q) -> Option<&Texture2D>
    where
        String: Borrow<Q>,
    {
        self.images.get(image_name)
    }
}
