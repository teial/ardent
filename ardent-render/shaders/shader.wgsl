// A minimal passthrough shader for 2D geometry rendering.

@vertex
fn vs_main(@location(0) in_pos: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(in_pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.8, 0.3, 0.2, 1.0); // temporary hardcoded fill color
}
