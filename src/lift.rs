use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_hanabi::prelude::*;

use crate:: {
    effects::steam, 
    help::SetHelpData, 

    shared::{get_platform, GameStage,  Player, SetMonologueText}
};




pub struct LiftPlugin;
impl Plugin for LiftPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, move_lift.run_if(any_with_component::<Lift>))
        .add_systems(OnEnter(GameStage::Lift), (prepare_effect, set_help))
        .add_systems(Update, switch_lift.run_if(input_just_pressed(KeyCode::KeyL)))
        ;
    }
}

// ---

#[derive(Component)]
struct Lift;

#[derive(Component)]
struct LiftEffect;

const FORCE_UP: f32 = 150.;
const FORCE_DOWN: f32 = 25.;

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
        ParticleEffect::new(effects.add(steam())),
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

// ---

fn switch_lift(
    mut lift_q: Query<Entity, With<Lift>>,
    effect_q: Single<(Entity, &mut EffectSpawner), With<LiftEffect>>,
    player_q: Single<&Transform, With<Player>>,
    spatiaal: SpatialQuery,
    mut cmd: Commands
) {

    let player_t = player_q.into_inner();
    let (ee, mut es)  = effect_q.into_inner();
    let  Some(RayHitData { entity: platform_e, distance: _, normal: _}) = get_platform(player_t, &spatiaal) else {
        return;
    };
    
    let mut switch_off = false;

    for l_e  in lift_q.iter_mut() {
        if l_e == platform_e {
            switch_off = true
        }
        cmd.entity(l_e).remove_children(&[ee]);
        cmd.entity(l_e)
        .insert(RigidBody::Static)
        .remove::<(LockedAxes, Friction, LinearDamping, ExternalForce, Lift)>();            
    }

    if !switch_off {
        es.active = true;
        cmd.entity(platform_e).insert((
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


fn set_help(
    mut cmd: Commands
) {
    cmd.trigger(SetHelpData{
        title: "Lift", 
        keys: "L (On / Off), Num + (Up), Num - (Down)",
        hint: "use the lift to go up or down"
    });
    cmd.trigger(SetMonologueText::new("Lift is available, check out the help"));
}
