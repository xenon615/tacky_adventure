use bevy::prelude::*;
use avian3d::prelude::*;
use std::ops::Range;

// #[allow(dead_code)]
// pub fn color2vec4(c: Color) -> Vec4 {
//     let color = c.to_srgba();
//     Vec4::new(color.red, color.green, color.blue, color.alpha)
// }

#[derive(Event)]
pub struct CastBuild;

// #[derive(Component)]
// #[component(immutable)]
// pub struct MaxHealth(pub f32);
// impl Default for MaxHealth {
//     fn default() -> Self {
//         Self(100.)
//     }
// }
#[derive(Component, Default)]
pub struct Damage(pub f32);


#[derive(Component)]
#[require(Damage)]
pub struct Player;

#[derive(Event)]
pub struct SetDamage(pub f32);

#[derive(Component, Default)]
pub struct Threat;

#[derive(Event)]
pub struct SetMonologueText<'a>{pub text: &'a str, pub time: u64}
impl <'a>SetMonologueText<'a>  {
    pub fn new(text: &'a str) -> Self {
        Self { text , time: 5 }
    }

    pub fn with_time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameStage {
    #[default]            
    Intro,
    Build,
    Lift,
    Aimer,
    Eye,
    Virus,
    Over
}

impl GameStage {
    pub fn get_index_by_state(s: &Self) -> u32{
        match s {
            Self::Intro => 0,
            Self::Build => 1,
            Self::Lift => 2,
            Self::Aimer => 3,
            Self::Eye => 4,
            Self::Virus => 5,
            Self::Over => 6
        }
    }

    pub fn get_state_by_index(index: u32) -> GameStage{
        match index {
            0 => Self::Intro,
            1 => Self::Build,
            2 => Self::Lift,
            3 => Self::Aimer,
            4 => Self::Eye,
            5 => Self::Virus,
            6 => Self::Over,
            _ => Self::Intro
        }

    }

}

pub const PLATFORM_DIM: Vec3 = Vec3::new(10., 0.1, 10.);

#[derive(Component)]
pub struct Exit;

// ---

pub fn get_platform(pt: &Transform, raycast_q: &SpatialQuery) -> Option<RayHitData> {
    raycast_q.cast_ray(
        pt.translation + pt.down() * 0.01, 
        Dir3::NEG_Y,
        f32::MAX,
        false, 
        &SpatialQueryFilter::default()
    )
}

// ---

pub fn fibonacci_sphere(count: usize) -> impl Iterator<Item = Vec3> {
    let phi = std::f32::consts::PI * (5.0_f32.sqrt() - 1.);
    (0 .. count).map(move |i| {
        let y = 1. - (i as f32 / (count - 1) as f32) * 2.;  
        let radius = (1. - y * y).sqrt();
        let theta = phi * i as f32;
        let x = theta.cos() * radius;
        let z = theta.sin() * radius;
        Vec3::new(x, y, z)
    })
} 

// ---

pub fn closest (verticis: &mut Vec<[f32; 3]>, p: Vec3, scale: f32 ) {
    if let Some(i) = verticis
        .iter()
        .enumerate()
        .map(|(idx, c)| (idx, Vec3::from_array(*c)))
        .min_by(| (_, a), (_, b) | {
            let ad = a.distance_squared(p);
            let bd = b.distance_squared(p);
            ad.total_cmp(&bd)
        }) 
        .map(|(idx, _)| idx) {
        verticis[i].iter_mut().for_each(|c| {
            *c *= scale;
        });                
    }
}

pub fn vec_rnd(rx: Range<i32>, ry: Range<i32>, rz: Range<i32>) -> Vec3{
    Vec3::new(
        fastrand::i32(rx) as _ , 
        fastrand::i32(ry) as _, 
        fastrand::i32(rz) as _
    )
}

// ---
