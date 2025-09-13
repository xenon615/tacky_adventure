use bevy::prelude::*;

use crate::{shared::GameStage, ui::UiSlots};
pub struct HelpPlugin;
impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, toggle_help
            .run_if(resource_changed::<ButtonInput<KeyCode>>)
            .run_if(not(in_state(GameStage::One)))
        )
        .add_systems(Update, init.run_if(resource_changed::<HelpData>))
        ;
    }
}

// ---

#[derive(Resource)]
pub struct HelpData(Vec<String>);


#[derive(Component)]
pub struct HelpWidget;

// ---

fn init (
    mut cmd: Commands,
    slots_q: Query<(Entity, &UiSlots)>,
    ready: Local<bool>
) {
    if *ready {
        return;
    }
    for (e, us) in  &slots_q {
        if *us == UiSlots::Middle {
            let ch = cmd.spawn((
                HelpWidget,
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                }
            )).id();
            cmd.entity(e).add_child(ch);
        }

        if *us == UiSlots::BottomLeft {
            let ch = cmd.spawn(
                Text::new("Alt + H: Help")
            ).id();
            cmd.entity(e).add_child(ch);
        }

    }

}


// fn toggle_help (
//     v_q: Single<&mut Visibility, With<MenuRoot>>,
//     keys: Res<ButtonInput<KeyCode>>
// ) {
//     if keys.just_pressed(KeyCode::KeyM) && keys.pressed(KeyCode::AltLeft) {
//         v_q.into_inner().toggle_visible_hidden();
//     }
// } 
