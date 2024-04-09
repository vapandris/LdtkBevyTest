use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::MyStates;
use components::*;

pub mod components;
mod systems;

pub struct IntGridPlugin;

impl Plugin for IntGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_int_cell::<WallBundle>(1)

            .add_systems(Update, (
                Wall::spawn_collision
            ).run_if(in_state(MyStates::Running)));
    }
}