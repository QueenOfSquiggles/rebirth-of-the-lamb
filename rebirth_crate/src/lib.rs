use godot::prelude::*;

mod async_child;
mod component_utils;
pub mod engine_helpers;
mod event_depot;
mod game_settings;
mod godot_impls;
mod interactables;
mod state_machine;
struct RebirthCrate;

#[gdextension]
unsafe impl ExtensionLibrary for RebirthCrate {
    fn on_level_init(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }
        event_depot::register();
        game_settings::register();
        component_utils::register();
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }
        event_depot::unregister();
        game_settings::unregister();
        component_utils::unregister();
    }
}
