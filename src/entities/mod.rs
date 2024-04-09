use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod player;
pub mod samurai_pig_ghost;

use player::PlayerPlugin;
use samurai_pig_ghost::SamuraiPigGhostPlugin;
use crate::MyStates;
use player::components::PlayerAnimations;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PlayerPlugin,
                SamuraiPigGhostPlugin
            ))

            // call load state here so it is only called once
            .add_loading_state(
                LoadingState::new(MyStates::AssetLoading)
                    .continue_to_state(MyStates::Running)
                    .load_collection::<PlayerAnimations>()
            );
    }
}