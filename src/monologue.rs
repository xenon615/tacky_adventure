
use std::{collections::HashMap, time::Duration};
use bevy::{
    color::palettes::css, prelude::*, time::common_conditions::on_timer
};
use bevy_gltf_animator_helper::AniData;

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
        .add_systems(
            Update, 
            read_mono_lines
            .run_if(
                not(resource_equals(MonoIndex(None))).and(
                    on_timer(Duration::from_secs(5)).or(run_once)
                )
            )
        )
        .add_observer(set_text::<MonologueCont>)
        ;
    }
}

// ---

#[derive(Component)]
pub struct MonologueCont;

#[derive(Resource, Default, PartialEq)]
pub struct MonoIndex(Option<usize>);


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
                "Complete bad taste, in short.",
                "I guess I should go to that shimmering thing .."
            ],
            vec![
                "Holy shit!",
                "Goodbye, colorless world",
                "Hello world of eye-bleeding colors and annoying flickering",
                "I repeat, complete bad taste",
                "Although what previously looked like dumplings...",
                "Whatever..",
                " ",
                "Probably need to get to that flickering thing again that looks like crazy plasma",
                "You can't just approach this thing, but something tells me it can be fixed.",
            ],
            vec![
                "Now it's easier for me to understand where to go.",
                "This is a really useful feature."
            ],
            vec![
                "An elevator is not bad, I will build less."
            ]

        ])
    }
}

// ---

fn startup(
    mut cmd: Commands
) {

    cmd.init_resource::<MonoIndex>();
    cmd.spawn((
        MonologueCont,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(90.),
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(2.)),
            padding: UiRect::all(Val::Px(20.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
        // BackgroundColor(css::BLACK.with_alpha(0.9).into()),
        BorderRadius::all(Val::Px(15.)),
        // BorderColor::all(css::WHITE),
    ));

}

// ---

#[allow(dead_code)]
fn follow(
    player_q: Single<(&Transform, &AniData), With<Player>>,   
    ballon_q: Single<(&mut Node, &ComputedNode), With<MonologueCont>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<Cam>>,
) {
    let (camera, camera_transform) = camera_query.into_inner();
    let (player_t, ad) = player_q.into_inner();

    let point = player_t.translation + Vec3::Y * if ad.animation_index != 7 {2.5} else {1.8};

    if let Ok(coords) = camera.world_to_viewport(camera_transform, point) {
        let (mut  b_node, b_cnode) = ballon_q.into_inner();
        b_node.top = Val::Px(coords.y - b_cnode.size.y / 2.) ;
        b_node.left = Val::Px(coords.x - b_cnode.size.x / 2.);
    }
}

// --

fn opt_index_changed(
    opt_index: Res<OptionIndex>,
    mut mono_index: ResMut<MonoIndex>
) {
    mono_index.0 = Some(opt_index.0);
}

// ---

fn read_mono_lines(
    mut cmd: Commands,
    mut mono_lines: ResMut<MonoLines>,
    mut mono_index: ResMut<MonoIndex>
) {

    let Some(mi) = mono_index.0 else {
        return;
    };

    if let Some(section) = mono_lines.0.get_mut(mi)  {
        if !section.is_empty() {
            let s = section.remove(0);
            cmd.trigger(MessagesAddLine::<MonologueCont>::new(s).with_time(4));            
        } else {
            mono_index.0 = None;    
        }
    } else {
        mono_index.0 = None;
    }
}