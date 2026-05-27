struct Globals {
    screen_width: f32,
    screen_height: f32,
};

@group(0) @binding(0) var<uniform> globals: Globals;

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
};

@group(0) @binding(1) var<uniform> rect: Rect;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

fn corner(idx: u32) -> vec2<f32> {
    if idx == 0u { return vec2<f32>(0.0, 0.0); }
    if idx == 1u { return vec2<f32>(1.0, 0.0); }
    if idx == 2u { return vec2<f32>(0.0, 1.0); }
    if idx == 3u { return vec2<f32>(0.0, 1.0); }
    if idx == 4u { return vec2<f32>(1.0, 0.0); }
    return vec2<f32>(1.0, 1.0);
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let uv = corner(in_vertex_index);
    let world_x = rect.x + uv.x * rect.w;
    let world_y = rect.y + uv.y * rect.h;
    let clip_x = (world_x / globals.screen_width) * 2.0 - 1.0;
    let clip_y = 1.0 - (world_y / globals.screen_height) * 2.0;

    return VertexOutput(vec4<f32>(clip_x, clip_y, 0.0, 1.0));
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(rect.r, rect.g, rect.b, rect.a);
}
