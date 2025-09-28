use bevy::{
    pbr::Material, 
    prelude::*, 
    render::render_resource::{AsBindGroup, ShaderRef},
    color::palettes::css
};

use crate::{shared::{vec_rnd, GameStage, Player, SetMonologueText, Threat} };

// ---

pub struct EyesPlugin;
impl Plugin for EyesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<EyeMaterial>::default())
        .add_systems(OnEnter(GameStage::Build), (startup, set_help))
        .add_systems(Update, (
            change_mode, 
            change_color,
            moving, 
            detect_threat.run_if(not(any_with_component::<Target>)),
            aiming.run_if(any_with_component::<Target>)
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
    Escort,
    Defence,
}

const EYES_COUNT: i8 = 12; 
const ESCORT_RELATIVE: Vec3 = Vec3::new(0., 5., 20.);
const ESCORT_SQUARE_TRESHOLD: f32 = 9.;
const BASE_VELOCITY: f32 = 1.;
const ANGLE_STEP: f32 = 360. / (EYES_COUNT as f32);
const DETECT_RANGE_SQUARED: f32  = 100.0 * 100.0;


#[derive(Component)]
pub struct Spot;

#[derive(Component)]
pub struct Blinking(Timer);

#[derive(Resource)]
pub struct EnabledEyes;

#[derive(Component, Clone)]
pub struct Target(Entity);

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<EyeMaterial>>,
) {

    let mesh_h = meshes.add(Sphere::new(1.));

    for i in 0..EYES_COUNT {
        cmd.spawn((
            Transform::from_translation(vec_rnd(-100 .. 100, -100 .. 100, -100 .. 100)),
            InheritedVisibility::VISIBLE,
            Eye{
                idx: i as u8,
                velocity: BASE_VELOCITY + fastrand::f32().powf(4.)  
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
            EyeMode::Defence => {
                css::RED.into()     
            },
            EyeMode::Escort => {
                css::GREEN_YELLOW.into()
            },
            EyeMode::Idle => {
                css::GREEN.into()
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
    let bias = Quat::from_rotation_y(angle).mul_vec3(ESCORT_RELATIVE);
    bias + target
}

// ---

fn change_mode (
    mut eye_q: Query<(&Transform, &mut EyeMode, &Eye)>,
    player_q: Single<&Transform, (With<Player>, Changed<Transform>)>
) {
    let player_t = player_q.into_inner();

    for (t, mut em, eye) in &mut eye_q {
        if *em == EyeMode::Defence {continue;}
        let desired = calc_desired(eye.idx, player_t.translation);
        let distance_squared = desired.distance_squared(t.translation);
        let crit = distance_squared >= ESCORT_SQUARE_TRESHOLD;
        if let Some(new_em)  = match *em {
            EyeMode::Idle if crit => Some(EyeMode::Escort),
            EyeMode::Escort if !crit => Some(EyeMode::Idle),
            _ => None
        } {
            *em = new_em
        }
    }
}

// ---

fn detect_threat(
    threat_q: Query<(Entity, &Transform), (With<Threat>, Without<Eye>)>,
    mut eyes_q: Query<(Entity, &Transform, &mut EyeMode), (Without<Threat>, Without<Target>)>,
    mut cmd: Commands
) {

    let mut assigned = vec![];
    for (threat_e, threat_t) in threat_q {
        let mut cc = 0;
        for (eye_e, eye_t, mut eye_mode) in eyes_q
        .iter_mut()
         {
            if assigned.contains(&eye_e) { break; }            
            if eye_t.translation.distance_squared(threat_t.translation) <= DETECT_RANGE_SQUARED {
                *eye_mode = EyeMode::Defence;
                cmd.entity(eye_e).insert(Target(threat_e));
                assigned.push(eye_e);
                cc += 1;
                if cc > 2 {break;}
            } 
        }
    } 
}

// ---

fn aiming(
    mut eye_q: Query<(&Target, &mut Transform), Without<Threat>>,
    threat_q: Query<&Transform, With<Threat>>,
    time: Res<Time>
) {
    for  (target_e, mut t) in &mut eye_q {
        let Ok(target_t) = threat_q.get(target_e.0) else {continue;};
        t.rotation = t.rotation.slerp(t.looking_at(target_t.translation, Vec3::Y).rotation, time.delta_secs())
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

        let looking_at = match em {
            EyeMode::Escort => {
                let desired = calc_desired(eye.idx, player_t.translation);
                let qua = desired.distance_squared(t.translation).log2();
                let m = t.forward() * time.delta_secs() * qua * eye.velocity;
                t.translation += m;

                desired
            },
            EyeMode::Idle => {
                player_t.translation + Vec3::Y * 1.2
            },
            _ => Vec3::ZERO
        };

        t.rotation = t.rotation.slerp(t.looking_at(looking_at, Vec3::Y).rotation, time.delta_secs() * 10.);

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
    cmd.trigger(SetMonologueText::new("What the hell is this? Are these guys going to attack me or help me? I don't know yet."));
}
