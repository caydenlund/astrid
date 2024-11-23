use astrid::pixel_grid::PixelGridPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelGridPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// use bevy::prelude::*;
// use bevy::render::render_asset::RenderAssetUsages;
// use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
//
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .run();
// }
//
// fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
//     // Create camera
//     commands.spawn(Camera2dBundle::default());
//
//     // Create the image data
//     let mut image_data = Vec::with_capacity(256 * 256 * 4);
//     for row in 0..256 {
//         for col in 0..256 {
//             image_data.push(row as u8); // Red component (row)
//             image_data.push(col as u8); // Green component (column)
//             image_data.push(255); // Blue component (always 255)
//             image_data.push(255); // Alpha component (always 255)
//         }
//     }
//
//     // Create the image
//     let image = Image::new(
//         Extent3d {
//             width: 256,
//             height: 256,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         image_data,
//         TextureFormat::Rgba8Unorm,
//         RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
//     );
//
//     // Add image to assets
//     let image_handle = images.add(image);
//
//     // Spawn sprite
//     commands.spawn(SpriteBundle {
//         texture: image_handle,
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..default()
//     });
// }
