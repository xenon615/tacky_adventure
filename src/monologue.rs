
use std::collections::HashMap;
use bevy::{
    color::palettes::css, prelude::*
};

use crate::{
    camera::Cam, messages::{HideTime, set_text}, shared::{MessagesAddLine, OptionIndex, Player}, ui
};
    

pub struct MonologuePlugin;
impl Plugin for MonologuePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MonoLines>()
        .add_systems(Startup, startup.after(ui::startup))
        .add_systems(Update, follow.run_if(any_with_component::<HideTime>))
        .add_systems(Update, opt_index_changed.run_if(resource_changed::<OptionIndex>))
        .add_observer(set_text::<MonologueCont>)
        ;
    }
}

// ---

#[derive(Component)]
pub struct MonologueCont;

// #[derive(Resource)]
// pub struct MonoLines(HashMap<&'static str, Vec<&'static str>>);

// impl FromWorld for MonoLines {
//     fn from_world(world: &mut World) -> Self {
//         Self(HashMap::from(
//             [
//                 ("intro", vec!["Hi", "Hello!"])
//             ]
//         ))
//     }
// }

#[derive(Resource)]
pub struct MonoLines(Vec<Vec<&'static str>>);

impl FromWorld for MonoLines {
    fn from_world(_world: &mut World) -> Self {
        Self(vec![
            vec![
                "What a strange place?",
                "I wonder how I ended up here.",
                "Probably again the fault of this idiot who thinks he is able to create realities.",
                "what the fuck is his name?",
                "God, demiurge, Sir Max?",
                "Never mind, let's take a look around",
                "A path leading to a strange, shimmering thing and overgrown flying dumplings.",
                "Everything is pale, I'm the only one here, blue as an drunkard's nose on a winter morning.",
                "Complete bad taste, in short",
                "I guess I should go to that shimmering thing .."
            ]
        ])
    }
}

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
            justify_content: JustifyContent::Start,
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


// --

fn opt_index_changed(
    opt_index: Res<OptionIndex>,
    mut cmd: Commands,
    lines: Res<MonoLines>
) {
    
    if let Some(section) =  lines.0.get(opt_index.0) {
        for s in section {
            cmd.trigger(MessagesAddLine::<MonologueCont>::new(s));        
        }
    }

} 