use bevy::{
    color::palettes::css, prelude::*
};

use crate::{
    camera::Cam,
    shared::{Player}, ui,
    messages::{HideTime, set_text}
};
    

pub struct MonologuePlugin;
impl Plugin for MonologuePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup.after(ui::startup))
        .add_systems(Update, follow.run_if(any_with_component::<HideTime>))
        .add_observer(set_text::<MonologueCont>)
        ;
    }
}

// ---

#[derive(Component)]
pub struct MonologueCont;

// ---

fn startup(
    mut cmd: Commands
) {
    cmd.spawn((
        MonologueCont,
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.)),
            padding: UiRect::all(Val::Px(20.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
        BackgroundColor(css::BLACK.with_alpha(0.9).into()),
        BorderRadius::all(Val::Px(15.)),
        BorderColor::all(css::WHITE),
    ));

}

// ---

#[allow(dead_code)]
fn follow(
    player_q: Single<&Transform, With<Player>>,   
    ballon_q: Single<(&mut Node, &ComputedNode), With<MonologueCont>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<Cam>>,
) {
    let (camera, camera_transform) = camera_query.into_inner();
    let point = player_q.into_inner().translation + Vec3::Y * 3.;

    if let Ok(coords) = camera.world_to_viewport(camera_transform, point) {
        let (mut  b_node, b_cnode) = ballon_q.into_inner();
        b_node.top = Val::Px(coords.y - b_cnode.size.y / 2.) ;
        b_node.left = Val::Px(coords.x - b_cnode.size.x / 2.);
    }
}
