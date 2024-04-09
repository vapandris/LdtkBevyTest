use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::colliders::component::*;

#[derive(AssetCollection, Resource)]
pub struct PlayerAnimations {
    // ====== IDLE ===== \\
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 23., columns = 10, rows = 1))]
    #[asset(path = "creatures/assasin/assasin-idle-down.png")]
    #[asset(image(sampler = nearest))]
    pub idle_down: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 23., columns = 10, rows = 1))]
    #[asset(path = "creatures/assasin/assasin-idle-up.png")]
    #[asset(image(sampler = nearest))]
    pub idle_up: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 23., columns = 10, rows = 1))]
    #[asset(path = "creatures/assasin/assasin-idle-side.png")]
    #[asset(image(sampler = nearest))]
    pub idle_right: Handle<TextureAtlas>,

    // ====== WALK ===== \\
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 23., columns = 10, rows = 1))]
    #[asset(path = "creatures/assasin/assasin-walk-down.png")]
    #[asset(image(sampler = nearest))]
    pub walk_down: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 23., columns = 10, rows = 1))]
    #[asset(path = "creatures/assasin/assasin-walk-up.png")]
    #[asset(image(sampler = nearest))]
    pub walk_up: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 23., columns = 10, rows = 1))]
    #[asset(path = "creatures/assasin/assasin-walk-side.png")]
    #[asset(image(sampler = nearest))]
    pub walk_right: Handle<TextureAtlas>,
}

#[derive(Component, Clone, Eq, PartialEq, Debug, Default)]
pub struct Player {
    pub state: PlayerState,
    pub dir: Direction,
}


#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
}

// Both x, y: [-1, 1]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Direction {
    pub x: i8,
    pub y: i8,
}

impl Default for Direction {
    fn default() -> Self {
        return Direction { x: 0, y: -1 };
    }
}

#[derive(Bundle, LdtkEntity, Clone, Default)]
pub struct PlayerBundle {
    #[with(PlayerAnimationTimer::unresolved)]
    pub timer: PlayerAnimationTimer,
    pub sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,

    pub player: Player,

    #[worldly]
    pub worldly: Worldly,
}

#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct PlayerAnimationTimer {
    pub timer: Timer
}

impl PlayerAnimationTimer {
    fn unresolved(_: &EntityInstance) -> PlayerAnimationTimer {
        PlayerAnimationTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating)
        }
    }
}