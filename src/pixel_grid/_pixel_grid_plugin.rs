use crate::pixel_grid::PixelGrid;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub struct PixelGridPlugin;

impl Plugin for PixelGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid)
            .add_systems(Update, update_grid);
    }
}

#[derive(Bundle)]
pub struct PixelGridBundle {
    grid: PixelGrid,
    transform: Transform,
    global_transform: GlobalTransform,
    inherited_visibility: InheritedVisibility,
    texture: Handle<Image>,
}

impl Default for PixelGridBundle {
    fn default() -> Self {
        Self {
            grid: PixelGrid::new(0, 0),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            inherited_visibility: InheritedVisibility::default(),
            texture: Handle::default(),
        }
    }
}

fn setup_grid(mut commands: Commands, window: Query<&Window>, mut images: ResMut<Assets<Image>>) {
    println!("setting grid colors");

    let width = 256;
    let height = 256;
    let mut grid = PixelGrid::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let color = Color::srgba(
                x as f32 / (width as f32 - 1.),
                y as f32 / (height as f32 - 1.),
                0.,
                1.,
            );
            grid[(x, y)] = color;
        }
    }

    println!("spawning entity grid");
    let window = window.single();

    println!("spawning pixels");

    let image_data = {
        let mut image_data = Vec::with_capacity(256 * 256 * 4);
        for x in 0..width {
            for y in 0..height {
                image_data.extend_from_slice(&grid[(x, y)].to_srgba().to_u8_array());
            }
        }
        image_data
    };

    let image = {
        let size = Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };

        let mut image = Image::new(
            size,
            TextureDimension::D2,
            image_data,
            TextureFormat::Rgba8Unorm,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );
        image.sampler = bevy::render::texture::ImageSampler::nearest();
        image
    };

    let image_handle = images.add(image);
    commands.spawn((
        grid,
        SpriteBundle {
            texture: image_handle,
            transform: Transform::default(),
            ..default()
        },
    ));
}

fn update_grid(
    mut grid: Query<(&PixelGrid, &mut Transform, &Handle<Image>)>,
    mut images: ResMut<Assets<Image>>,
) {
    println!("updating grid");
    let (grid, mut transform, handle) = grid.single_mut();
    transform.scale.x *= 1.01;
    transform.scale.y *= 1.01;

    if let Some(image) = images.get_mut(handle.id()) {
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let color = grid[(x, y)].to_srgba();
                image.data[4 * (y * grid.width() + x) + 0] = (color.red * 255.0).round() as u8;
                image.data[4 * (y * grid.width() + x) + 1] = (color.green * 255.0).round() as u8;
                image.data[4 * (y * grid.width() + x) + 2] = (color.blue * 255.0).round() as u8;
                image.data[4 * (y * grid.width() + x) + 3] = (color.alpha * 255.0).round() as u8;
            }
        }
    }
}
