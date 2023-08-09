use godot::engine::{AnimatedSprite2D, Area2D, Area2DVirtual, CollisionShape2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Area2D)]
pub struct Player {
    animated_sprite: Option<Gd<AnimatedSprite2D>>,
    collision_shape: Option<Gd<CollisionShape2D>>,
    input: Option<Gd<Input>>,
    screen_size: Vector2,
    #[export]
    speed: f32,
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn hit();

    #[func]
    fn on_body_entered(&mut self) {
        let Self {
            base,
            collision_shape,
            ..
        } = self;
        let collision_shape = collision_shape.as_mut().unwrap();
        base.hide();
        base.emit_signal("hit".into(), &[]);
        collision_shape.set_deferred("disabled".into(), true.to_variant());
    }

    #[func]
    pub fn start(&mut self, position: Vector2) {
        let Self {
            base,
            collision_shape,
            ..
        } = self;
        let collision_shape = collision_shape.as_mut().unwrap();
        base.set_position(position);
        base.show();
        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl Area2DVirtual for Player {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            animated_sprite: None,
            collision_shape: None,
            input: None,
            screen_size: Vector2::ZERO,
            speed: 400.0,
            base,
        }
    }

    fn ready(&mut self) {
        let Self {
            base,
            animated_sprite,
            collision_shape,
            input,
            screen_size,
            ..
        } = self;
        animated_sprite.replace(base.get_node_as::<AnimatedSprite2D>("AnimatedSprite2D"));
        collision_shape.replace(base.get_node_as::<CollisionShape2D>("CollisionShape2D"));
        input.replace(Input::singleton());
        *screen_size = base.get_viewport_rect().size;

        base.hide();
    }

    fn process(&mut self, delta: f64) {
        let Self {
            animated_sprite,
            input,
            speed,
            screen_size,
            base,
            ..
        } = self;
        let animated_sprite = animated_sprite.as_mut().unwrap();
        let input = input.as_ref().unwrap();

        let mut velocity = Vector2::ZERO;
        velocity.x = input.get_action_strength("move_right".into())
            - input.get_action_strength("move_left".into());
        velocity.y = input.get_action_strength("move_down".into())
            - input.get_action_strength("move_up".into());

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * *speed;

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

        let position =
            (base.get_position() + velocity * delta as f32).clamp(Vector2::ZERO, *screen_size);
        base.set_position(position);
    }
}
