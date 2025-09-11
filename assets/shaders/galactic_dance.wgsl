#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

const PI: f32 = 3.14;
const TAO: f32 = 2 * PI;
const OVERBRIGHT: f32 = 2.0;
const ARMCOUNT: f32 = 3.0;
const ARMROT: f32 = 1.6;
const INNERCOLOR =  vec4f(2.0, 0.5, 0.1, 1.0);
const OUTERCOLOR = vec4f(0.8, 0.6, 1.0, 1.0);
const WHITE = vec4f(1.0, 1.0, 1.0, 1.0);

@fragment
fn fragment(vo: VertexOutput) -> @location(0) vec4f {
    let time = globals.time;
    let uv = 2. * vo.uv - 1;
    
    //constant slow rotation
    let cost = cos(-time * 0.2);
    let sint = sin(-time * 0.2);

    let trm = mat2x2 (cost,sint,-sint,cost);
    var  p = uv;
    //apply slow rotation
    p = p * trm;
    
    //calc distance
    let d = length(p);
    
    //build arm rotation matrix
    let cosr = cos(ARMROT * sin(ARMROT * time));
    let sinr = sin(ARMROT * cos(ARMROT * time));
    let  rm = mat2x2(cosr,sinr,-sinr,cosr);
    
    
    p = mix(p, p * rm, d);
    
    //find angle to middle
    var angle = (atan2(p.y, p.x)/ TAO) * 0.5 + 0.5;
    //add the crinkle
    angle += sin(-time *5.0 + fract(d * d * d) * 10.0) * 0.004;
    //calc angle in terms of arm number
    angle *= 2.0 * ARMCOUNT;
    angle = fract(angle);
    //build arms & wrap the angle around 0.0 & 1.0
    var arms = abs(angle *2.0 - 1.0);
    //sharpen arms
    arms = pow(arms, 10.0 * d *d + 5.0);
    //calc radial falloff
    let bulk = 1.0 - clamp(d, 0., 1.);
    //create glowy center
    let core = pow(bulk, 9.0);
    //calc color
    let color = mix(INNERCOLOR, OUTERCOLOR, d * 2.0);
	
    return (bulk * arms * color + core + bulk * 0.25 * mix(color, WHITE,0.5)) * OVERBRIGHT;
}