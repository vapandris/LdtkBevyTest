use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub mod entities;
pub mod colliders;
pub mod intgrids;

use entities::EntityPlugin;
use entities::player::components::Player;
use intgrids::wall::IntGridPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum MyStates {
    #[default]
    AssetLoading,
    Running
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))

        .add_plugins((
            EntityPlugin,
            IntGridPlugin
        ))
        
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))

        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<MyStates>()

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(75.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .add_systems(Update, (
            camera_fit_inside_current_level,
        ).run_if(in_state(MyStates::Running)))
        .add_systems(OnEnter(MyStates::Running), setup)

        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        ..Default::default()
    });
}

// ==== CAMERA =====
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let (mut camera_orthographic_projection, mut camera_transform) =
        camera_query.get_single_mut()
        .expect("Exactly one camera should exist at all times!");

    let ldtk_project_handle =
        ldtk_projects.get_single()
        .expect("Exactly one LDTK project should be loadet at all times!");

    if let Some(ldtk_project) =
        ldtk_project_assets.get(ldtk_project_handle) {

        // Iterate through each level and check if they are the currently selected one:
        for (level_transform, level_iid) in &level_query {
            let level =
                ldtk_project.get_raw_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDTK project!");

            if level_selection.is_match(&LevelIndices::default(), level) {
                const BOTTOM_LEFT: Vec2 = Vec2::ZERO;
                camera_orthographic_projection.viewport_origin = BOTTOM_LEFT;
                camera_orthographic_projection.scaling_mode = ScalingMode::Fixed { width: level.px_wid as f32, height: level.px_hei as f32 };
                camera_transform.translation = level_transform.translation.clone();
            }
        }
    }
}