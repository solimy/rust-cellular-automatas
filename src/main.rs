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
    window::WindowResolution, render::{texture::CompressedImageFormats, render_resource::{Extent3d, TextureFormat, TextureDimension}},
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

#[derive(Component)]
struct WorldRepr {
    handle: Handle<Image>,
}

#[derive(Resource)]
struct WorldState {
    world: World,
}


fn main() {
    let config = Config::default();
    let tbt = config.0.tbt;
    let scale = config.0.scale;
    let width = (config.0.width * scale) as f32;
    let height = (config.0.height * scale) as f32;
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
    mut images: ResMut<Assets<Image>>,
    config: Res<Config>,
) {
    commands.spawn(Camera2dBundle::default());

    let handle = images.add(Image {..default()});

    commands.spawn((
        WorldRepr {handle: handle.clone()},
        SpriteBundle {
            texture: handle.clone(),
            ..Default::default()
        },
    ));

    let settings = &config.0;

    let mut world = World::new(
        settings.rules.clone(),
        settings.width,
        settings.height,
    );

    world.populate(0.2);
    world.revive(0, 0);
    world.revive(0, 1);

    commands.insert_resource(WorldState { world });
}


fn world_update(
    mut images: ResMut<Assets<Image>>,
    mut world_state: ResMut<WorldState>,
    dim: Res<Dimensions>,
    mut query: Query<&mut WorldRepr>,
) {
    world_state.world.tick();

    let mut world_repr = query.single_mut();

    let cell_width = dim.width as usize / world_state.world.width;
    let cell_height = dim.height as usize / world_state.world.height;

    let mut image_byte_buffer = vec![0; dim.width as usize * dim.height as usize * 4];

    world_state.world.cells.iter().enumerate().for_each(|(i, cell)| {
        let x = i % world_state.world.width;
        let y = i / world_state.world.width;

        let x = x * cell_width;
        let y = y * cell_height;

        let color = if cell.is_alive {
            [255, 255, 255, 255]
        } else {
            [0, 0, 0, 255]
        };

        for x in x..x + cell_width {
            for y in y..y + cell_height {
                let i = (x + y * dim.width as usize) * 4;
                image_byte_buffer[i..i + 4].copy_from_slice(&color);
            }
        }
    });


    let image = Image::new(
        Extent3d {
            width: dim.width as u32,
            height: dim.height as u32,
            ..default()
        },
        TextureDimension::D2,
        image_byte_buffer,
        TextureFormat::Rgba8UnormSrgb,
    );

    world_repr.handle = images.set(world_repr.handle.clone(), image);

}


fn sync_dimensions(dim: Res<Dimensions>, mut windows: Query<&mut Window>) {
    if dim.is_changed() {
        let mut window = windows.single_mut();
        window.resolution.set(dim.width as f32, dim.height as f32);
    }
}