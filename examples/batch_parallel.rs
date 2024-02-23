use finneon::prelude::*;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    tracing_subscriber::fmt::init();
    let app = App::new(State {
        frame: 0,
        frame_count: 10,
    });
    (0..10)
        .into_par_iter()
        .map(move |idx| {
            let app = app.clone();
            let file = std::fs::File::create(format!("static/parallel/frame_{}.png", idx))?;
            app.set_uniforms(State {
                frame: idx,
                frame_count: 10,
            })
            .run("static/dog.png", fragment, file)
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to render frame");
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
