use std::collections::HashMap;

use godot::{classes::Engine, prelude::*};

pub fn register() {
    Engine::singleton()
        .register_singleton(EventDepot::ID, &EventDepot::new_alloc().upcast::<Object>());
}

pub fn deregister() {
    Engine::singleton().unregister_singleton(EventDepot::ID);
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
struct EventResource {
    base: Base<Resource>,
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
struct EventArgs {
    #[export]
    event: Option<Gd<EventResource>>,
    #[export]
    args: VariantArray, // TODO: auto-populate with default args when event is set
    base: Base<Resource>,
}

#[derive(GodotClass)]
#[class(base=Object, init)]
struct EventDepot {
    busses: HashMap<Gd<EventResource>, Vec<Callable>>,
    base: Base<Object>,
}

#[godot_api]
impl EventDepot {
    pub const ID: &'static str = "EventDepot";

    #[func]
    pub fn trigger(&self, event_args: Gd<EventArgs>) {
        let Some(event) = event_args.bind().event.clone() else {
            godot_error!("No event associated with provided EventArgs");
            return;
        };
        let Some(callbacks) = self.busses.get(&event) else {
            godot_warn!("No callbacks registered for given event {}", event);
            return;
        };
        for call in callbacks.into_iter().filter(|p| p.is_valid()) {
            call.callv(&event_args.bind().args);
        }
    }

    #[func]
    pub fn add_listener(&mut self, event: Gd<EventResource>, target: Callable) {
        self.busses.entry(event).or_insert(Vec::new()).push(target);
    }

    #[func]
    pub fn clean_depot(&mut self) {
        // TODO: purge invalid callbacks and disconnected event refs
    }
}
