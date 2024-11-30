use godot::{classes::InputEvent, prelude::*};

#[derive(GodotClass)]
#[class(base=Node, init)]
/// The root of any particular state machine
struct FiniteStateMachine {
    #[export]
    entry: NodePath,
    active: Option<Gd<StateMachineState>>,
    base: Base<Node>,
}

#[derive(GodotClass)]
#[class(base=Node, init)]
/// A single leaf node of a state machine
///
/// Can be a direct child of a [FiniteStateMachine] or of a [HierarchicalState] for more complex machines
struct StateMachineState {
    base: Base<Node>,
}

#[godot_api]
impl INode for FiniteStateMachine {
    fn ready(&mut self) {
        self.set_state(self.entry.clone());
    }

    fn process(&mut self, delta: f64) {
        let Some(active) = &mut self.active else {
            return;
        };
        active.bind_mut().state_process(delta);
    }

    fn physics_process(&mut self, delta: f64) {
        let Some(active) = &mut self.active else {
            return;
        };
        active.bind_mut().state_physics(delta);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let Some(active) = &mut self.active else {
            return;
        };
        if active.bind_mut().state_input(event) {
            if let Some(viewport) = &mut self.base_mut().get_viewport() {
                viewport.set_input_as_handled();
            }
        }
    }
}

#[godot_api]
impl FiniteStateMachine {
    #[func]
    fn set_state(&mut self, next: NodePath) {
        if let Some(prev) = &mut self.active {
            prev.bind_mut().state_exit();
        }
        self.active = self.base().get_node_or_null(&next).and_then(|n| {
            match n.try_cast::<StateMachineState>() {
                Ok(state) => Some(state),
                Err(_) => None,
            }
        });
        if let Some(new) = &mut self.active {
            new.bind_mut().state_enter();
        }
    }
}

#[godot_api]
impl StateMachineState {
    #[func(virtual)]
    fn state_enter(&mut self) {}
    #[func(virtual)]
    fn state_exit(&mut self) {}

    #[func(virtual)]
    fn state_process(&mut self, _delta: f64) {}
    #[func(virtual)]
    fn state_physics(&mut self, _delta: f64) {}
    #[func(virtual)]
    fn state_input(&mut self, _event: Gd<InputEvent>) -> bool {
        false
    }
}
