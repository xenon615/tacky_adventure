use bevy::math::VectorSpace;
use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_hanabi::prelude::*;

use crate::shared::MakeLift;
use crate::platform::Platform;
use crate::effects::green_steam;

pub struct LiftPlugin;
impl Plugin for LiftPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, prepare_effect)
        .add_systems(Update, move_lift.run_if(any_with_component::<Lift>))
        .add_observer(switch_lift)
        ;
    }
}

// ---

#[derive(Component)]
struct Lift;

#[derive(Component)]
struct LiftEffect;

const FORCE_UP: f32 = 50.;
const FORCE_DOWN: f32 = 25.;

// ---

// fn switch_lift(
//     tr: Trigger<MakeLift>,
//     mut platform_q: Query<(Entity, &mut RigidBody, Option<&Lift>), With<Platform>>,
//     effect_q: Single<(Entity, &mut EffectSpawner), With<LiftEffect>>,
//     mut cmd: Commands
// ) {
//     let p_entity = tr.event().0;

//     let (ee, mut es)  = effect_q.into_inner();

//     for (pe, mut rb, ol) in platform_q.iter_mut() {
//         if (pe == p_entity) && ol.is_none() {
//             *rb = RigidBody::Dynamic;
//             es.active = true;

//             cmd.entity(pe).insert((
//                 LockedAxes::ALL_LOCKED.unlock_translation_y(),
//                 Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
//                 LinearDamping(2.),
//                 ExternalForce::new(Vec3::Y * FORCE_UP).with_persistence(true),
//                 Lift 
//             )).add_child(ee)
//             ;            
//         } else  {
//             cmd.entity(pe).remove_children(&[ee]);
//             *rb = RigidBody::Static;
//             cmd.entity(pe).remove::<(LockedAxes, Friction, LinearDamping, ExternalForce, Lift)>();            
//         }
//     }
// }

fn switch_lift(
    tr: Trigger<MakeLift>,
    mut lift_q: Query<Entity, With<Lift>>,
    effect_q: Single<(Entity, &mut EffectSpawner), With<LiftEffect>>,
    mut cmd: Commands
) {
    let (ee, mut es)  = effect_q.into_inner();
    let p_entity = tr.event().0;
    let mut switch_off = false;

    for l_e  in lift_q.iter_mut() {
        if l_e == p_entity {
            switch_off = true
        }
        cmd.entity(l_e).remove_children(&[ee]);
        cmd.entity(l_e)
        .insert(RigidBody::Static)
        .remove::<(LockedAxes, Friction, LinearDamping, ExternalForce, Lift)>();            
    }

    if !switch_off {
        es.active = true;
        cmd.entity(p_entity).insert((
            RigidBody::Dynamic,
            LockedAxes::ALL_LOCKED.unlock_translation_y(),
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
            LinearDamping(2.),
            ExternalForce::new(Vec3::Y * FORCE_UP).with_persistence(true),
            Lift 
        )).add_child(ee);

    } else {
        es.active = false;
    }
    
}


// ---

fn move_lift(
    lift_q: Single<&mut ExternalForce, With<Lift>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    let mut ef = lift_q.into_inner();
    if keys.just_pressed(KeyCode::NumpadAdd) {
        ef.set_force(Vec3::Y * FORCE_UP);
    }

    if keys.just_pressed(KeyCode::NumpadSubtract) {
        ef.set_force(Vec3::Y * FORCE_DOWN);
    }
}

// ---

fn prepare_effect(
    mut cmd: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    assets: ResMut<AssetServer>

) {
        cmd.spawn((
        Name::new("circle"),
        ParticleEffect::new(effects.add(green_steam())),
        Transform::from_xyz(0., -0.5, 0.),
        EffectProperties::default(),
        EffectMaterial{
            images: vec![
                assets.load("textures/cloud.png"),
            ]
        },
        LiftEffect
    ));        

}