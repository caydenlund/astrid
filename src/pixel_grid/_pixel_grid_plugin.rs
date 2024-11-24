use crate::pixel_grid::PixelGrid;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct CameraPanning {
    start_cursor_position: Vec2,
    start_camera_position: Vec2,
}

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (
                camera_zoom,
                camera_pan_start,
                camera_pan_update,
                camera_pan_end,
            ),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn camera_zoom(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let scroll = scroll_evr.read().map(|ev| ev.y).sum::<f32>();
    if scroll == 0.0 {
        return;
    }

    if let (Ok((mut transform, mut projection)), Ok(window)) =
        (query.get_single_mut(), q_window.get_single())
    {
        if let Some(cursor_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            let cursor_pos = cursor_pos - window_size / 2.0;
            let cursor_pos = Vec2::new(cursor_pos.x, -cursor_pos.y);
            let cursor_world_pos = cursor_pos * projection.scale + transform.translation.truncate();

            const ZOOM_SENSITIVITY: f32 = 0.2;
            const MIN_ZOOM: f32 = 0.01;
            const MAX_ZOOM: f32 = 1.5;

            let zoom_delta = -scroll * ZOOM_SENSITIVITY;
            let old_scale = projection.scale;
            projection.scale = (projection.scale * (1.0 + zoom_delta)).clamp(MIN_ZOOM, MAX_ZOOM);

            if (projection.scale - old_scale).abs() > f32::EPSILON {
                let new_cursor_world_pos =
                    cursor_pos * projection.scale + transform.translation.truncate();

                let world_pos_delta = new_cursor_world_pos - cursor_world_pos;
                transform.translation -= world_pos_delta.extend(0.0);
            }
        }
    }
}

fn camera_pan_start(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(Entity, &Transform), With<MainCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let (Ok((camera_entity, camera_transform)), Ok(window)) =
            (camera_query.get_single(), q_window.get_single())
        {
            if let Some(cursor_position) = window.cursor_position() {
                commands.entity(camera_entity).insert(CameraPanning {
                    start_cursor_position: cursor_position,
                    start_camera_position: camera_transform.translation.truncate(),
                });
            }
        }
    }
}

fn camera_pan_update(
    mut query: Query<(&mut Transform, &OrthographicProjection, &CameraPanning), With<MainCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let (Ok((mut transform, projection, panning)), Ok(window)) =
        (query.get_single_mut(), q_window.get_single())
    {
        if let Some(current_cursor_position) = window.cursor_position() {
            let delta = current_cursor_position - panning.start_cursor_position;
            let delta = Vec2::new(delta.x, -delta.y);
            let scaling_factor = projection.scale;

            transform.translation = (panning.start_camera_position - delta * scaling_factor)
                .extend(transform.translation.z);
        }
    }
}


fn camera_pan_end(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, With<CameraPanning>>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        for entity in query.iter() {
            commands.entity(entity).remove::<CameraPanning>();
        }
    }
}

pub struct PixelGridPlugin;

impl Plugin for PixelGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_image)
            .add_systems(Update, update_image);
    }
}

#[derive(Component)]
pub struct GridImage(Handle<Image>);

fn setup_image(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let width = 16;
    let height = 16;
    let mut grid = PixelGrid::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let color = Color::srgba(
                (x as f32) / 255.,
                (y as f32) / 255.,
                0.5,
                1.,
            );
            grid[(x, y)] = color;
        }
    }

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
        GridImage(image_handle.clone()),
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
    ));
}

fn update_image(
    mut images: ResMut<Assets<Image>>,
    query: Query<(&PixelGrid, &GridImage), Changed<PixelGrid>>,
) {
    for (grid, image) in query.iter() {
        if let Some(image) = images.get_mut(&image.0) {
            image.data = {
                let mut image_data = Vec::with_capacity(256 * 256 * 4);
                for x in 0..grid.width() {
                    for y in 0..grid.height() {
                        image_data.extend_from_slice(&grid[(x, y)].to_srgba().to_u8_array());
                    }
                }
                image_data
            };
        }
    }
}
