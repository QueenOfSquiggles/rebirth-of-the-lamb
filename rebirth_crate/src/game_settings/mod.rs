use godot::{
    classes::{Engine, ProjectSettings},
    meta::AsArg,
    prelude::*,
};
use graphics::GraphicsSettings;

mod graphics;

pub fn register() {
    Engine::singleton().register_singleton(GameSettings::ID, &GameSettings::new_alloc());
}

pub fn deregister() {
    Engine::singleton().unregister_singleton(GameSettings::ID);
}

#[derive(Debug, GodotClass)]
#[class(base=Object, tool)]
struct GameSettings {
    base: Base<Object>,

    graphics: Gd<GraphicsSettings>,
}

#[godot_api]
impl IObject for GameSettings {
    fn init(base: Base<Object>) -> Self {
        let gfx_path = get_setting_or_default(
            Self::SETTINGS_GFX,
            "res://settings/graphics.tres".to_variant(),
        );

        let gfx =
            try_load::<GraphicsSettings>(&GString::from_variant(&gfx_path)).unwrap_or_default();

        Self {
            base,
            graphics: gfx,
        }
    }
}

#[godot_api]
impl GameSettings {
    pub const ID: &'static str = "GameSettings";
    const SETTINGS_GFX: &'static str = "GameSettings/GraphicsSettings";
    #[func]
    fn apply(&mut self) {
        self.graphics.bind().apply();
    }
}

fn get_setting_or_default(name: impl AsArg<GString> + Clone, fallback: Variant) -> Variant {
    let initial = ProjectSettings::singleton().get_setting(name.clone());
    if initial.is_nil() {
        ProjectSettings::singleton().set_setting(name, &fallback);
        return fallback;
    }
    initial
}
