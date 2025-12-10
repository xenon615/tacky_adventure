use bevy::{color::palettes, prelude::*};

use crate::{
    camera::Cam, 
    shared::{GameState, Player}
};
pub struct IntroPlugin;
impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Intro), startup)
        .add_systems(FixedUpdate, camera_moving.run_if(in_state(GameState::Intro)))
        ;
    }
}

// ---

// #[derive(Component)]
// struct Cam1;

fn startup(
    cam_q: Single<&mut Transform, With<Cam>>,
    // mut cmd: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cam_q.into_inner().translation = Vec3::new(100., 100., 100.);

    *cam_q.into_inner() = Transform::from_xyz(0., 200., 0.).looking_at(Vec3::ZERO, Vec3::Y);

    // cmd.spawn((
    //     Mesh3d(meshes.add(Sphere::new(1.0))),
    //     MeshMaterial3d(materials.add(Color::WHITE)),
    //     Cam1,
    //     Transform::from_xyz(30., 50., 30.)
    // ));
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
       cam_t.looking_at(player_t.translation, Vec3::Y).rotation,
       time.delta_secs() * 10.
    );


    let radius = 50. - *minus_radius;
    let y = 50. - *minus_y;
    if y < 3. {
        next.set(GameState::Game);
        return;
    }

    cam_t.translation =  player_t.translation + Quat::from_rotation_y(time.elapsed_secs()).mul_vec3(radius * Vec3::Z).with_y(y)
    ;

    *minus_radius += 2. * time.delta_secs();
    *minus_y += 12. * time.delta_secs();


}