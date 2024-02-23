use image::{
    DynamicImage, GenericImageView, ImageBuffer, ImageError, ImageOutputFormat, Pixel, Rgba,
    RgbaImage,
};
use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelBridge,
        ParallelIterator,
    },
    slice::ParallelSlice,
    ThreadPool, ThreadPoolBuilder,
};
use std::{
    io::{self, BufWriter},
    num::NonZeroUsize,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
};

pub mod extract;
pub mod prelude;
pub mod utils;

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

#[derive(Debug, Clone)]
pub struct App<T> {
    uniforms: T,
    thread_pool: Arc<ThreadPool>,
}

impl<T> App<T>
where
    T: Send + Sync,
{
    pub fn new(uniforms: T) -> Self {
        App {
            uniforms,
            thread_pool: Arc::new(
                ThreadPoolBuilder::new()
                    .num_threads(
                        std::thread::available_parallelism()
                            .unwrap_or(NonZeroUsize::new(1).unwrap())
                            .get(),
                    )
                    .use_current_thread()
                    .build()
                    .expect("Failed to create thread pool"),
            ),
        }
    }

    pub fn set_uniforms(self, uniforms: T) -> Self {
        App { uniforms, ..self }
    }

    fn start_progress_thread(expected: f32) -> (Sender<()>, JoinHandle<()>) {
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        let progress = std::thread::spawn(move || {
            let mut current = 0.0;
            loop {
                let _ = rx.recv();
                current += 1.0;
                if (current / expected * 100.0).rem_euclid(10.0) == 0.0 {
                    tracing::info!("{}%", (current / expected * 100.0));
                    //tracing::info!("{}/{}", current, expected);
                }
                if current == expected {
                    break;
                }
            }
        });
        (tx, progress)
    }

    fn convert_to_rgba8(&self, img: Image) -> Result<DynamicImage, ImageError> {
        let img = match img {
            Image::Handle(img) => img,
            Image::Error(e) => return Err(e),
        };
        Ok(DynamicImage::ImageRgba8(img.to_rgba8()))
    }

    fn process<A>(
        &self,
        img: &DynamicImage,
        progress_sender: Sender<()>,
        fragment: &(impl extract::Handler<A, T> + Send + Sync),
    ) -> Vec<u8> {
        let result: Vec<_> = img
            .as_bytes()
            .par_chunks(4)
            .enumerate()
            .flat_map(|(idx, bytes)| {
                let color = Rgba::from_slice(bytes);
                let x = idx % img.width() as usize;
                let y = idx / img.width() as usize;
                let fragcoord = glam::Vec2::new(x as f32, y as f32);
                let color = color.0;
                let color = glam::Vec4::new(
                    color[0] as f32 / 255.0,
                    color[1] as f32 / 255.0,
                    color[2] as f32 / 255.0,
                    color[3] as f32 / 255.0,
                );
                let ctx = extract::Context {
                    app: self,
                    image: img,
                    fragcoord,
                    fragcolor: color,
                };
                let color = fragment.handle(&ctx);
                let _ = progress_sender.send(());
                [
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                    (color.w * 255.0) as u8,
                ]
                .into_par_iter()
            })
            .collect();
        result
    }

    pub fn run<A>(
        self,
        image: impl Into<Image>,
        fragment: impl extract::Handler<A, T> + Send + Sync,
        output: impl io::Write + io::Seek,
    ) -> Result<Self, ImageError> {
        tracing::info!("App started");
        let img = image.into();
        let img = self.convert_to_rgba8(img)?;
        let expected = img.width() as f32 * img.height() as f32;
        let (tx, progress) = Self::start_progress_thread(expected);
        tracing::info!(
            bytes = img.as_bytes().len(),
            threads = self.thread_pool.current_num_threads(),
            "Processing image..."
        );
        let result = self
            .thread_pool
            .install(|| self.process(&img, tx, &fragment));
        debug_assert_eq!(
            result.len(),
            img.width() as usize * img.height() as usize * 4
        );
        let result = RgbaImage::from_vec(img.width(), img.height(), result).unwrap();
        tracing::info!("Processing complete");
        tracing::info!("Writing to output...");
        image::write_buffer_with_format(
            &mut BufWriter::new(output),
            &result,
            result.width(),
            result.height(),
            image::ColorType::Rgba8,
            ImageOutputFormat::Png,
        )?;
        tracing::info!("Image written!");
        progress.join().expect("Failed to join progress thread");
        Ok(self)
    }
}
