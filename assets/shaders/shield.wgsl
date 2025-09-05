#import "shaders/inc/functions.wgsl"::fresnel
#import bevy_pbr::mesh_view_bindings::view
    
@fragment
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
) -> @location(0) vec4<f32> {
    let fresnel = fresnel(view.world_position.xyz, world_position.xyz, world_normal, 2.0, 2.0);
    return vec4(world_normal, smoothstep(0.0, 1.5, fresnel));
}
