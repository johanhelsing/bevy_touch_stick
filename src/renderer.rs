use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

use crate::StickIdType;

struct StickRendererPlugin<S: StickIdType> {
    marker: PhantomData<S>,
}

impl<S: StickIdType> Default for StickRendererPlugin<S> {
    fn default() -> Self {
        Self { marker: default() }
    }
}

impl<S: StickIdType> Plugin for StickRendererPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
    }
}

#[derive(Asset, AsBindGroup, Clone, TypePath)]
pub struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "sprite_stick.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "sprite_stick.wgsl".into()
    }
}
