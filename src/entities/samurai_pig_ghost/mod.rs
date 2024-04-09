use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::*;

use crate::MyStates;

use super::player::components::Player;

pub mod components;
mod systems;

pub struct SamuraiPigGhostPlugin;

impl Plugin for SamuraiPigGhostPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<SamuraiPigGhostBundle>("Enemy")
            .add_systems(Update, SamuraiPigGhost::walk.after(Player::walk).run_if(in_state(MyStates::Running)));
                
    }
}