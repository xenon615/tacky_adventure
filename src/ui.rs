use bevy:: {
    color::palettes::css, prelude::*
};

use crate::shared::{BuildAction, CastBuild, GameStage};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(Startup, startup)
        .add_systems(OnEnter(GameStage::Two), startup)
        .add_systems(Update, keypress
            .run_if(resource_changed::<ButtonInput<KeyCode>>)
            .run_if(in_state(GameStage::Two))
        )
        .add_systems(Update, interact_buttons)
        ;
    }
}

// ---

#[derive(Component)]
struct MenuRoot;

// ---

fn keypress(
    v_q: Single<&mut Visibility, With<MenuRoot>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    if keys.just_pressed(KeyCode::KeyM) && keys.pressed(KeyCode::AltLeft) {
        v_q.into_inner().toggle_visible_hidden();
    }
} 

// ---

fn startup(
    mut cmd: Commands,
) {
    cmd.spawn((
        Node {
           width: Val::Vw(100.),
           height: Val::Vh(100.),
            ..default()
        },
        MenuRoot,
        Visibility::Visible,
    )).with_children(| root | {
        root.spawn((
            BackgroundColor(Color::hsla(0., 0.1, 0.1, 0.6).into()),
            Node {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                ..default()
            },
        ))
        .with_children(| cont | {
            let menu = vec![
                (BuildAction::Up, "Up".to_string(), ),
                (BuildAction::Forward, "Forward".to_string()),
                (BuildAction::Down, "Down".to_string()),
                (BuildAction::Delete, "Delete".to_string()),

            ];
            for (k, v) in menu.iter() {
                cont.spawn(button(v.as_str())).insert(*k);
            }
        })
        ;
    })
    ;
}

// ---

fn interact_buttons(
    mut interaction_query: Query<(&Interaction, &Children, &BuildAction), (Changed<Interaction>, With<Button>)>,
    mut text_q: Query<&mut TextColor>,
    mut cmd: Commands
) {
    for (interaction, cc, ba) in &mut interaction_query {
        let Ok(mut color) = text_q.get_mut(cc[0]) else {
            continue;
        };
        match *interaction {
            Interaction::Pressed => {
               cmd.trigger(CastBuild(*ba));
            }
            Interaction::Hovered => {
                color.0 = css::YELLOW.into();
            }
            Interaction::None => {
                color.0 = css::BEIGE.into();
            }
        }
    }
}

// ---

fn button(text: &str) ->impl Bundle + use<> {
    (
        Button,
        Node {
            padding: UiRect::all(Val::Px(5.0)),
            width: Val::Px(150.),
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new(text.to_string()),
                TextColor(css::BEIGE.into()),
                TextFont {
                    font_size: 20.,
                    ..default()
                }                        
            )
        ]
    )
}
