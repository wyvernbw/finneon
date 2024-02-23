use glam::Vec4;
use image::{
    DynamicImage, GenericImageView, ImageBuffer, ImageError, ImageOutputFormat, Pixel, Rgba,
    RgbaImage,
};
use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    ThreadPool, ThreadPoolBuilder,
};
use std::{
    borrow::Cow,
    io::{self, BufWriter},
    num::NonZeroUsize,
    os::unix::thread,
};

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
    thread_pool: ThreadPool,
}

impl<T> App<T>
where
    T: Send + Sync,
{
    pub fn new(uniforms: T) -> Self {
        App {
            uniforms,
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(
                    std::thread::available_parallelism()
                        .unwrap_or(NonZeroUsize::new(1).unwrap())
                        .get()
                        * 32usize,
                )
                .build()
                .expect("Failed to create thread pool"),
        }
    }
    pub fn run<'a, A>(
        &'a self,
        image: impl Into<Image>,
        fragment: impl extract::Handler<A, T> + Send + Sync,
        output: impl io::Write + io::Seek,
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
                current += 1.0;
                if (current / expected * 100.0).rem_euclid(10.0) == 0.0 {
                    //tracing::info!("{}%", (current / expected * 100.0));
                    tracing::info!("{}/{}", current, expected);
                }
                if current == expected {
                    break;
                }
            }
        });
        let result = RgbaImage::from_fn(img.width(), img.height(), |x, y| {
            let fragcoord = glam::Vec2::new(x as f32, y as f32);
            let color = img.get_pixel(x, y);
            let color = color.0;
            let color = glam::Vec4::new(
                color[0] as f32 / 255.0,
                color[1] as f32 / 255.0,
                color[2] as f32 / 255.0,
                color[3] as f32 / 255.0,
            );
            let ctx = extract::Context {
                app: self,
                image: &img,
                fragcoord,
                fragcolor: color,
            };
            let color = fragment.handle(&ctx);
            let _ = tx.send(());
            Rgba([
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8,
                (color.w * 255.0) as u8,
            ])
        });
        tracing::info!("Processing complete");
        tracing::info!("Writing to output");
        image::write_buffer_with_format(
            &mut BufWriter::new(output),
            &result,
            result.width(),
            result.height(),
            image::ColorType::Rgba8,
            ImageOutputFormat::Png,
        )?;
        Ok(())
    }
}
