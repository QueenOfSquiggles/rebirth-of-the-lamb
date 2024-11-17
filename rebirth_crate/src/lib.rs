use godot::prelude::*;

mod test;

struct RebirthCrate;

#[gdextension]
unsafe impl ExtensionLibrary for RebirthCrate {}
