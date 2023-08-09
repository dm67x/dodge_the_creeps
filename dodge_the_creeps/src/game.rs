use std::f32::consts::PI;

use godot::engine::{Marker2D, Node, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
use rand::Rng;

use crate::hud::Hud;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base = Node)]
pub struct Game {
    score: u32,
    hud: Option<Gd<Hud>>,
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
            hud,
            music,
            death_sound,
            ..
        } = self;
        let score_timer = score_timer.as_mut().unwrap();
        let mob_timer = mob_timer.as_mut().unwrap();

        score_timer.stop();
        mob_timer.stop();

        let mut hud = hud.as_mut().map(|hud| hud.bind_mut()).unwrap();
        hud.show_game_over();

        let music = music.as_mut().unwrap();
        music.stop();

        let death_sound = death_sound.as_mut().unwrap();
        death_sound.play();
    }

    #[func]
    fn new_game(&mut self) {
        let Self {
            score,
            player,
            start_position,
            start_timer,
            hud,
            base,
            music,
            ..
        } = self;
        *score = 0;

        let start_position = start_position.as_mut().unwrap();

        let mut player = player.as_mut().map(|player| player.bind_mut()).unwrap();
        player.start(start_position.get_position());

        let start_timer = start_timer.as_mut().unwrap();
        start_timer.start();

        let mut hud = hud.as_mut().map(|hud| hud.bind_mut()).unwrap();
        hud.update_score(*score);
        hud.show_get_ready();

        base.get_tree()
            .unwrap()
            .call_group("mobs".into(), "queue_free".into(), &[]);

        let music = music.as_mut().unwrap();
        music.play();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        let Self { score, hud, .. } = self;
        *score += 1;

        let mut hud = hud.as_mut().map(|hud| hud.bind_mut()).unwrap();
        hud.update_score(*score);
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
            hud: None,
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
            hud,
            player,
            start_position,
            mob_spawn_location,
            mob_timer,
            score_timer,
            start_timer,
            music,
            death_sound,
            ..
        } = self;
        player.replace(base.get_node_as::<Player>("Player"));
        hud.replace(base.get_node_as::<Hud>("Hud"));
        start_position.replace(base.get_node_as::<Marker2D>("StartPosition"));
        mob_timer.replace(base.get_node_as::<Timer>("MobTimer"));
        score_timer.replace(base.get_node_as::<Timer>("ScoreTimer"));
        start_timer.replace(base.get_node_as::<Timer>("StartTimer"));
        mob_spawn_location.replace(base.get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation"));
        music.replace(base.get_node_as::<AudioStreamPlayer>("Music"));
        death_sound.replace(base.get_node_as::<AudioStreamPlayer>("DeathSound"));
    }
}
