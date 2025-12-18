use bevy::prelude::*;
use crate::{
    info::InfoCont, messages::MessagesAddLine, stage::{StageIndex, stage_index_changed}
};
pub struct EndPlugin;
impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_systems(Update, stage_index_changed::<6, EnabledEnd>.run_if(resource_changed::<StageIndex>))
          .add_systems(Update, the_end.run_if(resource_added::<EnabledEnd>))
        ;
    }
}

#[derive(Resource, Default)]
pub struct EnabledEnd;


fn the_end(
    mut cmd: Commands
) {
    cmd.trigger(MessagesAddLine::<InfoCont>::new("To be continued ...").with_font_size(48.).with_time(20));
}