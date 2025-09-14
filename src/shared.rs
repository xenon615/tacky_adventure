use bevy::prelude::*;
use avian3d::prelude::*;
#[allow(dead_code)]
pub fn color2vec4(c: Color) -> Vec4 {
    let color = c.to_srgba();
    Vec4::new(color.red, color.green, color.blue, color.alpha)
}

#[derive(Event)]
pub struct CastBuild;

#[derive(Component)]
pub struct Player;

#[derive(Event)]

pub struct SetMonologueText<'a>(pub &'a str);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameStage {
    #[default]
    Intro,
    Build,
    Lift,
    Aimer
}

impl GameStage {
    pub fn get_index_by_state(s: &Self) -> u32{
        match s {
            Self::Intro => 0,
            Self::Build => 1,
            Self::Lift => 2,
            Self::Aimer => 3
        }
    }

    pub fn get_state_by_index(index: usize) -> GameStage{
        match index {
            0 => Self::Intro,
            1 => Self::Build,
            2 => Self::Lift,
            3 => Self::Aimer,
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
