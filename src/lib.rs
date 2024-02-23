use glam::Vec4;
use image::{DynamicImage, GenericImageView, ImageError, ImageOutputFormat, RgbaImage};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{borrow::Cow, io};

pub mod extract;

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

impl<T> App<T>
where
    T: Send + Sync,
{
    pub fn new(uniforms: T) -> Self {
        App { uniforms }
    }
    pub fn run<A>(
        &self,
        image: impl Into<Image>,
        fragment: impl extract::Handler<A, T> + Send + Sync,
        mut output: impl io::Write + io::Seek,
    ) -> Result<(), ImageError> {
        tracing::info!("Running app");
        let img = image.into();
        let img = match img {
            Image::Handle(img) => img,
            Image::Error(e) => return Err(e),
        };
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        let expected = img.width() as f32 * img.height() as f32;
        let _ = std::thread::spawn(move || {
            let mut current = 0.0;
            loop {
                let _ = rx.recv();
                //tracing::info!("{}/{}", current, expected);
                current += 1.0;
                if (current / expected * 100.0).rem_euclid(10.0) == 0.0 {
                    tracing::info!("{}%", (current / expected * 100.0));
                }
                if current == expected {
                    break;
                }
            }
        });
        let result: Vec<_> = img
            .pixels()
            //.par_bridge()
            .map(|(x, y, color)| {
                let fragcoord = glam::Vec2::new(x as f32, y as f32);
                let color = color.0;
                let color = glam::Vec4::new(
                    color[0] as f32,
                    color[1] as f32,
                    color[2] as f32,
                    color[3] as f32,
                );
                let ctx = extract::Context {
                    app: self,
                    image: &img,
                    fragcoord,
                    fragcolor: color,
                };
                let color = fragment.handle(&ctx);
                let _ = tx.send(());
                (x, y, color)
            })
            .collect();
        let mut output_img = RgbaImage::new(img.width(), img.height());
        for (x, y, color) in result {
            output_img.put_pixel(
                x,
                y,
                image::Rgba([color.x as u8, color.y as u8, color.z as u8, color.w as u8]),
            );
        }
        image::write_buffer_with_format(
            &mut output,
            &output_img,
            output_img.width(),
            output_img.height(),
            image::ColorType::Rgba8,
            ImageOutputFormat::Png,
        )?;
        Ok(())
    }
}
