use finneon::{
    extract::{FragColor, Uniforms, Uv},
    App,
};
use glam::Vec4;

fn main() {
    tracing_subscriber::fmt::init();
    App::new(State {
        ramp: (
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            Vec4::new(0.95, 0.2, 0.35, 1.0),
        ),
    })
    .run(
        "static/dog.png",
        fragment,
        std::fs::File::create("static/color_ramp.png").unwrap(),
    )
    .unwrap();
}

#[derive(Clone)]
struct State {
    ramp: (Vec4, Vec4),
}

fn fragment(
    Uv(_uv): Uv,
    FragColor(color): FragColor,
    Uniforms(uniforms): Uniforms<State>,
) -> glam::Vec4 {
    let value = color.x + color.y + color.z;
    let value = value / 3.0;
    let (start, end) = uniforms.ramp;
    start.lerp(end, value)
}
