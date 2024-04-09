
use bevy::prelude::*;
use crate::entities::player::components::*;
use crate::entities::player::components::Direction;



impl Player {

    pub const SPEED: f32 = 55.0;
    
    pub fn animate(
        time: Res<Time>,
        asset: Res<PlayerAnimations>,
        mut sprites_to_animate: Query<(
            &mut PlayerAnimationTimer,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
            &Player
        )>
    ) {
        for (
            mut timer,
            mut sprite,
            mut atlas_handle,
            player
        ) in &mut sprites_to_animate {

            timer.timer.tick(time.delta());

            if timer.timer.finished() {
                sprite.index = (sprite.index + 1) % 10;
                
                match &player.state {
                    PlayerState::Idle => {
                        match player.dir {
                            Direction { x: _, y: -1 } => {
                                *atlas_handle = asset.idle_down.clone();
                                sprite.flip_x = false;
                            }
                            Direction { x: _, y: 1 } => {
                                *atlas_handle = asset.idle_up.clone();
                                sprite.flip_x = false;
                            }
                            Direction { x: -1, y: _ } => {
                                *atlas_handle = asset.idle_right.clone();
                                sprite.flip_x = true;
                            }
                            Direction { x: 1, y: _ } => {
                                *atlas_handle = asset.idle_right.clone();
                                sprite.flip_x = false;
                            }
                            _ => {}
                        }
                    }
                    PlayerState::Walk => {
                        match player.dir {
                            Direction { x: _, y: -1 } => {
                                *atlas_handle = asset.walk_down.clone();
                                sprite.flip_x = false;
                            }
                            Direction { x: _, y: 1 } => {
                                *atlas_handle = asset.walk_up.clone();
                                sprite.flip_x = false;
                            }
                            Direction { x: -1, y: _ } => {
                                *atlas_handle = asset.walk_right.clone();
                                sprite.flip_x = true;
                            }
                            Direction { x: 1, y: _ } => {
                                *atlas_handle = asset.walk_right.clone();
                                sprite.flip_x = false;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn walk(
        mut player_query: Query<(&mut Transform, &mut Player)>,
        keyboard_input: Res<Input<KeyCode>>,
        time: Res<Time>
    ) {
        if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
            let mut moved = false;
            let mut direction: Vec3 = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::A) {
                moved = true;
                direction.x = -1.;
            }
            if keyboard_input.pressed(KeyCode::D) {
                moved = true;
                direction.x = 1.;
            }

            if keyboard_input.pressed(KeyCode::W) {
                moved = true;
                direction.y = 1.;
            }
            if keyboard_input.pressed(KeyCode::S) {
                moved = true;
                direction.y = -1.;
            }

            if moved {
                player.state = PlayerState::Walk;
                player.dir.x = direction.x as i8;
                player.dir.y = direction.y as i8;
                transform.translation += direction.normalize() * Player::SPEED * time.delta_seconds();
            } else {
                player.state = PlayerState::Idle;
            }

        }
    }
}