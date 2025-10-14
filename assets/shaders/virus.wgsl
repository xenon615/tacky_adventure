
#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var <uniform> base_color: vec4f;

@fragment  
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    var uv = 2. * vo.uv - 1;
    var uv0 = uv;
    uv = fract(uv * 2) - 0.5;
    var col = sin(length(uv0) + globals.time);
    let d = 0.02 / length(uv);
    return vec4f(base_color.xyz + col * d, base_color.a);
}
