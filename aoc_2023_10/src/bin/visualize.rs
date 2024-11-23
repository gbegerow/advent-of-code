use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use camera::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // 0: | is a vertical pipe connecting north and south.
    // 1: - is a horizontal pipe connecting east and west.
    // 2: L is a 90-degree bend connecting north and east.
    // 3: J is a 90-degree bend connecting north and west.
    // 4: 7 is a 90-degree bend connecting south and west.
    // 5: F is a 90-degree bend connecting south and east.
    // 6: . is ground; there is no pipe in this tile.
    // 7: S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    // TODO: translate ../input.txt into tilemap

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.   qq
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("AOC 2023 10"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, camera::movement)
        .run();
}

mod camera {
    use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

    // A simple camera system for moving and zooming the camera.
    #[allow(dead_code)]
    pub fn movement(
        time: Res<Time>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    ) {
        for (mut transform, mut ortho) in query.iter_mut() {
            let mut direction = Vec3::ZERO;

            if keyboard_input.pressed(KeyCode::KeyA) {
                direction -= Vec3::new(1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyD) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyW) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyS) {
                direction -= Vec3::new(0.0, 1.0, 0.0);
            }

            if keyboard_input.pressed(KeyCode::KeyZ) || keyboard_input.pressed(KeyCode::KeyY) {
                ortho.scale += 0.1;
            }

            if keyboard_input.pressed(KeyCode::KeyX) {
                ortho.scale -= 0.1;
            }

            if ortho.scale < 0.5 {
                ortho.scale = 0.5;
            }

            let z = transform.translation.z;
            transform.translation += time.delta_seconds() * direction * 500.;
            // Important! We need to restore the Z values when moving the camera around.
            // Bevy has a specific camera setup and this can mess with how our layers are shown.
            transform.translation.z = z;
        }
    }
}
