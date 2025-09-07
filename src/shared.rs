use bevy::prelude::*;
// use avian3d::prelude::*;
#[allow(dead_code)]
pub fn color2vec4(c: Color) -> Vec4 {
    let color = c.to_srgba();
    Vec4::new(color.red, color.green, color.blue, color.alpha)
}

#[derive(Event)]
pub struct MakeLift(pub Entity);

#[derive(Event)]
pub struct CastBuild(pub BuildAction);

#[derive(Event)]
pub struct Build(pub BuildAction, pub Entity, pub Dir3);


#[derive(Component, Clone, Copy, Debug , PartialEq)]
pub enum BuildAction {
    Forward,
    Up,
    Down,
    Delete
}

#[derive(Component)]
pub struct Player;



