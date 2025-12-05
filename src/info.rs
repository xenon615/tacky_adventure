use bevy:: prelude::*;

use crate::{
    ui,
    messages::set_text
};
    

pub struct InfoPlugin;
impl Plugin for InfoPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup.after(ui::startup))
        .add_observer(set_text::<InfoCont>)
        ;
    }
}

// ---

#[derive(Component)]
pub struct InfoCont;

// ---

fn startup(
    mut cmd: Commands
) {
    cmd.spawn((
        InfoCont,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(25.),
            left: Val::Percent(25.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
    ));
}
