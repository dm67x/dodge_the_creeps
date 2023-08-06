use std::f32::consts::PI;

use godot::{
    engine::{Marker2D, PathFollow2D, RigidBody2D, Timer},
    prelude::*,
};
use rand::Rng;

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
struct Game {
    score: u32,

    #[export]
    mob_scene: Gd<PackedScene>,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Game {
    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        let mut score_timer = self.base.get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base.get_node_as::<Timer>("MobTimer");
        score_timer.start();
        mob_timer.start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob = self.mob_scene.instantiate_as::<RigidBody2D>();
        let mut mob_spawn_location = self
            .base
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);
        mob_spawn_location.set_progress(progress as f32);

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        mob.set_position(mob_spawn_location.get_position());

        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        mob.rotate(direction);

        let velocity = Vector2::new(rng.gen_range(150.0..250.0), 0.0);
        mob.set_linear_velocity(velocity.rotated(direction));

        self.base.add_child(mob.share().upcast());
    }

    #[func]
    fn game_over(&mut self) {
        let mut score_timer = self.base.get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base.get_node_as::<Timer>("MobTimer");

        score_timer.stop();
        mob_timer.stop();
    }

    #[func]
    fn new_game(&mut self) {
        self.score = 0;

        let marker = self.base.get_node_as::<Marker2D>("StartPosition");
        let mut player = self.base.get_node_as::<Player>("Player");
        player.bind_mut().start(marker.get_position());
        self.base.get_node_as::<Timer>("StartTimer").start();
    }
}

#[godot_api]
impl NodeVirtual for Game {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            score: 0,
            mob_scene: PackedScene::new(),
        }
    }

    fn ready(&mut self) {
        self.new_game();
    }
}
