use std::time::Duration;

use avian3d::prelude::*;
use bevy::{
    // gizmos, 
    prelude::*, 
    scene::SceneInstanceReady
};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::{*, TnuaAvian3dPlugin};

use crate::{
    monologue::MonologueCont, shared::{DamageCallback, DamageDeal, DamageDealed, GameState, HealthMax, NotReady, Targetable}, ui::{self, UiSlot}};
use bevy_gltf_animator_helper::{AllAnimations, AniData, AnimatorHelperPlugin};

use crate::shared:: {CastBuild, Damage, Player, MessagesAddLine};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_plugins(AnimatorHelperPlugin)
        .add_systems(Startup, (startup, init_ui).after(ui::startup))
        .add_systems(FixedUpdate, (
            apply_controls,
            movement,
            animate
        ).in_set(TnuaUserControlsSystems))
        .add_systems(Update, timer.run_if(any_with_component::<NextAfter>))
        .add_observer(build_action)
        .add_systems(OnEnter(GameState::Game), enter_game)
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
struct NextAfter(Timer, usize);

// ---

fn startup(
    mut cmd: Commands,
    mut all_animations: ResMut<AllAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset: ResMut<AssetServer>
) {
    all_animations.add("Player", "models/player.glb", 7, &mut graphs, &asset);
    cmd.spawn((
        SceneRoot(asset.load(GltfAssetLabel::Scene(0).from_asset("models/player.glb"))),
        Transform::from_xyz(0., 10., 4.).looking_to(-Vec3::Z, Vec3::Y),
        Player,
        Targetable,
        AniData::new("Player", 0),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        RigidBody::Dynamic,
        Collider::compound(vec![
            (Vec3::Y, Quat::IDENTITY,  Collider::capsule(0.5, 1.))
        ]),
        Movement{direction: 0, rotation: 0, jump: false},
        Name::new("Player"),
        HealthMax(100.),
        CollisionEventsEnabled,
        DamageDeal(1.),
        DamageCallback,
     ))
     .insert(NotReady)
     .observe(on_ready)
     .observe(on_damage)
     ;
    
}

// ---

fn on_ready (
    tr: On<SceneInstanceReady>,
    mut cmd: Commands
) {
    cmd.entity(tr.entity).remove::<NotReady>();
    
}

// ---

fn enter_game(
    mut cmd: Commands
) {
    cmd.trigger(MessagesAddLine::<MonologueCont>::new("Hi").with_time(5));
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
    if [4, 5, 6].contains(&ad.animation_index) {
        return;
    } 

    let Some(basis) = tc.dynamic_basis() else {
        return;
    };
    

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
    _tr: On<CastBuild>,
    ad_q: Single<(Entity, &mut AniData), With<Player>>,
    mut cmd: Commands
 ) {
    let (e, mut ad) = ad_q.into_inner();
    ad.animation_index = 4;
    cmd.entity(e).insert(NextAfter(Timer::new(Duration:: from_millis(500), TimerMode::Once), 0));
}

// ---

fn timer (
    timer_q: Single<( Entity, &mut NextAfter, &mut AniData)>,
    mut cmd: Commands,
    time: Res<Time>
) {
    let (e, mut na, mut ad) = timer_q.into_inner();
    na.0.tick(time.delta());
    if na.0.is_finished() {
        cmd.entity(e).remove::<NextAfter>();
        ad.animation_index = na.1;
    }
}

// ---

fn on_damage(
    tr: On<DamageDealed>,
    player_q: Single<(&mut AniData, &Damage, &HealthMax)>, 
    mut cmd: Commands,
    mut next: ResMut<NextState<GameState>>,
    health_ui_q: Single<(&mut Text, &mut TextColor), With<HealthUI>>
) {
    let (mut ad, damage, hm) = player_q.into_inner();
    cmd.trigger(MessagesAddLine::<MonologueCont>::new("Ouch!!").with_time(1));
    if hm.0 - damage.0 <= 0. {
        info!("Game Over");
        ad.animation_index = 5;
        cmd.entity(tr.entity).insert(NextAfter(Timer::new(Duration:: from_millis(1500), TimerMode::Once), 6));       
        next.set(GameState::Over);
    } 

    let h_per = 100. * (1. - (((damage.0 / hm.0) * 100.).round() / 100.));
    let (mut t, mut c) = health_ui_q.into_inner();

    t.0 = format!("Health: {h_per:.0}%");
    c.0.set_hue(h_per); 

}

// ---

#[derive(Component)]
struct HealthUI;

fn init_ui(
    mut cmd: Commands,
    slot_q: Query<(Entity, &UiSlot)>,
) {
    for (e, s) in &slot_q {
        if *s == UiSlot::BottomRight {
            let ch = cmd.spawn((
                HealthUI,
                Text::new("Health: 100%"),
                TextColor(Color::hsl(100., 1.0, 0.5))
            ))
            .id()
            ;
            cmd.entity(e).add_child(ch);
        }
    }
}
