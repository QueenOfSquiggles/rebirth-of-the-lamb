use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
/// A player class for testing purposes
struct Player {
	#[export]
	/// a currently unused speed value
    speed: f64,

    #[export]
    /// The angular speed at which the player rotates
    angular_speed: f64,

    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
	fn init(base: Base<Sprite2D>) -> Self {
		Self {
			speed: 400.0,
			angular_speed: std::f64::consts::PI,
			base,
		}
	}	

	fn physics_process(&mut self, delta: f64) {
		let rads = (self.angular_speed * delta) as f32;
		self.base_mut().rotate(rads);
	}
}
