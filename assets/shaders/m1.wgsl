#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}


@group(2) @binding(0) var <uniform> eye_color: vec4f;
@group(2) @binding(1) var <uniform> eye_mode: u32;


@fragment  
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    let uv = 2. * vo.uv - 1;
    var d = distance(uv, vec2f(-0.0, 0.0));
    let t = select(sin( globals.time), 1., eye_mode != 1);
    let r = sin(25 * d);
    let g = smoothstep(0.1, 0.5, d) * eye_color.g;
    return vec4f(r , g, t , 0.5);
}