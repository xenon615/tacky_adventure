use std::time::Duration;

use bevy::{
    platform::collections::HashMap, 
    prelude::*
};

use crate::shared::MessagesAddLine;

    

pub struct MessagesPlugin;
impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (hide_container).run_if(any_with_component::<HideTime>))
        ;
    }
}

// ---

#[derive(Component)]
pub struct MessageLine;


#[derive(Component)]
pub struct HideTime(Timer);

// ---

pub fn set_text<T: Component>(
    tr: On<MessagesAddLine<T>>,
    container_q: Single<(Entity,  &mut Visibility), With<T>>,
    mut cmd: Commands,
    mut count: Local<u32>
) {
    let (e, mut vis) = container_q.into_inner();

    let new_line = cmd.spawn(
        (
            Node {
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::bottom(Val::Px(5.)),
                 ..default()
            },
            MessageLine,
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

fn hide_container (
    mut line_q: Query<(Entity, &ChildOf, &mut HideTime), With<HideTime>>,
    mut cont_q: Query<&mut Visibility>,
    mut cmd: Commands,
    time: Res<Time>
) {
   
    if line_q.is_empty() {
        return;
    }

    let mut lhm: HashMap<Entity, (usize, usize)> = HashMap::new();

    for  (ble, parent, mut ht) in &mut line_q  {
        ht.0.tick(time.delta());
        let ee = lhm.entry(parent.0).or_insert((0,0));
        ee.0 += 1;
        if ht.0.is_finished() {
            cmd.entity(ble).despawn();
            ee.1 += 1;
        } 
    }
    
    lhm.iter().for_each(| e | if e.1.0 == e.1.1 {
        let mut v = cont_q.get_mut(*e.0).unwrap() ;
        *v = Visibility::Hidden;
    });

}

// ---

