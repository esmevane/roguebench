//! Roguebench game client.

use bevy::prelude::*;
use roguebench_engine::RoguebenchPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RoguebenchPlugin)
        .run();
}
