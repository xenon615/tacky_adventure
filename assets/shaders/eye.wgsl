#import bevy_pbr::mesh_view_bindings::{globals, view},
// #import "shaders/inc/functions.wgsl"::fresnel;
// #import bevy_render::color_operations::hsv_to_rgb;

@group(2) @binding(0) var <uniform> eye_color: vec4f;
@group(2) @binding(1) var <uniform> eye_blink: i32;

//  ---

fn dist(uv: vec2f, kx: f32, ky: f32) -> f32{
    return sqrt(uv.x * uv.x * kx + uv.y * uv.y * ky);
}

// ---

@fragment  
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv0: vec2<f32>,
) -> @location(0) vec4f {
    let uv = 2. * uv0 - 1;
    let d = dist(uv, 1., 1.);
    
    let d_pupil = dist(uv, 6.,  0.1);
    let d_iris = dist(uv, 4., 1.);
    var col = vec3f(0.01, 0.01, 0.01);
    

    if (d_pupil < 0.05) { // pupil
        col = vec3f(0, 0, 0);  
    } else if (d_iris < 0.3) { // iris
         col = mix(eye_color.xyz, vec3f(0,0,0), d_iris * 3.1) *  10.;     
    } 
    else if (d < 0.4) {  // white

        col = mix(
            clamp(eye_color.xyz, vec3f(0.1, 0.1, 0.1), vec3f(0.8, 0.8, 0.8)),
            vec3f(0., 0., 0.), 
            d * 2.3
        );
    } 
    
    if (eye_blink == 1) {
        let d_closed = dist(uv, 0.05, 4.);
        
        if (d_closed > clamp(sin(globals.time * 20) * 5., 0.1, 0.99)) {
            col = vec3f(0.01, 0.01, 0.01);
        }
    }

    return vec4(col, 1.);
}

