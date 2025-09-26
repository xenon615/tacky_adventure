use bevy::prelude::*;

use crate::help::SetHelpData;
use crate::shared::Exit;
use crate::{shared::{GameStage, SetMonologueText, Player}, ui::UiSlot};
pub struct AimerPlugin;
impl Plugin for AimerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(OnEnter(GameStage::Aimer), (init_ui, set_help))
        .add_systems(Update, update_aimer)
        ;
        
    }
}

// ---

#[derive(Resource)]
pub struct AimerImageHandle(Handle<Image>);

#[derive(Component)]
pub struct ArrowYaw;

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
            let ch = cmd.spawn((
                ArrowYaw,
                ImageNode::new(ihr.0.clone())
            ))
            .id()
            ;
            cmd.entity(e).add_child(ch);
        }
    }
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
    cmd.trigger(SetMonologueText::new("Aimer is available, check out the help"));
}

// ---

fn update_aimer(
    exit_q: Single<&Transform, (With<Exit>, Without<Player>, Without<ArrowYaw>)>,
    player_q: Single<&Transform, (With<Player>, Without<Exit>, Without<ArrowYaw>)>,
    arrow_yaw_q: Single<&mut Transform, (With<ArrowYaw>, Without<Player>, Without<Exit>)>,
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
    arrow_yaw_t.rotation =  arrow_yaw_t.rotation.slerp(Quat::from_rotation_z(angle), time.delta_secs() * 0.5);
}
