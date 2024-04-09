use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Clone, Eq, PartialEq, Debug, Default)]
pub struct SamuraiPigGhost;

#[derive(Bundle, LdtkEntity, Clone, Default)]
pub struct SamuraiPigGhostBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    samurai_pig_ghost: SamuraiPigGhost
}