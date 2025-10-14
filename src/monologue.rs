use std::time::Duration;

use bevy::{
    color::palettes::css, prelude::*
};

use crate::{
    camera::Cam,
    shared::{Player, SetMonologueText}
};
    

pub struct MonologuePlugin;
impl Plugin for MonologuePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        // .add_systems(Update, follow.run_if(any_match_filter::<ActiveBallon<Ballon, HideTime>>))
        .add_systems(Update, (follow,hide_ballon).run_if(any_with_component::<HideTime>))
        // .add_systems(Update, hide_ballon.run_if(any_with_component::<HideTime>))
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
pub struct Ballon;

#[derive(Component)]
pub struct HideTime(Timer);

// ---

fn startup(
    mut cmd: Commands
) {
    cmd.spawn((
        Ballon,
        Node {
            position_type: PositionType::Absolute,
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
        children![
            (
                Text::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
                TextColor(Color::WHITE)
            )
        ],

    ));



    // let Ok(content) = fs::read_to_string("monologue/script.txt") else {
    //     return;
    // };


}

// ---

fn set_text(
    tr: On<SetMonologueText>,
    ballon_q: Single<(Entity, &Children, &mut Visibility), With<Ballon>>,
    mut text_q: Query<&mut Text>,
    mut cmd: Commands
) {
    let (e, ch, mut vis) = ballon_q.into_inner();
    let Ok(mut text) = text_q.get_mut(ch[0]) else {
        return;
    };
    *vis = Visibility::Visible;

    text.0 = tr.event().text.to_string();
    cmd.entity(e).insert(HideTime(Timer::new(Duration::from_secs(tr.event().time), TimerMode::Once)));
}

// ---

fn hide_ballon (
    ballon_q: Single<(Entity, &mut Visibility, &mut HideTime), With<Ballon>>,
    mut cmd: Commands,
    time: Res<Time>
) {
    let (e, mut v, mut ht) = ballon_q.into_inner();
    ht.0.tick(time.delta());
    if ht.0.is_finished() {
        *v = Visibility::Hidden;
        cmd.entity(e).remove::<HideTime>();
    }
}

// ---

#[allow(dead_code)]
fn follow(
    player_q: Single<&Transform, With<Player>>,   
    ballon_q: Single<&mut Node, With<Ballon>>,
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
