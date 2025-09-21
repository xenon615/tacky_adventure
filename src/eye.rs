use bevy::{
    pbr::Material, 
    prelude::*, 
    render::render_resource::{AsBindGroup, ShaderRef}
};

use crate::shared::{GameStage, Player, PLATFORM_DIM, SetMonologueText};

// ---

pub struct EyesPlugin;
impl Plugin for EyesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<EyeMaterial>::default())
        // .add_systems(Startup, startup)
        .add_systems(OnEnter(GameStage::Eye), (startup, set_help))
        .add_systems(Update, (
            moving, 
            change_mode, 
            change_color
        ).run_if(resource_exists::<EnabledEyes>)) 
        .add_systems(Update, check_blink.run_if(any_with_component::<Blinking>))
        ;
    }
} 

//  ---

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EyeMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(1)]
    pub blink: i32
}

impl Material for EyeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/eye.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}

#[derive(Component)]
pub struct Eye {
    idx: u8,
    velocity: f32
}



#[derive(Component, Default, PartialEq, Debug)]
pub enum EyeMode {
    #[default]
    Idle,
    Patrol,
    Chase,
    Attack,
}

const CHASE_TRESHOLD: (f32, f32) = (45., 10.);
const EYES_COUNT: i8 = 12; 
const BASE_VELOCITY: f32 = 5.;
const ANGLE_STEP: f32 = 360. / (EYES_COUNT as f32);
const EYE_Y: f32 = 6.;  


#[derive(Component)]
pub struct Spot;

#[derive(Component)]
pub struct Blinking(Timer);

#[derive(Resource)]
pub struct EnabledEyes;


// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<EyeMaterial>>,
) {

    let mesh_h = meshes.add(Sphere::new(1.));

    for i in 0..EYES_COUNT {
        cmd.spawn((
            Transform::from_xyz((i as f32 + 1.) * 20., EYE_Y, 0.)
            .with_rotation(Quat::from_rotation_y(fastrand::i32(-15 .. 15) as f32))
            ,
            InheritedVisibility::VISIBLE,
            Eye{
                idx: i as u8,
                velocity: BASE_VELOCITY + fastrand::f32().powf(2.)  
            },
            EyeMode::Idle,
            children![
                (
                    Mesh3d(mesh_h.clone()),
                    MeshMaterial3d(materials.add(EyeMaterial {
                        color: Color::hsl(126., 2., 0.5).into(),
                        blink: 1
                    })),
                    Transform::from_rotation(Quat::from_rotation_y(90_f32.to_radians()))
                ),
                (
                    Name::new("Spot"),                    
                    SpotLight {
                        intensity: 10_000_000.,
                        range: 14.3,
                        shadows_enabled: true,
                        inner_angle: - 0.5,
                        outer_angle: 0.6,
                        ..default()
                    },
                    Spot,
                    Transform::from_translation(-Vec3::Z * 4.)
                )
            ]
        ));
               
    }
    cmd.insert_resource(EnabledEyes);

}

// ---

#[allow(dead_code)]
fn gizmos(
    mut gizmos: Gizmos,
    sht: Query<&GlobalTransform, With<Spot>>
) {
    for t in &sht {
        gizmos.axes(*t, 10.);
    }
}

// ---

#[allow(dead_code)]
fn change_color(
    mut cmats: ResMut<Assets<EyeMaterial>>,
    eye_q: Query<(&EyeMode, &Children), Or<(Changed<EyeMode>, Added<EyeMode>)>>,
    mut cmh_q: Query<(Option<&MeshMaterial3d<EyeMaterial>>, Option<&mut SpotLight>)>,
    mut cmd: Commands
) {
    for (e_mode, children) in &eye_q  {
        let color = match e_mode {
            EyeMode::Attack => {
                Color::hsla(0., 1., 0.5, 1.).into()     
            },
            EyeMode::Chase => {
                Color::hsla(60., 1., 0.5, 1.).into() 
            },
            EyeMode::Patrol => {
                Color::hsla(200., 1., 0.5, 1.).into()
            },
            EyeMode::Idle => {
                Color::hsla(270., 1., 0.5, 1.).into()
            }
        };

        for ce in children {
            let (o_cmh, o_sl) = cmh_q.get_mut(*ce).unwrap();

            if let Some(cmh) = o_cmh {
                let Some(m) = cmats.get_mut(cmh) else {continue;};
                m.color = color;
                m.blink = 1;
                cmd.entity(*ce).insert(Blinking(Timer::from_seconds(1., TimerMode::Once)));               
            }

            if let Some(mut sl) = o_sl {
                sl.color = color.into();
            }     
        }
    }
}


