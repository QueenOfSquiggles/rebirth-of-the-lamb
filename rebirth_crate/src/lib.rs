use godot::prelude::*;

mod event_depot;
mod test;

struct RebirthCrate;

#[gdextension]
unsafe impl ExtensionLibrary for RebirthCrate {
    fn on_level_init(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }
        event_depot::register();
    }

    fn on_level_deinit(level: InitLevel) {
        if level != InitLevel::Scene {
            return;
        }
        event_depot::deregister();
    }
}
