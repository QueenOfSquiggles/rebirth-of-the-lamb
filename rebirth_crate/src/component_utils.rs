use godot::{classes::Engine, prelude::*};

pub fn register() {
    Engine::singleton().register_singleton(Components::ID, &Components::new_alloc());
}
pub fn unregister() {
    Engine::singleton().unregister_singleton(Components::ID);
}

#[derive(Debug, GodotClass)]
#[class(base=Object, init, tool)]
/// A utility singleton for performing component-oriented operations
pub struct Components {
    base: Base<Object>,
}

#[godot_api]
impl Components {
    pub const ID: &'static str = "Components";

    #[func]
    /// Tries to get a component from the target assuming components are an immediate child of the target
    ///
    /// Returns null if target of class cannot be found
    pub fn get_component(target: Gd<Node>, class: GString) -> Option<Gd<Node>> {
        for child in target.get_children().iter_shared() {
            if child.get_class() == class {
                return Some(child);
            }
        }
        None
    }
}

// impl CrateSingleton for Components {
//     const ID: &'static str = "Components";
// }

pub struct ComponentsRs;

impl ComponentsRs {
    pub fn get_component<T>(target: &Gd<Node>) -> Option<Gd<T>>
    where
        T: GodotClass + Inherits<Node>,
    {
        let class_name = T::class_name().to_gstring();
        for child in target.get_children().iter_shared() {
            if child.get_class() == class_name {
                let Ok(cast) = child.try_cast::<T>() else {
                    continue;
                };
                return Some(cast);
            }
        }
        None
    }
}
