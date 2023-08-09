use std::f32::consts::PI;

use godot::engine::{Marker2D, Node, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
use rand::Rng;

use crate::player::Player;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Game {
    score: u32,
    player: Option<Gd<Player>>,
    start_position: Option<Gd<Marker2D>>,
    mob_spawn_location: Option<Gd<PathFollow2D>>,
    mob_timer: Option<Gd<Timer>>,
    score_timer: Option<Gd<Timer>>,
    start_timer: Option<Gd<Timer>>,
    music: Option<Gd<AudioStreamPlayer>>,
    death_sound: Option<Gd<AudioStreamPlayer>>,
    #[export]
    mob_scene: Option<Gd<PackedScene>>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Game {
    #[func]
    fn game_over(&mut self) {
        let Self {
            score_timer,
            mob_timer,
            ..
        } = self;
        let score_timer = score_timer.as_mut().unwrap();
        let mob_timer = mob_timer.as_mut().unwrap();
        score_timer.stop();
        mob_timer.stop();
    }

    #[func]
    fn new_game(&mut self) {
        let Self {
            score,
            player,
            start_position,
            start_timer,
            ..
        } = self;
        *score = 0;
        let mut player = player.as_mut().map(|player| player.bind_mut()).unwrap();
        let start_position = start_position.as_mut().unwrap();
        let start_timer = start_timer.as_mut().unwrap();
        player.start(start_position.get_position());
        start_timer.start();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        let Self { score, .. } = self;
        *score += 1;
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        let Self {
            score_timer,
            mob_timer,
            ..
        } = self;
        let score_timer = score_timer.as_mut().unwrap();
        let mob_timer = mob_timer.as_mut().unwrap();
        mob_timer.start();
        score_timer.start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let Self {
            mob_scene,
            mob_spawn_location,
            base,
            ..
        } = self;
        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);
        let mob_scene = mob_scene.as_ref().unwrap();
        let mut mob = mob_scene.instantiate_as::<RigidBody2D>();
        let mob_spawn_location = mob_spawn_location.as_mut().unwrap();

        mob_spawn_location.set_progress(progress as f32);

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        mob.set_position(mob_spawn_location.get_position());

        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        mob.rotate(direction);

        let velocity = Vector2::new(rng.gen_range(150.0..250.0), 0.0);
        mob.set_linear_velocity(velocity.rotated(direction));

        base.add_child(mob.share().upcast());
    }
}

#[godot_api]
impl NodeVirtual for Game {
    fn init(base: Base<Node>) -> Self {
        Self {
            score: 0,
            player: None,
            start_position: None,
            mob_spawn_location: None,
            mob_timer: None,
            score_timer: None,
            start_timer: None,
            music: None,
            death_sound: None,
            mob_scene: None,
            base,
        }
    }

    fn ready(&mut self) {
        let Self {
            base,
            player,
            start_position,
            mob_spawn_location,
            mob_timer,
            score_timer,
            start_timer,
            ..
        } = self;
        player.replace(base.get_node_as::<Player>("Player"));
        start_position.replace(base.get_node_as::<Marker2D>("StartPosition"));
        mob_timer.replace(base.get_node_as::<Timer>("MobTimer"));
        score_timer.replace(base.get_node_as::<Timer>("ScoreTimer"));
        start_timer.replace(base.get_node_as::<Timer>("StartTimer"));
        mob_spawn_location.replace(base.get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation"));

        self.new_game();
    }
}
