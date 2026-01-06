//! In-game editor UI for content authoring.
//!
//! Uses bevy_egui to provide form-based editing of game content.

pub mod item_editor;

pub use item_editor::ItemEditorPlugin;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

/// Main editor plugin that enables all editor UI.
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        // Add EguiPlugin if not already added
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_plugins(ItemEditorPlugin);
    }
}
