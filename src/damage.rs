use bevy::prelude::*;
use avian3d::prelude::*;

use crate::shared::{Damage, DamageCallback, DamageDeal, DamageDealed, HealthMax};

pub struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_observer(on_collision)
        ;
    }
}

fn on_collision(
    tr: Trigger<OnCollisionStart>,
    mut damageable_q: Query<(&mut Damage, &HealthMax, Option<&DamageCallback>)>,
    dd_q: Query<&DamageDeal>,
    // name_q: Query<&Name>,
    mut cmd: Commands
) {
    let Some(other)  = tr.body else {return;};

    let me = tr.target();
    // let def = Name::new("Unknown");
    // let me_name = name_q.get(me).unwrap_or(&def);
    // let other_name = name_q.get(other).unwrap_or(&def);
    
    let Ok((mut damage, health_max, c_o)) =  damageable_q.get_mut(other) else {
        return;
    };
    let Ok(dd) = dd_q.get(me) else {
        return;
    };

    damage.0 += dd.0;
    if c_o.is_some() {
        cmd.trigger_targets(DamageDealed, other);
    } else {
        if health_max.0 - damage.0 <= 0. {
            cmd.entity(other).despawn();
        } 

    }

    // println!("{:?} collided with {:?} current damage {}" , me_name, other_name, damage.0);
        
}


