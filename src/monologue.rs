use std::time::Duration;

use bevy::{
    color::palettes::css, prelude::*
};

use crate::{
    camera::Cam,
    shared::{MonologueAddLine, Player}, ui
};
    

pub struct MonologuePlugin;
impl Plugin for MonologuePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup.after(ui::startup))
        .add_systems(Update, (
            follow, 
            hide_ballon
        ).run_if(any_with_component::<HideTime>))
        .add_observer(set_text)
        ;
    }
}

// ---

// #[derive(QueryFilter)]
// struct ActiveBallon<T1: Component, T2: Component > {
//     _gt: (With<T1>, With<T2>)
// }

#[derive(Component)]
pub struct Balloon;

#[derive(Component)]
pub struct BalloonLine;


#[derive(Component)]
pub struct HideTime(Timer);

// ---

fn startup(
    mut cmd: Commands
) {
    cmd.spawn((
        Balloon,
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Px(200.),
            left: Val::Px(200.),
            width: Val::Percent(25.),
            border: UiRect::all(Val::Px(5.)),
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

fn set_text(
    tr: On<MonologueAddLine>,
    ballon_q: Single<(Entity,  &mut Visibility), With<Balloon>>,
    mut cmd: Commands,
    mut count: Local<u32>
) {
    let (e, mut vis) = ballon_q.into_inner();

    let new_line = cmd.spawn(
        (
            Node {
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::bottom(Val::Px(5.)),
                 ..default()
            },
            BalloonLine,
            children![(
                TextColor(if *count % 2 == 0 {Color::linear_rgb(0.4, 1.1, 0.0)} else {Color::linear_rgb(1.1, 1.1, 0.1)}),
                Text::new(tr.event().text),
            )]
        )    
    ).id()
    ;
    cmd.entity(e).add_child(new_line); 

    cmd.entity(new_line).insert(HideTime(Timer::new(Duration::from_secs(tr.event().time), TimerMode::Once)));
    *vis = Visibility::Visible;
    *count += 1;
}


// ---

fn hide_ballon (
    mut line_q: Query<(Entity, &mut HideTime), With<BalloonLine>>,
    ballon_q: Single<&mut Visibility, With<Balloon>>,
    mut cmd: Commands,
    time: Res<Time>
) {
    let mut v = ballon_q.into_inner();
    let count = line_q.count();
    let mut despawned = 0;
    for  (ble, mut ht) in &mut line_q  {
        ht.0.tick(time.delta());
        if ht.0.is_finished() {
            cmd.entity(ble).despawn();
            despawned += 1;
        }

    }
    if despawned == count {
        *v  = Visibility::Hidden;
    }
}

// ---

#[allow(dead_code)]
fn follow(
    player_q: Single<&Transform, With<Player>>,   
    ballon_q: Single<&mut Node, With<Balloon>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<Cam>>,
) {
    let (camera, camera_transform) = camera_query.into_inner();
    let point = player_q.into_inner().translation + Vec3::Y * 3.;

    if let Ok(coords) = camera.world_to_viewport(camera_transform, point) {
        let mut b_node = ballon_q.into_inner();
        b_node.top = Val::Px(coords.y) ;
        // b_node.left = Val::Px(coords.x);
    }
}
