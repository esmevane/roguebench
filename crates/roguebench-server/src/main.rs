//! Roguebench game server.

use bevy::prelude::*;
use roguebench_engine::RoguebenchPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RoguebenchPlugin)
        .run();
}
