use std::{collections::HashMap, path::Path};

use godot::{classes::Engine, prelude::*};

pub fn register() {
    Engine::singleton().register_singleton(EventDepot::ID, &EventDepot::new_alloc());
}

pub fn deregister() {
    Engine::singleton().unregister_singleton(EventDepot::ID);
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
/// The base of all events compatible with the [EventDepot] singleton.
/// This allows management of events being emitted by different elements to be done mostly from the GUI, with some code needed.
struct EventResource {
    base: Base<Resource>,

    #[export]
    /// The default arguments that should be passed when triggering this event
    ///
    /// This determines what the function signature should be for callbacks
    args: VariantArray,
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
/// An intermediary for triggering an event in the [EventDepot]
struct EventArgs {
    #[var(get,set=set_event)]
    #[export]
    /// The event to be triggered
    event: Option<Gd<EventResource>>,
    #[export]
    /// The arguments to pass through this event triggering. Refer to the [EventResource] for more details
    args: VariantArray,
    base: Base<Resource>,
}

#[derive(GodotClass)]
#[class(base=Object, init)]
/// Like an event bus, but the housing for several of those!
///
/// This singleton is intended to be used for registering and calling an [Observer Pattern](https://en.wikipedia.org/wiki/Observer_pattern) without the use of godot signals. Mainly because signals cause too-close binding of different elements in the scene-tree.
///
///
struct EventDepot {
    busses: HashMap<Gd<EventResource>, Vec<Callable>>,
    base: Base<Object>,
}

#[godot_api]
impl EventDepot {
    pub const ID: &'static str = "EventDepot";

    #[func]
    /// Triggers an event using the intermediary [EventArgs]
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
    /// Adds a listener/observer to the given event
    pub fn add_listener(&mut self, event: Gd<EventResource>, target: Callable) {
        self.busses.entry(event).or_insert(Vec::new()).push(target);
    }

    #[func]
    /// Purges invalid callbacks and removes any events with no listeners.
    pub fn clean_depot(&mut self) {
        // in a love/hate relationship with this function :/
        self.busses = self
            .busses
            .iter()
            .filter_map(|(key, callbacks)| {
                let n_callbacks = callbacks
                    .into_iter()
                    .filter(|p| p.is_valid())
                    .cloned()
                    .collect::<Vec<_>>();

                if n_callbacks.is_empty() {
                    return None;
                }
                Some((key.clone(), n_callbacks))
            })
            .collect::<HashMap<_, _>>();
    }
}

#[godot_api]
impl EventArgs {
    #[func]
    pub fn set_event(&mut self, value: Option<Gd<EventResource>>) {
        if let Some(event) = &value {
            if let Some(name) = Path::new(&event.get_path().to_string()).file_stem() {
                self.base_mut().set_name(name.to_str().unwrap_or_default());
            }
            self.args = event.bind().args.clone();
        }
        self.event = value;
    }
}
