use bevy::prelude::*;
use std::ops::Range;

#[derive(Component)]
pub struct NotReady;

#[derive(Component, Default)]
pub struct Threat;

#[derive(Component, Clone)]
pub struct Target(pub Entity);

#[derive(Component, Default, Clone)]
pub struct TargetedBy(pub Vec<Entity>);

#[derive(Component, Default, Clone)]
#[require(TargetedBy)]
pub struct Targetable;

// --

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]            
    Loading,
    Intro,
    Game,
    Over
}
// ---

#[allow(dead_code)]
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

#[allow(dead_code)]
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

// ---

pub fn vec_rnd(rx: Range<i32>, ry: Range<i32>, rz: Range<i32>) -> Vec3{
    Vec3::new(
        fastrand::i32(rx) as _ , 
        fastrand::i32(ry) as _, 
        fastrand::i32(rz) as _
    )
}
