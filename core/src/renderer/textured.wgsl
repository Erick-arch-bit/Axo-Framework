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
    border_radius: f32,
};

@group(0) @binding(1) var<uniform> rect: Rect;
@group(0) @binding(2) var texture: texture_2d<f32>;
@group(0) @binding(3) var tex_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec2<f32>,
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
    return VertexOutput(vec4<f32>(clip_x, clip_y, 0.0, 1.0), uv, vec2<f32>(world_x, world_y));
}

fn rounded_alpha(world_x: f32, world_y: f32) -> f32 {
    let radius = min(rect.border_radius, min(rect.w, rect.h) * 0.5);
    if radius <= 0.0 { return 1.0; }
    let local_x = world_x - rect.x;
    let local_y = world_y - rect.y;
    let r = radius;
    if local_x < r && local_y < r {
        let d = distance(vec2<f32>(local_x, local_y), vec2<f32>(r, r));
        return 1.0 - smoothstep(max(0.0, r - 1.0), r, d);
    }
    if local_x > rect.w - r && local_y < r {
        let d = distance(vec2<f32>(local_x, local_y), vec2<f32>(rect.w - r, r));
        return 1.0 - smoothstep(max(0.0, r - 1.0), r, d);
    }
    if local_x < r && local_y > rect.h - r {
        let d = distance(vec2<f32>(local_x, local_y), vec2<f32>(r, rect.h - r));
        return 1.0 - smoothstep(max(0.0, r - 1.0), r, d);
    }
    if local_x > rect.w - r && local_y > rect.h - r {
        let d = distance(vec2<f32>(local_x, local_y), vec2<f32>(rect.w - r, rect.h - r));
        return 1.0 - smoothstep(max(0.0, r - 1.0), r, d);
    }
    return 1.0;
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>, @location(1) world_pos: vec2<f32>) -> @location(0) vec4<f32> {
    let tint = vec4<f32>(rect.r, rect.g, rect.b, rect.a);
    let tex_color = textureSample(texture, tex_sampler, uv);
    let alpha = rounded_alpha(world_pos.x, world_pos.y);
    return vec4<f32>(tex_color.rgb * tint.rgb, tex_color.a * tint.a * alpha);
}
