use godot::{
    classes::{
        box_container::AlignmentMode, object::ConnectFlags, Button, EditorInspectorPlugin,
        EditorPlugin, HBoxContainer, IEditorInspectorPlugin, IEditorPlugin, PanelContainer,
    },
    prelude::*,
};

use crate::engine_helpers::engine;

use super::AsyncChild;

#[derive(GodotClass)]
#[class(base=EditorPlugin, tool, init)]
struct AsyncChildEditorPlugin {
    base: Base<EditorPlugin>,
    async_child_inspector: Option<Gd<AsyncChildInspectorPlugin>>,
}

#[godot_api]
impl IEditorPlugin for AsyncChildEditorPlugin {
    fn enter_tree(&mut self) {
        let inspector = AsyncChildInspectorPlugin::new_gd();
        self.base_mut().add_inspector_plugin(&inspector);
        self.async_child_inspector = Some(inspector);
        let ref_self = self.to_gd().clone();
        self.base_mut().add_tool_menu_item(
            "load_all_async",
            &Callable::from_object_method(&ref_self, "tool_load_all_scenes"),
        );
        self.base_mut().add_tool_menu_item(
            "unload_all_async",
            &Callable::from_object_method(&ref_self, "tool_unload_all_scenes"),
        );
    }
    fn exit_tree(&mut self) {
        if let Some(inspector) = self.async_child_inspector.clone() {
            self.base_mut().remove_inspector_plugin(&inspector);
        };
    }
}

#[godot_api]
impl AsyncChildEditorPlugin {
    #[func]
    fn tool_load_all_scenes(&mut self) {
        let Some(ed) = self.base_mut().get_editor_interface() else {
            return;
        };
        let Some(root) = ed.get_edited_scene_root() else {
            return;
        };
        for child in Self::get_all_valid_nodes(&root).iter_mut() {
            child.bind_mut().load_scene(true);
        }
    }

    #[func]
    fn tool_unload_all_scenes(&mut self) {
        let Some(ed) = self.base_mut().get_editor_interface() else {
            return;
        };
        let Some(root) = ed.get_edited_scene_root() else {
            return;
        };
        for child in Self::get_all_valid_nodes(&root).iter_mut() {
            child.bind_mut().unload_scene();
        }
    }

    fn get_all_valid_nodes(root: &Gd<Node>) -> Vec<Gd<AsyncChild>> {
        let mut buffer = Vec::new();
        for child in root.get_children().iter_shared() {
            buffer.append(&mut Self::get_all_valid_nodes(&child));
            if let Ok(asc) = child.try_cast::<AsyncChild>() {
                buffer.push(asc);
            }
        }
        buffer
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=EditorInspectorPlugin)]
struct AsyncChildInspectorPlugin {
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for AsyncChildInspectorPlugin {
    fn can_handle(&self, object: Option<Gd<Object>>) -> bool {
        object
            .and_then(|o| match o.try_cast::<AsyncChild>() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .is_some()
    }

    fn parse_begin(&mut self, object: Option<Gd<Object>>) {
        let Some(obj) = object else {
            return;
        };
        let mut panel = PanelContainer::new_alloc();
        let mut hbox = HBoxContainer::new_alloc();
        let mut btn_load = Button::new_alloc();
        let mut btn_unload = Button::new_alloc();
        hbox.add_child(&btn_load);
        hbox.add_child(&btn_unload);
        panel.add_child(&hbox);
        self.base_mut().add_custom_control(&panel);

        hbox.set_alignment(AlignmentMode::CENTER);

        btn_unload.set_text("Unload");
        btn_load.set_text("Load");

        let obj_var = obj.to_variant();
        let _ = btn_unload
            .connect_ex(
                "pressed",
                &Callable::from_fn("button_unload_scene", button_unload_scene)
                    .bindv(&array![&obj_var]),
            )
            .flags(ConnectFlags::DEFERRED.ord() as u32)
            .done();
        let _ = btn_load
            .connect_ex(
                "pressed",
                &Callable::from_fn("button_load_scene", button_load_scene).bindv(&array![&obj_var]),
            )
            .flags(ConnectFlags::DEFERRED.ord() as u32)
            .done();
    }
}

fn button_load_scene(args: &[&Variant]) -> Result<Variant, ()> {
    let Some(child) = &mut args
        .first()
        .and_then(|a| match a.try_to::<Gd<AsyncChild>>() {
            Ok(child) => Some(child),
            Err(_) => None,
        })
    else {
        return Err(());
    };
    child.bind_mut().load_scene(false);

    Ok(Variant::nil())
}

fn button_unload_scene(args: &[&Variant]) -> Result<Variant, ()> {
    let Some(child) = &mut args
        .first()
        .and_then(|a| match a.try_to::<Gd<AsyncChild>>() {
            Ok(child) => Some(child),
            Err(_) => None,
        })
    else {
        return Err(());
    };
    child.bind_mut().unload_scene();
    Ok(Variant::nil())
}

fn button_pressed(_args: &[&Variant]) -> Result<Variant, ()> {
    godot_print!("Button was pressed!");
    Ok(Variant::nil())
}
