use bevy::prelude::*;
use crate::shared::StageIndex;
pub struct FeaturePlugin;
impl Plugin for FeaturePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, stage_index_changed.run_if(resource_changed::<StageIndex>))
        ;
    }
}

// ---

fn stage_index_changed() {

}