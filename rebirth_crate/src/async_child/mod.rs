use godot::{
    classes::{object::ConnectFlags, resource_loader::ThreadLoadStatus, Engine, ResourceLoader},
    global::{push_warning, Error},
    obj::WithBaseField,
    prelude::*,
};

mod editor;

#[derive(GodotClass)]
#[class(base=Node, init, tool)]
/// A node that loads a target scene asynchronously
///
/// It can load immediately after the containing scene is loaded in, or on-demand with a function call
pub struct AsyncChild {
    #[export(file = "*.tscn")]
    scene: GString,
    #[export]
    mode: InstanceMode,
    #[export]
    speed: InstanceSpeed,
    #[export]
    child_of: NodePath,
    #[export]
    #[var(get)]
    progress: f32,

    instanced: Option<Gd<Node>>,
    awaiting_load: bool,

    base: Base<Node>,
}

#[derive(GodotConvert, Var, Export, Default, PartialEq, Eq)]
#[godot(via=i32)]
/// The mode for when the scene should be loaded
pub enum InstanceMode {
    #[default]
    OnSelfReady,
    OnParentReady,
    Custom,
}

#[derive(GodotConvert, Var, Export, Default, PartialEq, Eq)]
#[godot(via=i32)]
pub enum InstanceSpeed {
    #[default]
    Lazy,
    AsSoonAsPossible,
}

#[godot_api]
impl INode for AsyncChild {
    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        match self.mode {
            InstanceMode::Custom => (), // do nothing
            InstanceMode::OnSelfReady => {
                self.to_gd()
                    .call_deferred("load_scene", &[true.to_variant()]);
            }
            InstanceMode::OnParentReady => {
                let Some(mut parent) = self.base_mut().get_parent() else {
                    return;
                };
                parent
                    .connect_ex(
                        "ready",
                        &Callable::from_object_method(&self.to_gd(), "load_scene")
                            .bindv(&varray![true]),
                    )
                    .flags((ConnectFlags::DEFERRED.ord() | ConnectFlags::ONE_SHOT.ord()) as u32)
                    .done();
            }
        }
        self.base_mut().set_process(false);
        self.base_mut().set_physics_process(false);
    }

    fn process(&mut self, _delta: f64) {
        if !self.awaiting_load {
            return;
        }
        let progress = Array::<Variant>::new();
        let status = ResourceLoader::singleton()
            .load_threaded_get_status_ex(&self.scene)
            .progress(&varray![progress])
            .done();

        self.progress = progress
            .front()
            .and_then(|var| match f32::try_from_variant(&var) {
                Ok(float) => Some(float),
                Err(_) => None,
            })
            .unwrap_or_default();
        if status == ThreadLoadStatus::IN_PROGRESS {
            // just keep awaiting
            return;
        } else if status == ThreadLoadStatus::LOADED {
            self.handle_loaded_resource(ResourceLoader::singleton().load_threaded_get(&self.scene));
        } else {
            push_warning(&[
                format!("Failed to load async file; error: {}", status.godot_name()).to_variant(),
            ]);
        }
        self.awaiting_load = false;
        self.base_mut().set_process(false);
    }
}

#[godot_api]
impl AsyncChild {
    #[signal]
    /// Emitted after the async scene is finished loading
    pub fn scene_loaded() {}

    #[signal]
    /// Emitted after the async scene managed by this node is unloaded
    pub fn scene_unloaded() {}

    #[func]
    pub fn has_instance(&self) -> bool {
        self.instanced.is_some()
    }

    #[func]
    /// Tells this node to start loading the targeted scene. If you choose `Custom` for your [InstanceMode], you must call this method manually. Additionally, you can choose to pass false to this method to make the loading instant. Though this may defeat the purpose of this node anyway
    pub fn load_scene(&mut self, as_async: bool) {
        if self.has_instance() {
            push_warning(&["Skipping loading scene, already has an instance".to_variant()]);
            return;
        }
        if !as_async {
            self.handle_loaded_resource(ResourceLoader::singleton().load(&self.scene));
        } else {
            let err = ResourceLoader::singleton()
                .load_threaded_request_ex(&self.scene)
                .use_sub_threads(self.speed == InstanceSpeed::AsSoonAsPossible)
                .done();
            if err != Error::OK {
                push_warning(&[format!(
                    "Failed to dispatch async load request for {}, error code: {}",
                    self.scene,
                    err.godot_name()
                )
                .to_variant()]);
                return;
            }
            self.awaiting_load = true;
            self.base_mut().set_process(true);
        }
    }

    #[func]
    pub fn force_finish_load(&mut self) {
        if self.has_instance() {
            push_warning(&["Skipping loading scene, already has an instance".to_variant()]);
            return;
        }
        self.handle_loaded_resource(ResourceLoader::singleton().load_threaded_get(&self.scene));
    }

    #[func]
    /// Tells this node to `queue_free` the managed scene
    pub fn unload_scene(&mut self) {
        let gd = self.to_gd();
        let Some(target) = &mut self.instanced else {
            return;
        };
        target.queue_free();
        target
            .connect_ex(
                "tree_exited",
                &Callable::from_object_method(&gd, "emit_signal").bindv(&varray!["scene_unloaded"]),
            )
            .flags((ConnectFlags::DEFERRED.ord() | ConnectFlags::ONE_SHOT.ord()) as u32)
            .done();
        self.instanced = None;
    }

    fn handle_loaded_resource(&mut self, res_opt: Option<Gd<Resource>>) {
        let Some(res) = res_opt else {
            push_warning(&["Failed to process scene. Failed to load".to_variant()]);
            return;
        };
        let Ok(packed) = res.try_cast::<PackedScene>() else {
            push_warning(&[
                format!("Failed to load scene: {}\nWrong file type", self.scene).to_variant(),
            ]);
            return;
        };
        let Some(mut scene) = packed.instantiate() else {
            push_warning(&[format!(
                "Failed to load scene: {}\nCannot instance packed scene",
                self.scene
            )
            .to_variant()]);
            return;
        };
        let Some(parent) = &mut self.base().get_node_or_null(&self.child_of) else {
            push_warning(&[format!("Failed to find target: {}", self.child_of).to_variant()]);
            return;
        };
        parent.add_child(&scene);
        scene.set_owner(&*parent);
        self.instanced = Some(scene);
        self.base_mut().emit_signal("scene_loaded", &[]);
    }
}
