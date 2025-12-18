use std::{
    time::Duration,
    marker::PhantomData
};

use bevy::{
    platform::collections::HashMap, 
    prelude::*
};
  

pub struct MessagesPlugin;
impl Plugin for MessagesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (hide_container).run_if(any_with_component::<HideTime>))
        ;
    }
}

// ---

#[derive(Event)]
pub struct MessagesAddLine<T>{
    pub text: &'static str, 
    time: u64,
    font_size: f32,
    color: Option<Color>,
     _marker: PhantomData<T>
}

impl <T>MessagesAddLine<T>  {
    pub fn new(text: &'static str) -> Self {
        Self { text , time: 10, font_size: 22.0, color: None, _marker: PhantomData::<T>}
    }
     #[allow(dead_code)]
    pub fn with_time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }

    #[allow(dead_code)]
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }    
    #[allow(dead_code)]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }    
}


#[derive(Component)]
struct MessageLine;


#[derive(Component)]
struct HideTime(Timer);

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
                // width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::bottom(Val::Px(5.)),
                 ..default()
            },
            // BackgroundColor(Color::BLACK.with_alpha(0.5)),
            MessageLine,
            children![(
                TextColor(
                    if let Some(color) = tr.event().color {
                        color
                    } else {
                        if *count % 2 == 0 {Color::linear_rgb(0.4, 1.1, 0.0)} else {Color::linear_rgb(1.1, 1.1, 0.1)}
                    } 
                ),
                Text::new(tr.event().text),
                TextFont{font_size: tr.event().font_size,
                     ..default()
                }
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
    mut line_q: Query<(Entity, &ChildOf, &mut HideTime, &Children), With<HideTime>>,
    mut cont_q: Query<&mut Visibility>,
    mut cmd: Commands,
    time: Res<Time>,
    mut text_q: Query<&mut TextColor>

) {
   
    if line_q.is_empty() {
        return;
    }

    let mut lhm: HashMap<Entity, (usize, usize)> = HashMap::new();

    for  (ble, parent, mut ht, ch) in &mut line_q  {
        ht.0.tick(time.delta());

        if ht.0.remaining_secs() < 1. {
            let Ok(mut text_color) = text_q.get_mut(ch[0])  else {
                continue;
            };
            let alpha = text_color.alpha() - 0.1;
            text_color.set_alpha(alpha);

        }


        let ee = lhm.entry(parent.0).or_insert((0, 0));
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

