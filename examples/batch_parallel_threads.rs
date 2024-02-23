use finneon::prelude::*;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

//
// This example spins a new OS thread for each frame to be rendered.
// It is not recommended to use this method for rendering frames in parallel.
// When the number of frames exceeds the amount of logical processors, the performance will degrade quickly.
// This is because the OS will spend more time switching between threads than actually doing work.
// Prefer using a threadpool (see `batch_parallel_rayon.rs` for an example using rayon) or using a single thread to render frames sequentially.

fn main() {
    tracing_subscriber::fmt::init();
    let app = App::new(State {
        frame: 0,
        frame_count: 10,
    });
    let join_handles = (0..32)
        .map(|idx| {
            let app = app.clone();
            std::thread::spawn(move || {
                let file = std::fs::File::create(format!("static/parallel/frame_{}.png", idx))?;
                app.set_uniforms(State {
                    frame: idx,
                    frame_count: 10,
                })
                .run("static/dog.png", fragment, file)
            })
        })
        .collect::<Vec<_>>();
    for join_handle in join_handles {
        join_handle.join().unwrap().unwrap();
    }
}

#[derive(Clone)]
struct State {
    frame: usize,
    frame_count: usize,
}

fn fragment(FragColor(color): FragColor, Uniforms(uniforms): Uniforms<State>) -> glam::Vec4 {
    let value = uniforms.frame as f32 / uniforms.frame_count as f32;
    Vec4::new(value, color.y, color.z, 1.0)
}
