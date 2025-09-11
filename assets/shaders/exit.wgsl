#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

// @group(2) @binding(0) var <uniform> base_color: vec4f;
@group(2) @binding(1) var <uniform> stage_index: u32;

const ARM_COUNT =  3.0;
const WHIRL = 14.0;

@fragment
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    var  uv = (2. * vo.uv - 1) * 0.5;
    uv = vec2((atan2(uv.y,uv.x)) - globals.time  * 1.1, sqrt(uv.x * uv.x + uv.y * uv.y));
    let g = pow(1.-uv.y, 10.) * 10.;
    let col = vec3f(sin((uv.x + pow(uv.y, 0.2) * WHIRL) * ARM_COUNT)) + g - uv.y * 2.2;
    // let alpha = select(1., 0., col.x < 0.1); 
    // return vec4f(col / 2., alpha);
    return vec4f(col / 2., 1.);
    
}
    
