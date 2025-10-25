use bevy::prelude::*;


pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        ;
    }
}

// ---

#[derive(Component)]
struct UiRoot;

#[derive(Component, PartialEq)]
pub enum UiSlot {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
    Middle
}

// ---

pub fn startup(
    mut cmd: Commands,
) {
    cmd.spawn((
        Node {
           width: Val::Vw(100.),
           height: Val::Vh(100.),
           flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        UiRoot,
        Name::new("UIROOT")
    ))
    .with_children(| root | {
        root.spawn((
            // BackgroundColor(css::BROWN.into()),
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.),
                height: Val::Px(50.),
                ..default()
            },
            children![
                (
                    Node {width: Val::Percent(50.), justify_content:JustifyContent::Center,  ..default()},
                    UiSlot::TopLeft
                ),
                (
                    Node {width: Val::Percent(50.), justify_content:JustifyContent::Center, ..default()},
                    UiSlot::TopRight,
                    Name::new("TopRight")
                )                
            ]
        ));

        root.spawn((
            // BackgroundColor(Color::hsl(10., 1.0, 0.5).into()),
            Node {
                // align_items: AlignItems::Center,
                flex_grow: 1.0,
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            UiSlot::Middle,
            Name::new("MIDDLE")
        ));

        root.spawn((
            // BackgroundColor(css::BROWN.into()),
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Px(50.),
                ..default()
            },
            children![
                (
                    Node {width: Val::Percent(50.), justify_content:JustifyContent::Center, ..default()},
                    UiSlot::BottomLeft
                ),
                (
                    Node {width: Val::Percent(50.), justify_content:JustifyContent::Center, ..default()},
                    UiSlot::BottomRight

                )                
            ]
        ));        
    })
    ;
}


