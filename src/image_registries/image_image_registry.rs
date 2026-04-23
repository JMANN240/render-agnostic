use std::{borrow::Borrow, collections::HashMap, hash::Hash};

use image::RgbaImage;

#[derive(Debug, Default, Clone)]
pub struct ImageImageRegistry {
    images: HashMap<String, RgbaImage>,
}

impl ImageImageRegistry {
    pub fn register_image(&mut self, image_name: String, image: RgbaImage) {
        self.images.insert(image_name, image);
    }

    pub fn get_image<Q: Hash + Eq + ?Sized>(&self, image_name: &Q) -> Option<&RgbaImage>
    where
        String: Borrow<Q>,
    {
        self.images.get(image_name)
    }
}
