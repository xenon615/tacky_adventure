#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

// @group(2) @binding(0) var <uniform> base_color: vec4f;
@group(2) @binding(1) var <uniform> stage_index: u32;

const ARM_COUNT =  3.0;
const WHIRL = 14.0;

// ---

const PI: f32 = 3.14;
const TAO: f32 = 2 * PI;
const OVERBRIGHT: f32 = 2.0;
const ARMCOUNT: f32 = 3.0;
const ARMROT: f32 = 1.6;
const INNERCOLOR =  vec4f(2.0, 0.5, 0.1, 1.0);
const OUTERCOLOR = vec4f(0.8, 0.6, 1.0, 1.0);
const WHITE = vec4f(1.0, 1.0, 1.0, 1.0);

// ---

@fragment
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    if stage_index == 0 {
        var  uv = (2. * vo.uv - 1) * 0.5;
        uv = vec2((atan2(uv.y,uv.x)) - globals.time  * 1.1, sqrt(uv.x * uv.x + uv.y * uv.y));
        let g = pow(1.-uv.y, 10.) * 10.;
        let col = vec3f(sin((uv.x + pow(uv.y, 0.2) * WHIRL) * ARM_COUNT)) + g - uv.y * 2.2;
        return vec4f(col / 2., 1.);
    } else {
        let time = globals.time;
        let uv = 2. * vo.uv - 1;
        let cost = cos(-time * 0.2);
        let sint = sin(-time * 0.2);
        let trm = mat2x2 (cost,sint,-sint,cost);
        var  p = uv;
        p = p * trm;
        let d = length(p);
        let cosr = cos(ARMROT * sin(ARMROT * time));
        let sinr = sin(ARMROT * cos(ARMROT * time));
        let  rm = mat2x2(cosr,sinr,-sinr,cosr);
        p = mix(p, p * rm, d);
        var angle = (atan2(p.y, p.x)/ TAO) * 0.5 + 0.5;
        angle += sin(-time *5.0 + fract(d * d * d) * 10.0) * 0.004;
        angle *= 2.0 * ARMCOUNT;
        angle = fract(angle);
        var arms = abs(angle *2.0 - 1.0);
        arms = pow(arms, 10.0 * d *d + 5.0);
        let bulk = 1.0 - clamp(d, 0., 1.);
        let core = pow(bulk, 9.0);
        var color = mix(INNERCOLOR, OUTERCOLOR, d * 2.0);
        color = (bulk * arms * color + core + bulk * 0.25 * mix(color, WHITE, 0.5)) * OVERBRIGHT;
        if color.a < 0.01 {
            color = vec4f(0, 0, 0, 0.5);
        }
        return color;
    }

}
    
