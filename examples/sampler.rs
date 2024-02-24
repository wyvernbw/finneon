use std::fs;

use finneon::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();
    let output = fs::File::create("static/sampler.png").unwrap();
    let app = App::new(Block {
        texture: "static/color_ramp.png".try_into().unwrap(),
    })
    .run("static/dog.png", fragment, output);
}

#[derive(Debug, Clone)]
struct Block {
    texture: Sampler,
}

fn fragment(FragColor(color): FragColor, Uv(uv): Uv, Uniforms(block): Uniforms<Block>) -> Vec4 {
    let sample = block.texture.sample_u8(uv);
    sample.lerp(color, uv.y)
}
