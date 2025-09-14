use bevy:: {
    prelude::*,
    color::palettes::css
};

use crate::{shared::GameStage, ui::UiSlot};
pub struct HelpPlugin;
impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, toggle_help
            .run_if(resource_changed::<ButtonInput<KeyCode>>)
            .run_if(not(in_state(GameStage::Intro)))
        )
        .add_observer(init)
        ;
    }
}

// ---

#[derive(Event)]
pub struct SetHelpData<'a> {
    pub title: &'a str,
    pub keys: &'a str,
    pub hint:&'a str 
}

// ---

#[derive(Component)]
pub struct HelpWidget;

// ---

fn init (
    tr: Trigger<SetHelpData>,
    mut cmd: Commands,
    slots_q: Query<(Entity, &UiSlot)>,
    help_q: Option<Single<Entity, With<HelpWidget>>>
) {
    let hwe = match help_q  {
        Some(x) => *x,
        None => {
            let mut h = Entity::PLACEHOLDER;
            for (e, us) in  &slots_q {
                if *us == UiSlot::Middle {
                    h = cmd.spawn((
                        Name::new("Help"),
                        HelpWidget,
                        Visibility::Hidden,
                        
                        Node {
                            width: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        }
                    )).id();
                    cmd.entity(e).add_child(h);
                }

                if *us == UiSlot::BottomLeft {
                    let ch = cmd.spawn((
                        Node {
                            margin: UiRect::all(Val::Px(10.)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        children![
                            Text::new("Alt + H: Help"),
                        ],
                        
                        
                    )).id();
                    cmd.entity(e).add_child(ch);
                }
            }
            h
        }
    };


    let SetHelpData{title, keys, hint} = tr.event();
    let info = cmd.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.)),
            margin: UiRect::bottom(Val::Px(10.)),
            ..default()
        },
        BorderRadius::all(Val::Px(15.)),
        children![
            (
                Text::new(*title),
                TextColor(css::YELLOW_GREEN.into())
            ),
            Text::new(*keys),
            Text::new(*hint),
        ],
        BackgroundColor(Color::BLACK.with_alpha(0.5)),
    )).id();

    cmd.entity(hwe).add_child(info);

}

// ---

fn toggle_help (
    v_q: Single<&mut Visibility, With<HelpWidget>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyH) && (keys.pressed(KeyCode::AltLeft) || keys.pressed(KeyCode::AltRight)) {
        v_q.into_inner().toggle_visible_hidden();
    }
} 
