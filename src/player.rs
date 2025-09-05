use std::time::Duration;

use avian3d::prelude::*;
use bevy::{
    // gizmos, 
    // pbr:: {NotShadowCaster, NotShadowReceiver}, 
    color::palettes::basic, input::{keyboard::KeyboardInput, mouse::MouseMotion}, prelude::*
};
use bevy_tnua::{prelude::*, TnuaAnimatingState};
use bevy_tnua_avian3d::*;

use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use bevy_gltf_animator_helper::{AllAnimations, AniData, AnimatorHelperPlugin};

use crate::shared::{Build, CastBuild};
use crate::shared::{MakeLift, Player};


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
        .add_systems(Update, actions.run_if(on_event::<KeyboardInput>))
        .add_systems(Update, timer.run_if(any_with_component::<Interval>))
        .add_observer(build_action)
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
        Transform::from_xyz(0., 50., 0.)
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
        Name::new("Player")
     ))
     ;
}

// ---

fn apply_controls(
    keys: Res<ButtonInput<KeyCode>>,
    player_q: Single<&mut Movement, With<Player>>,
) {
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
    let (player_transform, movement  ,mut controller) = player_q.into_inner();
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


// fn animate(
//     player_q: Single<(&Transform, &mut AniData, &TnuaController), With<Player>>
// ) {
    
//     let (t, mut ad, tc) = player_q.into_inner();
//     // println!("{:?}", ad.animation_key);
//     let Some(basis) = tc.dynamic_basis()  else {
//         return;
//     };
    
//     let back = t.forward().dot(basis.effective_velocity().normalize()) < 0.;

//     let candidate = if basis.is_airborne() {
//         2
//     } else if basis.effective_velocity().length() > 0.1 {
//         if back {3} else {1}
//     } else {0};

//     if candidate != ad.animation_index {
//         ad.animation_index = candidate;
//     }
// }

// ---

// fn actions(
//     keys: Res<ButtonInput<KeyCode>>,
//     mut cmd: Commands,
//     raycast_q: SpatialQuery,
//     player_q: Single<&Transform, With<Player>>,
// ) {
//     if keys.pressed(KeyCode::KeyL) {
//         let player_t = player_q.into_inner();
//         if let Some(hit) = raycast_q.cast_ray(
//             player_t.translation + player_t.forward() * 1., 
//             Dir3::NEG_Y,
//             f32::MAX,
//             false, 
//             &SpatialQueryFilter::default()
//         ) {
//             cmd.trigger(MakeLift(hit.entity));
//         }
//     }
// }

// ---

fn get_platform(pt: &Transform, raycast_q: &SpatialQuery) -> Option<RayHitData> {
    raycast_q.cast_ray(
        // pt.translation + pt.forward() * 2., 
        pt.translation + pt.down() * 0.01, 
        Dir3::NEG_Y,
        f32::MAX,
        false, 
        &SpatialQueryFilter::default()
    )
}

// ---

fn actions(
    keys: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    raycast_q: SpatialQuery,
    player_q: Single<&Transform, With<Player>>,
) {
    if keys.pressed(KeyCode::KeyL) {
        let player_t = player_q.into_inner();
        if let Some(hit) = get_platform(player_t, &raycast_q) {
            cmd.trigger(MakeLift(hit.entity));
        }
    }
}

// ---

fn build_action(
    tr: Trigger<CastBuild>,
    ad_q: Single<(Entity, &mut AniData, &Transform), With<Player>>,
    raycast_q: SpatialQuery,
    mut cmd: Commands
 ) {
    let (e, mut ad, t) = ad_q.into_inner();
    let Some(hit) = get_platform(t, &raycast_q) else {
        println!("no platform");
        return;
    };

    cmd.trigger(Build(tr.event().0, hit.entity, t.forward()));
    
    ad.animation_index = 4;
    cmd.entity(e).insert(Interval(Timer::new(Duration:: from_secs(2), TimerMode::Once)));
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