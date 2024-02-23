use finneon::{extract::Uv, App};

fn main() {
    tracing_subscriber::fmt::init();
    let output = std::fs::File::create("static/uv.png").unwrap();
    let app = App::new(()).run("static/fantasy.png", fragment, output);
}

fn fragment(Uv(uv): Uv) -> glam::Vec4 {
    glam::Vec4::new(uv.x, uv.y, 0.0, 1.0)
}