// ---

fn calc_desired(idx: u8, target: Vec3) -> Vec3{
    let angle = (ANGLE_STEP  * idx as f32).to_radians();
    let bias = CHASE_TRESHOLD.1 * Vec3::new(angle.cos(), 0., angle.sin());
    (bias + target).with_y(target.y + EYE_Y)

}

// ---

fn change_mode (
    mut eye_q: Query<(&Transform, &mut EyeMode, &Eye)>,
    player_q: Single<&Transform, With<Player>>
) {
    let player_t = player_q.into_inner();

    for (t, mut em, eye) in &mut eye_q {
        let desired = calc_desired(eye.idx, player_t.translation);
        let ds = (desired - t.translation).length_squared();
        if let Some(new_em)  = match *em {
            EyeMode::Idle => Some(EyeMode::Patrol),
            EyeMode::Patrol if ds < CHASE_TRESHOLD.0.powf(2.) => Some(EyeMode::Chase),
            EyeMode::Chase => {
                if ds < 1. {
                    Some(EyeMode::Attack)
                } else {
                    None
                }
            },
            EyeMode::Attack if ds > CHASE_TRESHOLD.1.powf(2.) => Some(EyeMode::Chase),
            _ => None
        } {
            *em = new_em
        }
    }
}

// ---

fn moving ( 
    mut eye_q: Query<(&mut Transform, &EyeMode, &Eye), Without<Player>>,
    player_q: Single<&Transform, With<Player>>,
    time: Res<Time>,
    // mut gizmos: Gizmos
) {
    
    let player_t = player_q.into_inner();
    for (mut t, em, eye ) in &mut eye_q {
        let mut qua = 0.;
        match em {
            EyeMode::Patrol => {
                let center = Vec3::ZERO.with_y(5.);
                
                if t.translation.distance_squared(center) >=  PLATFORM_DIM.z.powf(2.) * 4.
                &&
                 (center - t.translation).normalize().dot(*t.forward()) <= 0.9 
                {
                    t.rotate_y(1.0_f32.to_radians());
                }
                qua = 4.;
            },
            EyeMode::Chase => {
                let desired = calc_desired(eye.idx, player_t.translation);    
                t.rotation = t.rotation.slerp(t.looking_at(desired, Vec3::Y).rotation, time.delta_secs() * 10.);
                qua = 2.;
            },
            EyeMode::Attack => {
                let look_at = player_t.translation + Vec3::Y * 1.2;
                t.rotation = t.rotation.slerp(t.looking_at(look_at, Vec3::Y).rotation, time.delta_secs() * 10.);
            },
            _ => ()
        }

        if qua != 0. {
            let m = t.forward() * time.delta_secs() * qua * eye.velocity;
            t.translation += m;
        }
    }
}

// ---

fn check_blink(
    blink_q: Query<(Entity, &MeshMaterial3d<EyeMaterial>, &mut Blinking)>,
    mut cmats: ResMut<Assets<EyeMaterial>>,
    mut cmd: Commands,
    time: Res<Time>
) {
    for (e, mh, mut b) in blink_q {
        b.0.tick(time.delta());
        if b.0.finished() {
            cmd.entity(e).remove::<Blinking>();
            let Some(m) = cmats.get_mut(mh) else {continue;};
            m.blink = 0;        
        }
    }
}

// ---

fn set_help(
    mut cmd: Commands
) {
    cmd.trigger(SetMonologueText("What the hell is this? Are these guys going to attack me or help me? I don't know yet."));
}
