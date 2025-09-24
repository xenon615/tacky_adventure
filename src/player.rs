use std::time::Duration;

use avian3d::prelude::*;
use bevy::{
    // gizmos, 
    // pbr:: {NotShadowCaster, NotShadowReceiver}, 
    color::palettes::basic, ecs::system::entity_command::observe, input::{keyboard::KeyboardInput, mouse::MouseMotion}, prelude::*, scene::SceneInstanceReady
};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::{
    *,
    TnuaAvian3dPlugin
};

// use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use bevy_gltf_animator_helper::{AllAnimations, AniData, AnimatorHelperPlugin};

use crate::shared:: {CastBuild, MaxHealth, Damage ,Player, SetDamage, SetMonologueText};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_plugins(AnimatorHelperPlugin)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, (
            apply_controls,
            movement,
            animate
        ).in_set(TnuaUserControlsSystemSet))
        .add_systems(Update, timer.run_if(any_with_component::<Interval>))
        .add_observer(build_action)
        .add_observer(on_damage)
        ;
    }
}

// ---




#[derive(Component)]
pub struct Movement {
    direction: i8,
    rotation: i8,
    jump: bool
}

#[derive(Component)]
struct Interval(Timer);

// ---

fn startup(
    mut cmd: Commands,
    mut all_animations: ResMut<AllAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset: ResMut<AssetServer>
) {
    all_animations.add("Player", "models/player.glb", 5, &mut graphs, &asset);
    cmd.spawn((
        SceneRoot(asset.load(GltfAssetLabel::Scene(0).from_asset("models/player.glb"))),
        Transform::from_xyz(0., 10., 4.)
        .looking_to(-Vec3::Z, Vec3::Y),
        Player,
        AniData::new("Player", 1),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        RigidBody::Dynamic,
        Collider::compound(vec![
            (Vec3::Y, Quat::IDENTITY,  Collider::capsule(0.5, 1.))
        ]),
        Movement{direction: 0, rotation: 0, jump: false},
        Name::new("Player"),

     ))
     .observe(on_ready)
     ;
}

// ---

fn on_ready (
    _: Trigger<SceneInstanceReady>,
    mut cmd: Commands
) {
    cmd.trigger(SetMonologueText::new("Hi").with_time(20));
}

// ---

fn apply_controls(
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Single<&mut Movement, With<Player>>,
) {
    if keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
        return;
    }
    
    let forward_keys = [KeyCode::ArrowUp,  KeyCode::KeyW];
    let back_keys = [KeyCode::ArrowDown,  KeyCode::KeyS];
    let right_keys = [KeyCode::ArrowRight,  KeyCode::KeyD];
    let left_keys = [KeyCode::ArrowLeft,  KeyCode::KeyA];

    let forward = keys.any_pressed(forward_keys);
    let back  = keys.any_pressed(back_keys);

    let right = keys.any_pressed(right_keys);
    let left = keys.any_pressed(left_keys);

    let direction = forward as i8 - back as i8;
    let rotation = right as i8 - left as i8;

    let jump = keys.pressed(KeyCode::Space);
    let mut m = player_q.into_inner();
    m.direction = direction;
    m.rotation = rotation;
    m.jump = jump;

}

// ---

fn movement(
    player_q: Single<(&Transform, &Movement, &mut TnuaController), With<Player>>
) {
    let (player_transform, movement, mut controller) = player_q.into_inner();
    let desired_forward =  Quat::from_rotation_y(movement.rotation as f32 * -3_f32.to_radians()).mul_vec3(player_transform.forward() * 1.);

    let speed = if movement.direction > 0 {16.} else if movement.direction < 0  {-4.} else { 0.}; 

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: player_transform.forward() * speed, 
        float_height: 0.1,
        desired_forward: Dir3::new(desired_forward.normalize()).ok(),
        ..default()
    });

    if movement.jump {
        controller.action(TnuaBuiltinJump {
            height: 4.0,
            ..Default::default()
        });
    }
}

// ---

fn animate(
    player_q: Single<(&Transform, &mut AniData, &TnuaController), With<Player>>
) {
    
    let (t, mut ad, tc) = player_q.into_inner();
    if ad.animation_index == 4 {
        return;
    }

    let Some(basis) = tc.dynamic_basis() else {
        return;
    };
    
    if ad.animation_index == 4 {
        return;
    }

    let back = t.forward().dot(basis.effective_velocity().normalize()) < 0.;

    let candidate = if basis.is_airborne() {
        2
    }  else if basis.effective_velocity().length_squared() > 0.1 {
        if back {3} else {1}
    } else {
        0
    };

    if candidate != ad.animation_index {
        ad.animation_index = candidate;
    }
}

// ---

fn build_action(
    _tr: Trigger<CastBuild>,
    ad_q: Single<(Entity, &mut AniData), With<Player>>,
    mut cmd: Commands
 ) {
    let (e, mut ad) = ad_q.into_inner();
    ad.animation_index = 4;
    cmd.entity(e).insert(Interval(Timer::new(Duration:: from_millis(500), TimerMode::Once)));
}



// ---

fn timer (
    timer_q: Single<( Entity, &mut Interval, &mut AniData)>,
    mut cmd: Commands,
    time: Res<Time>
) {
    let (e, mut i, mut ad) = timer_q.into_inner();
    i.0.tick(time.delta());
    if i.0.finished() {
        cmd.entity(e).remove::<Interval>();
        ad.animation_index = 0;
    }
}

// ---

fn on_damage(
    tr: Trigger<SetDamage>,
    player_q: Single<(&mut Damage, &MaxHealth)>, 
    mut cmd: Commands
) {
    let (mut damage, max_health) = player_q.into_inner();
    cmd.trigger(SetMonologueText::new("Ouch!!"));
    damage.0 += tr.event().0;
    if max_health.0 - damage.0 <= 0. {
        info!("Game Over");
    } 

}