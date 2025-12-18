
use std::time::Duration;
use bevy::{
    prelude::*, time::common_conditions::on_timer
};

use crate::{
    camera::Cam, 
    messages::set_text, 
    ui,
    player::Player,
    messages::MessagesAddLine
};
    
pub struct MonologuePlugin;
impl Plugin for MonologuePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MonoLines>()
        .add_systems(Startup, startup.after(ui::startup))
        // .add_systems(Update, follow.run_if(any_with_component::<HideTime>))
        .add_systems(Update, send_line.run_if( resource_exists_and_equals(MonoActive(true)).and( on_timer(Duration::from_secs(5)).or(run_once))))
        .add_systems(Update, lines_added.run_if(resource_exists_and_changed::<MonoLines>.and(resource_exists::<MonoActive>)))
        .add_observer(set_text::<MonologueCont>)
        ;
    }
}

// ---

#[derive(Component)]
pub struct MonologueCont;


#[derive(Resource, Default)]
pub struct MonoLines(pub Vec<&'static str>);

#[derive(Resource, PartialEq, Default)]
pub struct MonoActive(bool);

// ---

fn startup(
    mut cmd: Commands
) {

    cmd.init_resource::<MonoActive>();
    cmd.spawn((
        MonologueCont,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(45.0),
            width: Val::Percent(90.),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.)),
            padding: UiRect::all(Val::Px(20.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
        BorderRadius::all(Val::Px(15.)),
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
    let player_t = player_q.into_inner();

    let point = player_t.translation + 0.5 * Vec3::NEG_Y;

    if let Ok(coords) = camera.world_to_viewport(camera_transform, point) {
        let (mut  b_node, b_cnode) = ballon_q.into_inner();
        b_node.top = Val::Px(coords.y - b_cnode.size.y / 2.) ;
        b_node.left = Val::Px(coords.x - b_cnode.size.x / 2.);
    }
}

// ---

fn send_line(
    mut cmd: Commands,
    mut mono_lines: ResMut<MonoLines>,
    mut mono_active: ResMut<MonoActive>,
) {
    if let Some(line) = mono_lines.0.get(0) {
        cmd.trigger(MessagesAddLine::<MonologueCont>::new(line).with_time(5));
        mono_lines.0.remove(0);
    } else {
        mono_active.0 = false;
    };
}

// ---

fn lines_added(
    mut mono_active: ResMut<MonoActive>,
    mono_lines: Res<MonoLines>
) {
    if  !mono_active.0  && !mono_lines.0.is_empty() {
        mono_active.0 = true;
    }
}