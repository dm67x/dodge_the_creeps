use godot::engine::{Button, CanvasLayer, CanvasLayerVirtual, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = CanvasLayer)]
pub struct Hud {
    score_label: Option<Gd<Label>>,
    message_label: Option<Gd<Label>>,
    start_message_timer: Option<Gd<Timer>>,
    get_ready_message_timer: Option<Gd<Timer>>,
    start_button: Option<Gd<Button>>,
    start_button_timer: Option<Gd<Timer>>,
    #[base]
    base: Base<CanvasLayer>,
}

#[godot_api]
impl Hud {
    #[signal]
    fn start_game();

    #[func]
    pub fn show_get_ready(&mut self) {
        let Self {
            message_label,
            get_ready_message_timer,
            ..
        } = self;
        let message_label = message_label.as_mut().unwrap();
        let get_ready_message_timer = get_ready_message_timer.as_mut().unwrap();
        message_label.set_text("Get Ready".into());
        message_label.show();
        get_ready_message_timer.start();
    }

    #[func]
    pub fn show_game_over(&mut self) {
        let Self {
            message_label,
            start_message_timer,
            ..
        } = self;
        let message_label = message_label.as_mut().unwrap();
        let start_message_timer = start_message_timer.as_mut().unwrap();
        message_label.set_text("Game Over".into());
        message_label.show();
        start_message_timer.start();
    }

    #[func]
    pub fn update_score(&mut self, score: u32) {
        let Self { score_label, .. } = self;
        let score_label = score_label.as_mut().unwrap();
        score_label.set_text(score.to_string().into());
    }

    #[func]
    fn on_start_button_pressed(&mut self) {
        let Self {
            base,
            start_button_timer,
            start_button,
            ..
        } = self;
        let start_button_timer = start_button_timer.as_mut().unwrap();
        let start_button = start_button.as_mut().unwrap();
        start_button_timer.stop();
        start_button.hide();
        base.emit_signal("start_game".into(), &[]);
    }

    #[func]
    fn on_start_message_timer_timeout(&mut self) {
        let Self {
            message_label,
            start_button_timer,
            ..
        } = self;
        let message_label = message_label.as_mut().unwrap();
        let start_button_timer = start_button_timer.as_mut().unwrap();
        message_label.set_text("Dodge the Creeps".into());
        message_label.show();
        start_button_timer.start();
    }

    #[func]
    fn on_get_ready_message_timer_timeout(&mut self) {
        let Self { message_label, .. } = self;
        let message_label = message_label.as_mut().unwrap();
        message_label.hide();
    }

    #[func]
    fn on_start_button_timer_timeout(&mut self) {
        let Self { start_button, .. } = self;
        let start_button = start_button.as_mut().unwrap();
        start_button.show();
    }
}

#[godot_api]
impl CanvasLayerVirtual for Hud {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            score_label: None,
            message_label: None,
            start_message_timer: None,
            get_ready_message_timer: None,
            start_button: None,
            start_button_timer: None,
            base,
        }
    }

    fn ready(&mut self) {
        let Self {
            score_label,
            message_label,
            start_message_timer,
            get_ready_message_timer,
            start_button,
            start_button_timer,
            base,
        } = self;
        score_label.replace(base.get_node_as::<Label>("ScoreLabel"));
        message_label.replace(base.get_node_as::<Label>("MessageLabel"));
        start_message_timer.replace(base.get_node_as::<Timer>("StartMessageTimer"));
        get_ready_message_timer.replace(base.get_node_as::<Timer>("GetReadyMessageTimer"));
        start_button.replace(base.get_node_as::<Button>("StartButton"));
        start_button_timer.replace(base.get_node_as::<Timer>("StartButtonTimer"));
    }
}
