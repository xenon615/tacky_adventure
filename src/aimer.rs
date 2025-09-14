use bevy::prelude::*;

use crate::help::SetHelpData;
use crate::{shared::{GameStage, SetMonologueText}, ui::UiSlot};
pub struct AimerPlugin;
impl Plugin for AimerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .add_systems(OnEnter(GameStage::Aimer), (init, set_help))
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
    cmd.insert_resource(AimerImageHandle(assets.load("images/arrow.png")));
}

// ---

fn init(
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

fn set_help(
    mut cmd: Commands
) {
    cmd.trigger(SetHelpData{
        title: "Aimer", 
        keys: "",
        hint: "the aimer indicates the direction to the target"
    });
    cmd.trigger(SetMonologueText("Aimer is available, check out the help"));
}
