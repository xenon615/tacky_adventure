
#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var <uniform> base_color: vec4f;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var <uniform> stage_index: u32;

fn palette(t: f32) ->  vec3f {
    let a = vec3f(0.5, 0.5, 0.5);
    let b = vec3f(0.5, 0.5, 0.5);
    let c = vec3f(1, 1, 1);
    let d = vec3f(0.263, 0.416, 0.557);
    return a + b * cos( 6.28318 * (c * t + d));
}

@fragment  
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    if (stage_index == 0) {
        return vec4f(1, 1, 1, 1);
    } 

    var uv = 2. * vo.uv - 1;
    var uv0 = uv;
    uv = fract(uv * 2) - 0.5;
    var col = palette(
        sin(length(uv0) + globals.time)
    );
    let d = 0.02 / length(uv);
    return vec4f(base_color.xyz + col * d, base_color.a);
}
