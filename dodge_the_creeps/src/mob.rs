use godot::engine::{AnimatedSprite2D, RigidBody2D, RigidBody2DVirtual};
use godot::prelude::*;
use rand::seq::SliceRandom;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
struct Mob {
    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visibility_screen_exited(&mut self) {
        self.base.queue_free();
    }
}

#[godot_api]
impl RigidBody2DVirtual for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let mut animated_sprite = self.base.get_node_as::<AnimatedSprite2D>("AnimatedSprite");
        let mob_types = animated_sprite
            .get_sprite_frames()
            .map(|frames| frames.get_animation_names().to_vec())
            .unwrap_or_default();
        let mut rng = rand::thread_rng();
        let animation_name = mob_types.choose(&mut rng).expect("No animation set");
        animated_sprite.set_animation(animation_name.into());
        animated_sprite.play();
    }
}
