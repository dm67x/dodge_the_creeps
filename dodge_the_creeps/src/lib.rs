mod game;
mod hud;
mod mob;
mod player;

use godot::prelude::*;

struct DodgeTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheCreeps {}
