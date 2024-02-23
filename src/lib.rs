use image::{DynamicImage, GenericImageView, ImageError};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{borrow::Cow, io};

pub enum Image {
    Handle(DynamicImage),
    Error(ImageError),
}

impl From<&str> for Image {
    fn from(path: &str) -> Self {
        match image::open(path) {
            Ok(img) => Image::Handle(img),
            Err(e) => Image::Error(e),
        }
    }
}

impl From<DynamicImage> for Image {
    fn from(img: DynamicImage) -> Self {
        Image::Handle(img)
    }
}

pub struct App<T> {
    uniforms: T,
}

impl<T> App<T> {
    fn new(uniforms: T) -> Self {
        App { uniforms }
    }
    fn run(
        &self,
        image: impl Into<Image>,
        fragment: impl Fn(&T, glam::Vec2, glam::Vec4) -> glam::Vec4,
        mut output: impl io::Write + io::Seek,
    ) -> Result<(), ImageError> {
        let img = image.into();
        let img = match img {
            Image::Handle(img) => img,
            Image::Error(e) => return Err(e),
        };
        img.pixels().par_bridge().for_each(|(x, y, color)| {});
        Ok(())
    }
}
