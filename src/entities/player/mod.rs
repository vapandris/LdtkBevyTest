use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::MyStates;
use components::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, (
                Player::animate,
                Player::walk,
            ).run_if(in_state(MyStates::Running)));

    }
}