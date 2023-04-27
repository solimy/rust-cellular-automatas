extern crate termion;
extern crate clap;


mod braille;
mod canvas;
mod cellular_automata;
mod settings;


use std::time::Duration;

use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
    sprite::MaterialMesh2dBundle,
    window::WindowResolution,
};

use rayon::prelude::*;

use cellular_automata::World;

#[derive(Resource, Default)]
struct Config(settings::CommandLineProvidedSettings);


#[derive(Resource)]
struct Dimensions {
    width: u16,
    height: u16,
}

#[derive(Resource)]
struct CellsMaterials {
    dead: Handle<ColorMaterial>,
    alive: Handle<ColorMaterial>,
}

#[derive(Component)]
struct WorldIndex {
    i: usize,
}

#[derive(Resource)]
struct WorldState {
    world: World,
}


fn main() {
    let config = Config::default();
    let tbt = config.0.tbt;
    let width = (config.0.width * 5) as f32;
    let height = (config.0.height * 5) as f32;
    let dimensions = Dimensions {
        width: width as u16,
        height: height as u16,
    };

    App::new()
        .insert_resource(dimensions)
        .insert_resource(config)
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(width, height)
                        .with_scale_factor_override(1.0),
                    title: "Cellular automata".into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_startup_system(setup)
        .add_system(sync_dimensions)
        .add_system(world_update.run_if(on_timer(Duration::from_millis(tbt))))
        .run()
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    dim: Res<Dimensions>,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let settings = &config.0;

    let mut world = World::new(
        settings.rules.clone(),
        settings.width,
        settings.height,
    );

    let cell_width = dim.width as f32 / world.width as f32;
    let cell_height = dim.height as f32 / world.height as f32;

    let black_material = materials.add(ColorMaterial::from(Color::BLACK));
    let white_material = materials.add(ColorMaterial::from(Color::WHITE));

    let cells_materials = CellsMaterials {
        dead: black_material.clone(),
        alive: white_material.clone(),
    };

    commands.insert_resource(cells_materials);

    for x in 0..world.width {
        for y in 0..world.height {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(
                    cell_width,
                    cell_height,
                )).into()).into(),
                material: white_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    // reverse coordinates, up is down
                    x as f32 * cell_width - dim.width as f32 / 2.0,
                    dim.height as f32 / 2.0 - y as f32 * cell_height,
                    0.0,
                )),
                ..Default::default()
            }, WorldIndex { i: x + y * world.width }));
        }
    }


    // NOT DEBUG
    world.populate(0.2);
    world.revive(0, 0);
    world.revive(0, 1);

    commands.insert_resource(WorldState { world });
}


// tick only every settings.tbt milliseconds
fn world_update(
    mut commands: Commands,
    mut world_state: ResMut<WorldState>,
    query: Query<Entity, With<WorldIndex>>,
    world_index: Query<&WorldIndex>,
    cells_materials: Res<CellsMaterials>,
) {
    world_state.world.tick();
    query.iter().for_each(|entity| {
        let index = world_index.get(entity).unwrap().i;
        commands.entity(entity).remove::<Handle<ColorMaterial>>();
        commands.entity(entity).insert(if world_state.world.cells[index].is_alive {
            cells_materials.alive.clone()
        } else {
            cells_materials.dead.clone()
        });        
    });
}


fn sync_dimensions(dim: Res<Dimensions>, mut windows: Query<&mut Window>) {
    if dim.is_changed() {
        let mut window = windows.single_mut();
        window.resolution.set(dim.width as f32, dim.height as f32);
    }
}