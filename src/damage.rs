use bevy::prelude::*;
use avian3d::prelude::*;

use crate::shared::{Damage, DamageCallback, DamageDeal, DamageDealed, HealthMax, Targetable, Target};

pub struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_observer(on_collision)
        .add_observer(clear_targets)
        ;
    }
}

fn on_collision(
    tr: On<CollisionStart>,
    mut damageable_q: Query<(&mut Damage, &HealthMax, Option<&DamageCallback>)>,
    dd_q: Query<&DamageDeal>,
    mut cmd: Commands
) {
    let Some(other)  = tr.body2 else {return;};
    let Some(me) = tr.body1 else {return;};
    
    let Ok((mut damage, health_max, c_o)) =  damageable_q.get_mut(other) else {
        return;
    };
    let Ok(dd) = dd_q.get(me) else {
        return;
    };

    damage.0 += dd.0;
    if c_o.is_some() {
        cmd.trigger(DamageDealed{entity: other});
    } else {
        if health_max.0 - damage.0 <= 0. {
            cmd.entity(other).try_despawn();
        } 

    }
}

// --

fn clear_targets(
    tr: On<Remove, Targetable>,
    q: Query<(Entity, &Target)>,
    mut cmd: Commands
) {
    let e = tr.entity;
    for (et, t) in &q {
        if t.0 == e {
            cmd.entity(et).remove::<Target>();
        } 
    }
}  