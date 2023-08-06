use godot::engine::{AnimatedSprite2D, Area2D, Area2DVirtual, CollisionShape2D, PhysicsBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Player {
    speed: f64,

    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func]
    fn on_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        self.base.hide();
        self.base.emit_signal("hit".into(), &[]);
        self.base
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_deferred("disabled".into(), true.to_variant());
    }

    #[func]
    pub fn start(&mut self, position: Vector2) {
        self.base.set_global_position(position);
        self.base.show();
        self.base
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_disabled(false);
    }
}

#[godot_api]
impl Area2DVirtual for Player {
    fn init(base: Base<Area2D>) -> Self {
        Self { speed: 400.0, base }
    }

    fn ready(&mut self) {
        self.base.hide();
    }

    fn physics_process(&mut self, delta: f64) {
        let screen_size = self.base.get_viewport_rect().size;
        let input = Input::singleton();
        let mut animated_sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::ZERO;

        if input.is_action_pressed("move_right".into()) {
            velocity.x += 1.0;
        }
        if input.is_action_pressed("move_left".into()) {
            velocity.x -= 1.0;
        }
        if input.is_action_pressed("move_down".into()) {
            velocity.y += 1.0;
        }
        if input.is_action_pressed("move_up".into()) {
            velocity.y -= 1.0;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed as f32;

            if velocity.x != 0.0 {
                animated_sprite.set_animation("walk".into());
                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0);
            } else if velocity.y != 0.0 {
                animated_sprite.set_animation("up".into());
                animated_sprite.set_flip_v(velocity.y > 0.0);
            }

            animated_sprite.play();
        } else {
            animated_sprite.stop();
        }

        let position = (self.base.get_global_position() + velocity * delta as f32)
            .clamp(Vector2::ZERO, screen_size);
        self.base.set_global_position(position);
    }
}
