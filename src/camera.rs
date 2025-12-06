use bevy::{
    prelude::*,
    input:: {
        mouse::{MouseMotion, MouseWheel},
        common_conditions::input_pressed,
    },

    core_pipeline::{
        Skybox, 
    },
    post_process::motion_blur::MotionBlur
    // pbr::Atmosphere
};

use avian3d::prelude::PhysicsSystems;
// use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::shared::{GameState, OptionIndex, Player};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, 
            follow
            .after(PhysicsSystems::Writeback)
            .before(TransformSystems::Propagate)
            .run_if(in_state(GameState::Game))
        )
        .add_systems(Update, mouse_drag
            .run_if(input_pressed(MouseButton::Left))
            .run_if(on_message::<MouseMotion>)
            .run_if(in_state(GameState::Game))
        )
        .add_systems(Update, distancing
            .run_if(on_message::<MouseWheel>)
            .run_if(in_state(GameState::Game))
        ) 
        .add_observer(cam_reset)   
        .add_systems(Update, opt_index_changed.run_if(resource_changed::<OptionIndex>))
        ; 
    }
} 

// ---

#[derive(Component)]
pub struct Cam;

#[derive(Event)]
pub struct CamReset;

#[derive(Resource)]
pub struct CamFollowParams {
    pub tranlation_bias: Vec3,
    pub look_bias: Vec3,
    pub translation_speed: f32,
    pub rotation_speed: f32
}

// ---

fn setup (
    mut cmd: Commands,
    
) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(100., 0., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        Cam,
        Camera::default(),
        // PanOrbitCamera::default(),
        // NoIndirectDrawing
        
    ));

    cmd.insert_resource(
        CamFollowParams{
            tranlation_bias: Vec3::new(0., 3., 15.),
            look_bias: Vec3::new(0., 4.5, 0.),
            translation_speed: 3.,
            rotation_speed: 8.
        }
    );

}

// ---

#[allow(dead_code)]
fn follow (
    focus_q: Single<&Transform , With<Player>>,
    cam_q: Single<&mut Transform, (With<Cam>, Without<Player>)>,
    cam_param: Res<CamFollowParams>,
    time: Res<Time>,
) {

    let focus_t = focus_q.into_inner(); 
    let mut cam_t = cam_q.into_inner();

    let desired = focus_t.translation +  focus_t.rotation.mul_vec3(cam_param.tranlation_bias);

    cam_t.translation = cam_t.translation.lerp(desired, time.delta_secs() * cam_param.translation_speed);
    let look_at = focus_t.translation + focus_t.rotation.mul_vec3(cam_param.look_bias);

    cam_t.rotation = cam_t.rotation.slerp(cam_t.looking_at(look_at, Vec3::Y).rotation, time.delta_secs() * cam_param.rotation_speed);
}


#[allow(dead_code)]
fn mouse_drag (
    mut er: MessageReader<MouseMotion>,
    mut cam_param: ResMut<CamFollowParams>,
    time: Res<Time>,
) {
    let total_delta :Vec2 = er.read().map(|e|  e.delta).sum();
    if total_delta == Vec2::ZERO {
        return;
    }
    // let yaw = -total_delta.x * time.delta_secs() * 0.1;
    let yaw = 0.;
    let pitch = -total_delta.y * time.delta_secs() * 0.1;
    cam_param.tranlation_bias =  Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0).mul_vec3(cam_param.tranlation_bias);
}

// ---

#[allow(dead_code)]
fn distancing (
    mut er: MessageReader<MouseWheel>,
    mut cp: ResMut<CamFollowParams>
) {
    for e in er.read() {
        let MouseWheel{y, ..} = *e;
        cp.tranlation_bias *= if y > 0. {0.9}  else {1.1};
    }
}

// ---

#[allow(dead_code)]
fn cam_reset(
    _tr: On<CamReset>,
    mut cp: ResMut<CamFollowParams>
) {
    cp.tranlation_bias.x = 0.;
    cp.tranlation_bias.z = cp.tranlation_bias.z.abs();
    // cp.tranlation_bias.y = 2.;
}

// ---

fn opt_index_changed (
    mut cmd: Commands,
    assets: ResMut<AssetServer> ,
    cam_q: Single<Entity, With<Cam>>,
    opt_index: Res<OptionIndex>
) {

    if opt_index.0 == 1 {

        let cam_e = cam_q.into_inner();
        cmd.entity(cam_e).insert((
            Skybox {
                // image: assets.load("skyboxes/interstellar_blue.ktx2"),
                image: assets.load("skyboxes/space_green.ktx2"),
                brightness: 500.,
                ..default()
            },

            MotionBlur::default(),
            // DistanceFog {
            //     color: Color::srgb(0.25, 0.25, 0.25),
            //     falloff: FogFalloff::Linear {
            //         start: 50.0,
            //         end: 200.0,
            //     },
            //     ..default()
            // },
        // Atmosphere::EARTH,

        ));
    }
}