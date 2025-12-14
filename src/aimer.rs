use bevy::prelude::*;
use crate:: {
    help::SetHelpData,
    info::InfoCont,
    shared::{StageIndex,  MessagesAddLine, Player, Exit}, 
    ui::UiSlot,
    monologue::MonoLines
};

pub struct AimerPlugin;
impl Plugin for AimerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(Update, (init_ui, set_help, add_lines).run_if(resource_added::<EnabledAimer>))
        .add_systems(Update, opt_index_changed.run_if(resource_changed::<StageIndex>))
        .add_systems(Update, update_aimer.run_if(resource_exists::<EnabledAimer>))
        ;
        
    }
}

// ---

#[derive(Resource)]
pub struct AimerImageHandle(Handle<Image>);

#[derive(Component)]
pub struct ArrowYaw;

#[derive(Component)]
pub struct Elevation;


#[derive(Resource)]
pub struct EnabledAimer;


// ---

fn startup(
    mut cmd: Commands,
    assets: ResMut<AssetServer>
) {
    cmd.insert_resource(AimerImageHandle(assets.load("images/arrow2.png")));
}

// ---

fn init_ui(
    mut cmd: Commands,
    slot_q: Query<(Entity, &UiSlot)>,
    ihr: Res<AimerImageHandle>
) {
    for (e, s) in &slot_q {
        if *s == UiSlot::TopRight {
            let ch1 = cmd.spawn((
                ArrowYaw,
                ImageNode::new(ihr.0.clone()),
            ))
            .id()
            ;
            let ch2 = cmd.spawn((
                Elevation,
                Node{
                    padding: UiRect::default().with_left(Val::Px(10.)).with_top(Val::Px(10.)),
                    ..default()
                },
                
                children![
                    (Text::new(""), Elevation)
                ]
                
            ))
            .id()
            ;

            cmd.entity(e).add_child(ch1);
            cmd.entity(e).add_child(ch2);
        }
    }
    cmd.insert_resource(EnabledAimer);
}

// ---

fn set_help(
    mut cmd: Commands
) {
    cmd.trigger(SetHelpData{
        title: "Aimer", 
        keys: "",
        hint: "the aimer indicates the direction to the target"
    });
    cmd.trigger(MessagesAddLine::<InfoCont>::new("Aimer is available, check out the help"));
}

// ---

fn update_aimer(
    exit_q: Single<&Transform, (With<Exit>, Without<Player>, Without<ArrowYaw>)>,
    player_q: Single<&Transform, (With<Player>, Without<Exit>, Without<ArrowYaw>)>,
    arrow_yaw_q: Single<&mut UiTransform, (With<ArrowYaw>, Without<Player>, Without<Exit>)>,
    elevation_text: Single<&mut Text, With<Elevation>>,
    time: Res<Time>
) {
    let exit_t = exit_q.into_inner();
    let player_t = player_q.into_inner();
    let mut arrow_yaw_t  = arrow_yaw_q.into_inner();
    let to_target = exit_t.translation - player_t.translation;
    let to_target_xz = to_target.normalize().reject_from_normalized(Vec3::Y);
    let forward_xz:Vec3 = player_t.forward().into();
    let dot = to_target_xz.dot(forward_xz);
    let sign = to_target_xz.cross(forward_xz).y.signum();
    let angle = dot.acos() * sign;
    arrow_yaw_t.rotation =  arrow_yaw_t.rotation.nlerp(Rot2::radians(angle), time.delta_secs() * 0.5);


    let elevation_update = match exit_t.translation.y - player_t.translation.y {
        x if x > 2. => Some("Higher"),
        x if x < -2. => Some("Lower"),
        _ => None
    };
    if let Some(t) = elevation_update {
       elevation_text.into_inner().0 = t.into(); 
    }
}

// --

const OPTION_INDEX: usize = 2;

fn opt_index_changed(
    opt_index: Res<StageIndex>,
    mut cmd: Commands
) {
    if opt_index.0 == OPTION_INDEX {
        cmd.insert_resource(EnabledAimer);
    }
} 

// ---

fn add_lines(
    mut mono_lines: ResMut<MonoLines>
) {
    mono_lines.0 =  vec![
        "Now it's easier for me to understand where to go.",
        "This is a really useful feature."
    ];
}
