use crate::triplanar_material::TriplanarMaterial;
use bevy::asset::{embedded_asset, load_internal_asset, weak_handle};
use bevy::prelude::*;

const TRIPLANAR_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("0196ba23-57d7-7691-8cdd-49257195c511");
const BIPLANAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("6439bf52-280e-427c-b5ea-a6de1137a57d");

pub struct TriplanarMaterialPlugin;

impl Plugin for TriplanarMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TriplanarMaterial>::default());

        load_internal_asset!(
            app,
            TRIPLANAR_SHADER_HANDLE,
            "shaders/triplanar.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            BIPLANAR_SHADER_HANDLE,
            "shaders/biplanar.wgsl",
            Shader::from_wgsl
        );
        embedded_asset!(app, "shaders/triplanar_material_vert.wgsl");
        embedded_asset!(app, "shaders/triplanar_material_frag.wgsl");
    }
}
