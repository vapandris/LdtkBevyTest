use bevy::prelude::*;
use crate::entities::player::components::Player;

use super::SamuraiPigGhost;

impl SamuraiPigGhost {
    pub const SPEED: f32 = 35.0;

    pub fn walk(
        mut samurai_pig_query: Query<&mut Transform, (With<SamuraiPigGhost>, Without<Player>)>,
        player_query: Query<&Transform, With<Player>>,
        time: Res<Time>
    ) {
        if let Ok(player_transform) = player_query.get_single() {
            for mut sam_pig_transform in samurai_pig_query.iter_mut() {
                let distance = player_transform.translation - sam_pig_transform.translation;

                sam_pig_transform.translation += distance.normalize() * SamuraiPigGhost::SPEED * time.delta_seconds();
            }
        } 
    }
}