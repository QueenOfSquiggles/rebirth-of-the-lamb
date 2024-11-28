use godot::{
    classes::{IRayCast3D, RayCast3D},
    global::Error,
    obj::WithBaseField,
    prelude::*,
};

use crate::component_utils::{RustyComponent, RustyComponents};

#[derive(Debug, GodotClass)]
#[class(base=RayCast3D, init)]
struct Interactor {
    last_interaction: Option<Gd<Node3D>>,
    base: Base<RayCast3D>,
}

#[godot_api]
impl IRayCast3D for Interactor {
    fn physics_process(&mut self, _delta: f64) {
        let option_collider =
            self.base()
                .get_collider()
                .and_then(|coll| match coll.try_cast::<Node3D>() {
                    Ok(node) => Some(node),
                    Err(_) => None,
                });
        if option_collider == self.last_interaction {
            return;
        }
        if let Some(last) = self.last_interaction.clone() {
            if let Some(mut interact) =
                RustyComponents::get_component::<InteractionComponent>(&last.upcast())
            {
                interact.emit_signal(InteractionComponent::SIGNAL_DESELECT, &[]);
            }
        }
        if let Some(new) = option_collider.clone() {
            if let Some(mut interact) =
                RustyComponents::get_component::<InteractionComponent>(&new.upcast())
            {
                interact.emit_signal(InteractionComponent::SIGNAL_SELECT, &[]);
            }
        }
        self.last_interaction = option_collider;
    }
}

#[godot_api]
impl Interactor {
    #[func]
    fn do_interact(&self) -> bool {
        let Some(node) = &self.last_interaction else {
            return false;
        };
        let Some(mut interact) =
            RustyComponents::get_component::<InteractionComponent>(&node.clone().upcast())
        else {
            return false;
        };
        interact.emit_signal(InteractionComponent::SIGNAL_INTERACT, &[]) == Error::OK
    }
}

#[derive(Debug, GodotClass)]
#[class(base=Node3D, init)]
/// A component for creating an interactable object
struct InteractionComponent {
    base: Base<Node3D>,
}

impl RustyComponent<Node3D> for InteractionComponent {}

#[godot_api]
impl INode3D for InteractionComponent {
    fn ready(&mut self) {
        self.on_ready();
    }
}

#[godot_api]
impl InteractionComponent {
    pub const SIGNAL_SELECT: &'static str = "on_select";
    pub const SIGNAL_DESELECT: &'static str = "on_deselect";
    pub const SIGNAL_INTERACT: &'static str = "on_interact";

    #[signal]
    /// Emitted when the player is first able to interact but hasn't
    pub fn on_select() {}

    #[signal]
    /// Emitted when the player is no longer able to interact
    pub fn on_deselect() {}

    #[signal]
    /// Emitted when the player has actually intentionally interacted with this object
    pub fn on_interact() {}
}
