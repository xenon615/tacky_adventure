// Author @patriciogv - 2015
// http://patriciogonzalezvivo.com
// translated by xenon615

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}


fn random (_st: vec2f) -> f32 {
    return fract(sin(dot(_st.xy,vec2f(12.9898, 78.233))) * 43758.5453123);
}

fn noise (_st: vec2f ) -> f32 {
    let i = floor(_st);
    let f = fract(_st);

    let a = random(i);
    let b = random(i + vec2f(1.0, 0.0));
    let c = random(i + vec2f(0.0, 1.0));
    let d = random(i + vec2f(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);

    return mix(a, b, u.x) + (c - a)* u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

const NUM_OCTAVES = 5;

fn fbm (st: vec2f) -> f32{
    var _st = st;
    var v = 0.0;
    var a = 0.5;
    let shift = vec2f(100.0);
    let rot = mat2x2(cos(0.5), sin(0.5), -sin(0.5), cos(0.50));

    for (var i = 0; i < NUM_OCTAVES; i += 1) {
        v += a * noise(_st);
        _st = rot * _st * 2.0 + shift;
        a *= 0.5;
    }
    return v;
}

@fragment
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    let u_time = globals.time;
    let st = 2. * vo.uv - 1;
    var color = vec3f(0.0);

    var q = vec2f(0.);
    q.x = fbm( st + 0.00 * u_time);
    q.y = fbm( st + vec2f(1.0));

    var r = vec2f(0.);
    r.x = fbm( st + 1.0 * q + vec2f(1.7, 9.2) + 0.15 * u_time );
    r.y = fbm( st + 1.0 * q + vec2f(8.3, 2.8) + 0.126 * u_time);

    let f = fbm(st + r);

    color = mix(vec3(0.101961,0.619608,0.666667), vec3f(0.666667,0.666667,0.498039), clamp((f * f) * 4.0, 0.0, 1.0));

    color = mix(color, vec3f(0,0,0.164706),clamp(length(q),0.0,1.0));

    color = mix(color, vec3f(0.666667, 1, 1), clamp(length(r.x),0.0,1.0));

    return vec4f(( f * f * f + 0.6 * f * f + 0.5 * f) * color, 1.);
}