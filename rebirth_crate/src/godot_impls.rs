// use godot::{classes::Engine, prelude::*};

// TODO: why are singletons not working as I expect???

// pub trait CrateSingleton: GodotClass + Inherits<Object> {
//     const ID: &'static str;
//     fn try_singleton() -> Option<Gd<Self>> {
//         Engine::singleton()
//             .get_singleton(Self::ID)
//             .and_then(|obj| match obj.try_cast::<Self>() {
//                 Ok(sel) => Some(sel),
//                 Err(_) => None,
//             })
//     }

//     fn singleton() -> Gd<Self> {
//         Engine::singleton()
//             .get_singleton(Self::ID)
//             .unwrap()
//             .try_cast::<Self>()
//             .unwrap()
//     }
// }
