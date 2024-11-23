use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PixelComponent {
    pub x: usize,
    pub y: usize,
}

#[derive(Bundle, Default)]
pub struct PixelBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,

    pub pixel: PixelComponent,
}
