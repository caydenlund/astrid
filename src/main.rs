use astrid::pixel_grid::{CameraControlPlugin, PixelGridPlugin};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelGridPlugin)
        .add_plugins(CameraControlPlugin)
        .run();
}
