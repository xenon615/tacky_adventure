use bevy::prelude::*;

use crate::{
    camera::Cam, 
    shared::GameState,
    monologue::MonoLines,
    player::Player
};
pub struct IntroPlugin;
impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Intro), (startup, add_lines))
        .add_systems(FixedUpdate, camera_moving.run_if(in_state(GameState::Intro)))
        ;
    }
}

// ---

fn startup(
    cam_q: Single<&mut Transform, With<Cam>>,
    
) {
    *cam_q.into_inner() = Transform::from_xyz(0., 200., 0.).looking_at(Vec3::ZERO, Vec3::Y);
}

// ---

fn add_lines(
    mut mono_lines: ResMut<MonoLines>
) {
    mono_lines.0 = vec![
        "What a strange place?",
        "I wonder how I ended up here.",
        "Probably again the fault of this idiot who thinks he is able to create realities.",
        "What was his name?",
        "God, demiurge, Sir Max?",
        "Never mind, let's take a look around",
        "A path leading to a strange, shimmering thing and overgrown flying dumplings.",
        "Everything is pale, I'm the only one here, blue as an drunkard's nose on a winter morning.",
        "Complete bad taste, in short.",
        "I guess I should go ahead .."
    ];
}

// ---

fn camera_moving (
    cam_q: Single<&mut Transform, (With<Cam>, Without<Player>)>,
    player_q: Single<&Transform , (Without<Cam>, With<Player>)>,
    time: Res<Time>,
    mut minus_radius: Local<f32>,
    mut minus_y : Local<f32>,
    mut next: ResMut<NextState<GameState>>
) {
    let mut cam_t = cam_q.into_inner();
    let player_t = player_q.into_inner();


    cam_t.rotation = cam_t.rotation.slerp(
       cam_t.looking_at(player_t.translation.with_y(5.), Vec3::Y).rotation,
       time.delta_secs() * 10.
    );


    let radius = 50. - *minus_radius;
    let y = 50. - *minus_y;
    if y < 3. {
        next.set(GameState::Game);
        return;
    }

    cam_t.translation =  player_t.translation + Quat::from_rotation_y(time.elapsed_secs() * 0.5).mul_vec3(radius * Vec3::Z).with_y(y)
    ;

    *minus_radius += 2. * time.delta_secs();
    *minus_y += 2. * time.delta_secs();


}