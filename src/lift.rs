use bevy::prelude::*;
use avian3d::prelude::*;

use crate::shared::MakeLift;
use crate::platform::Platform;

pub struct LiftPlugin;
impl Plugin for LiftPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, move_lift.run_if(any_with_component::<Lift>))
        .add_observer(switch_lift)
        ;
    }
}

// ---

#[derive(Component)]
struct Lift;

// ---

fn switch_lift(
    tr: Trigger<MakeLift>,
    mut platform_q: Query<(Entity, &mut RigidBody, Option<&Lift>), With<Platform>>,
    mut cmd: Commands
) {
    let p_entity = tr.event().0;
    for (pe, mut rb, ol) in platform_q.iter_mut() {
        if (pe == p_entity) && ol.is_none() {
            *rb = RigidBody::Dynamic;
            cmd.entity(pe).insert((
                LockedAxes::ALL_LOCKED.unlock_translation_y(),
                Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
                LinearDamping(2.),
                ExternalForce::new(Vec3::Y * 110.).with_persistence(true),
                Lift 
            ));            
        } else  {
            *rb = RigidBody::Static;
            cmd.entity(pe).remove::<(LockedAxes, Friction, LinearDamping, ExternalForce, Lift)>();            
        }
    }
}

// ---

fn move_lift(
    lift_q: Single<&mut ExternalForce, With<Lift>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    let mut ef = lift_q.into_inner();
    if keys.just_pressed(KeyCode::NumpadAdd) {
        ef.set_force(Vec3::Y * 110.);
    }

    if keys.just_pressed(KeyCode::NumpadSubtract) {
        ef.set_force(Vec3::Y * 90.);
    }
}