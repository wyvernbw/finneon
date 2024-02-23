use finneon::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();
    let app = App::new(State {
        frame: 0,
        frame_count: 10,
    });
    (0..10)
        // accumulate the result of each iteration
        .try_fold(app, |app, idx| {
            let file = std::fs::File::create(format!("static/batch/frame_{}.png", idx)).unwrap();
            app.set_uniforms(State {
                frame: idx,
                frame_count: 10,
            })
            .run("static/dog.png", fragment, file)
        })
        .expect("failed to render frame");
}

#[derive(Clone)]
struct State {
    frame: usize,
    frame_count: usize,
}

fn fragment(FragColor(color): FragColor, Uniforms(uniforms): Uniforms<State>) -> glam::Vec4 {
    let value = uniforms.frame as f32 / uniforms.frame_count as f32;
    let color = color * value;
    Vec4::new(color.x, color.y, color.z, 1.0)
}
