# Finneon 🐠

Finneon is a simple rust image post processing library.

```rust
#[derive(Debug, Clone)]
struct Block {
    texture: Sampler,
}

fn fragment(
    FragColor(color): FragColor,
    Uv(uv): Uv,
    Uniforms(block): Uniforms<Block>
) -> Vec4 {
    let sample = block.texture.sample_u8(uv);
    sample.lerp(color, uv.y)
}
```

The api is a modern take on shader apis, using the extractor pattern. It allows for quick prototying and easy to read code.

The api is simpler than any graphics api but offers worse performance as all the work is done in parallel on the cpu.

## Examples

Head over to the [examples](/examples/) folder for a showcase of what you can do with finneon.
