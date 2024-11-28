pub mod engine {
    use godot::{
        classes::{Engine, SceneTree, Viewport},
        obj::Gd,
    };
    pub fn get_scene_tree() -> Option<Gd<SceneTree>> {
        match Engine::singleton().get_main_loop()?.try_cast::<SceneTree>() {
            Ok(tree) => Some(tree),
            Err(_) => None,
        }
    }

    pub fn get_viewport() -> Option<Gd<Viewport>> {
        get_scene_tree()?.get_root()?.get_viewport()
    }
}
