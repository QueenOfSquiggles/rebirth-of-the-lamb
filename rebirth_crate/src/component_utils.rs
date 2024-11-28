use godot::{classes::Engine, obj::WithBaseField, prelude::*};

use crate::engine_helpers::engine;

pub fn register() {
    Engine::singleton().register_singleton(Components::ID, &Components::new_alloc());
}
pub fn unregister() {
    Engine::singleton().unregister_singleton(Components::ID);
}

#[derive(Debug, GodotClass)]
#[class(base=Node, init)]
pub struct ComponentNode {
    base: Base<Node>,
}

#[derive(Debug, GodotClass)]
#[class(base=Node2D, init)]
pub struct ComponentNode2D {
    base: Base<Node2D>,
}

#[derive(Debug, GodotClass)]
#[class(base=Node3D, init)]
pub struct ComponentNode3D {
    base: Base<Node3D>,
}

/// A rust-side helper trait because we cannot inherit from a rust defined class on another rust defined class
///
/// Honestly being able to inherit from my own rust classes would be super awesome for more intuitive systems
pub trait RustyComponent<T>: GodotClass + Inherits<Node> + WithBaseField<Base = T>
where
    T: GodotClass + Inherits<Node>,
{
    fn on_ready(&mut self) {
        self.base_mut()
            .clone()
            .upcast::<Node>()
            .add_to_group(Self::class_name().to_gstring().arg());
    }
}

impl RustyComponent<Node> for ComponentNode {}
impl RustyComponent<Node2D> for ComponentNode2D {}
impl RustyComponent<Node3D> for ComponentNode3D {}

#[godot_api]
impl INode for ComponentNode {
    fn ready(&mut self) {
        self.on_ready();
    }
}

#[godot_api]
impl INode2D for ComponentNode2D {
    fn ready(&mut self) {
        self.on_ready();
    }
}

#[godot_api]
impl INode3D for ComponentNode3D {
    fn ready(&mut self) {
        self.on_ready();
    }
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

    #[func]
    /// An ECS-like query where you can collect all components currently in the scene tree based on the class name
    ///
    /// This is mainly useful for using with a follow-up delineation such as finding all `Damageable` components within a certain range
    pub fn get_all_components(class: GString) -> Array<Gd<Node>> {
        let Some(mut scene) = engine::get_scene_tree() else {
            return Array::new();
        };
        scene.get_nodes_in_group(class.arg())
    }
}

pub struct RustyComponents;

impl RustyComponents {
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

    pub fn get_all_components<T>() -> Option<Vec<Gd<T>>>
    where
        T: GodotClass + Inherits<Node>,
    {
        match Vec::try_from_godot(
            engine::get_scene_tree()?.get_nodes_in_group(T::class_name().to_gstring().arg()),
        ) {
            Ok(set) => Some(
                set.into_iter()
                    .filter_map(|p| match p.try_cast::<T>() {
                        Ok(comp) => Some(comp),
                        Err(_) => None,
                    })
                    .collect(),
            ),
            Err(_) => None,
        }
    }
}
