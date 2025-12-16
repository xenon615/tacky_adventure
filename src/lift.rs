use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_hanabi::prelude::*;

use crate:: {
    effects::lift_steam, help::SetHelpData, info::InfoCont,
    platform::get_platform,
    monologue::{MonologueCont, MonoLines},
    player::Player,
    messages::MessagesAddLine,
    stage::{StageIndex, stage_index_changed}
};

pub struct LiftPlugin;
impl Plugin for LiftPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, move_lift.run_if(any_with_component::<Lift>))
        .add_systems(Update, (prepare_effect, set_help, add_lines).run_if(resource_added::<EnabledLift>))
        .add_systems(Update, stage_index_changed::<3, EnabledLift>.run_if(resource_changed::<StageIndex>))
        .add_systems(Update, switch_lift
            .run_if(input_just_pressed(KeyCode::KeyL))
            .run_if(resource_exists::<EnabledLift>)
        )
        ;
    }
}

// ---

#[derive(Component)]
struct Lift;

#[derive(Component)]
struct LiftEffect;

#[derive(Resource, Default)]
struct EnabledLift;

const FORCE_UP: f32 = 150.;
const FORCE_DOWN: f32 = 25.;
const FORCE_NEUTRAL: f32 = 100.;

// ---

fn move_lift(
    lift_q: Single<&mut ConstantForce, With<Lift>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    let mut ef = lift_q.into_inner();
    if keys.just_pressed(KeyCode::PageUp) {
        ef.0 = Vec3::Y * FORCE_UP;
    }

    if keys.just_pressed(KeyCode::PageDown) {
        ef.0  = Vec3::Y * FORCE_DOWN;
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
        ParticleEffect::new(effects.add(lift_steam())),
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
    mut cmd: Commands,
    mut done: Local<bool>
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
        .remove::<(LockedAxes, Friction, LinearDamping, ConstantForce, Lift)>();            
    }

    if !switch_off {
        es.active = true;
        cmd.entity(platform_e).insert((
            RigidBody::Dynamic,
            LockedAxes::ALL_LOCKED.unlock_translation_y(),
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
            LinearDamping(4.),
            ConstantForce(Vec3::Y * FORCE_NEUTRAL),
            Lift 
        )).add_child(ee);

    } else {
        es.active = false;
    }
    
    if !*done {
        cmd.trigger(MessagesAddLine::<MonologueCont>::new("Woo-hoo!").with_time(5));
        *done = true;
    }

}


fn set_help(
    mut cmd: Commands
) {
    cmd.trigger(SetHelpData{
        title: "Lift", 
        keys: "L (On / Off), Page(Up), Page(Down)",
        hint: "use the lift to go up or down"
    });
    cmd.trigger(MessagesAddLine::<InfoCont>::new("Lift is available, check out the help"));
}

// ---

fn add_lines(
    mut mono_lines: ResMut<MonoLines>
) {
    mono_lines.0 =  vec![
        "An lift is not bad, I will build less.",
        "Although this pink smoke is completely tastelessness"
    ];
}
