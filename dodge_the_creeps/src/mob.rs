use godot::engine::{AnimatedSprite2D, RigidBody2D, RigidBody2DVirtual};
use godot::prelude::*;
use rand::seq::SliceRandom;

#[derive(GodotClass)]
#[class(base = RigidBody2D)]
pub struct Mob {
    animated_sprite: Option<Gd<AnimatedSprite2D>>,
    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visible_on_screen_notifier_2d_screen_exited(&mut self) {
        self.base.queue_free();
    }
}

#[godot_api]
impl RigidBody2DVirtual for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Self {
            animated_sprite: None,
            base,
        }
    }

    fn ready(&mut self) {
        let Self {
            base,
            animated_sprite,
        } = self;
        animated_sprite.replace({
            let mut animated_sprite = base.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
            animated_sprite.play();
            let mob_types = animated_sprite
                .get_sprite_frames()
                .map(|frames| frames.get_animation_names().to_vec())
                .unwrap_or_default();
            let mut rng = rand::thread_rng();
            let animation_name = mob_types.choose(&mut rng).expect("No animation set");
            animated_sprite.set_animation(animation_name.into());
            animated_sprite
        });
    }
}
