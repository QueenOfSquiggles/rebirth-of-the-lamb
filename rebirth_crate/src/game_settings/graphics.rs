use godot::classes::viewport::{Msaa, ScreenSpaceAa};
use godot::classes::{CameraAttributes, Environment, Viewport};
use godot::prelude::*;

use crate::engine_helpers::engine;

#[derive(Debug, GodotClass)]
#[class(base=Resource, tool)]
pub struct GraphicsSettings {
    base: Base<Resource>,
    //
    //  Viewport config
    //
    #[export(enum =
        (   None=Msaa::DISABLED.ord().into(),
            X2=Msaa::MSAA_2X.ord().into(),
            X4=Msaa::MSAA_4X.ord().into(),
            X8=Msaa::MSAA_8X.ord().into()
        ))]
    msaa: i32,

    #[export]
    use_taa: bool,
    #[export]
    use_fxaa: bool,
    #[export]
    use_occlusion_culling: bool,
    #[export]
    use_debanding: bool,

    #[export]
    mesh_lod_pixels: f32,

    //
    //  Camera config
    //
    #[export]
    use_dynamic_exposure: bool,
    #[export]
    static_exposure: f32,
    #[export]
    dynamic_exposure: f32,
    #[export]
    dynamic_exposure_speed: f32,
    //
    //  Environment config
    //
    #[export]
    fog_enabled: bool,
    #[export]
    glow_enabled: bool,
    #[export]
    sdfgi_enabled: bool,
    #[export]
    ssr_enabled: bool,
    #[export]
    ssao_enabled: bool,
    #[export]
    ssil_enabled: bool,
    #[export]
    volumetric_fog_enabled: bool,
}

#[godot_api]
impl IResource for GraphicsSettings {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            msaa: Msaa::MSAA_2X.ord().into(),
            mesh_lod_pixels: 32.0,
            base,
            use_taa: true,
            use_fxaa: false,
            use_occlusion_culling: true,
            use_debanding: true,
            use_dynamic_exposure: false,
            static_exposure: 1.0,
            dynamic_exposure: 1.0,
            dynamic_exposure_speed: 1.0,
            fog_enabled: true,
            glow_enabled: true,
            sdfgi_enabled: true,
            ssr_enabled: true,
            ssao_enabled: true,
            ssil_enabled: true,
            volumetric_fog_enabled: true,
        }
    }
}

#[godot_api]
impl GraphicsSettings {
    #[func]
    pub fn apply(&self) {
        let Some((mut viewport, mut env, mut cam)) = get_graphics_targets() else {
            return;
        };
        // Viewport
        viewport.set_msaa_3d(Msaa::from_ord(self.msaa));
        viewport.set_mesh_lod_threshold(self.mesh_lod_pixels);
        viewport.set_use_taa(self.use_taa);
        viewport.set_use_occlusion_culling(self.use_occlusion_culling);
        viewport.set_use_debanding(self.use_debanding);
        viewport.set_screen_space_aa(match self.use_fxaa {
            true => ScreenSpaceAa::FXAA,
            false => ScreenSpaceAa::DISABLED,
        });

        // Camera
        cam.set_auto_exposure_enabled(self.use_dynamic_exposure);
        if self.use_dynamic_exposure {
            cam.set_exposure_multiplier(self.static_exposure);
        } else {
            cam.set_auto_exposure_scale(self.dynamic_exposure);
            cam.set_auto_exposure_speed(self.dynamic_exposure_speed);
        }

        // Environment
        env.set_fog_enabled(self.fog_enabled);
        env.set_glow_enabled(self.glow_enabled);
        env.set_sdfgi_enabled(self.sdfgi_enabled);
        env.set_ssr_enabled(self.ssr_enabled);
        env.set_ssao_enabled(self.ssao_enabled);
        env.set_ssil_enabled(self.ssil_enabled);
        env.set_volumetric_fog_enabled(self.volumetric_fog_enabled);
    }
}

fn get_graphics_targets() -> Option<(Gd<Viewport>, Gd<Environment>, Gd<CameraAttributes>)> {
    let Some(viewport) = engine::get_viewport() else {
        return None;
    };
    let Some(world) = viewport.get_world_3d() else {
        return None;
    };
    let Some(cam) = world.get_camera_attributes() else {
        return None;
    };
    let Some(env) = world.get_environment() else {
        return None;
    };
    return Some((viewport, env, cam));
}
