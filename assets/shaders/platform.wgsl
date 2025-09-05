
#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(2) @binding(0) var <uniform> base_color: vec4f;

fn palette(t: f32) ->  vec3f {
    let a = vec3f(0.5, 0.5, 0.5);
    let b = vec3f(0.5, 0.5, 0.5);
    let c = vec3f(1, 1, 1);
    let d = vec3f(0.263, 0.416, 0.557);
    return a + b * cos( 6.28318 * (c * t + d));
}

@fragment  
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    var uv = 2. * vo.uv - 1;
    var uv0 = uv;
    var finalColor = base_color.xyz;

    uv = fract(uv * 2) - 0.5;
    var col = 
    palette(
        sin(length(uv0) + globals.time)
    );
    var d = length(uv);
    // d = sin(d * 8 + globals.time) / 8;
    // d = abs(d);
    d = 0.02 / d;
    finalColor += col * d; 
    return vec4f(finalColor, base_color.a);
    
    
}
