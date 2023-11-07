use std::marker::PhantomData;

use bevy::{
    prelude::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
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

pub struct CustomMaterial;

impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "sprite_stick.wgsl"
    }

    fn fragment_shader() -> ShaderRef {
        "sprite_stick.wgsl"
    }
}
