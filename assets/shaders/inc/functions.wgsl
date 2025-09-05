fn fresnel(
    camera_view_world_position: vec3<f32>,
    world_position: vec3<f32>,
    world_normal: vec3<f32>,
    power: f32,
    strength: f32,
) -> f32 {
    var V = normalize(camera_view_world_position - world_position);
    var fresnel = 1.0 - dot(world_normal, V);
    return pow(fresnel, power) * strength;
};