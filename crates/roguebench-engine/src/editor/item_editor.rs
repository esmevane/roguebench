//! Item editor UI panel.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use roguebench_core::items::{Effect, ItemDefinition, ItemId, ItemType};

use crate::items::ItemRegistry;

/// Plugin for the item editor UI.
pub struct ItemEditorPlugin;

impl Plugin for ItemEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemEditorState>()
            .add_systems(Update, (toggle_editor_system, item_editor_ui).chain());
    }
}

/// State for the item editor UI.
#[derive(Resource)]
pub struct ItemEditorState {
    /// Whether the editor window is open.
    pub open: bool,
    /// Currently selected item ID for editing.
    pub selected_item: Option<String>,
    /// Form state for creating/editing items.
    pub form: ItemForm,
    /// Whether we're creating a new item vs editing existing.
    pub creating_new: bool,
    /// Pending delete confirmation.
    pub delete_confirm: Option<String>,
    /// Validation errors.
    pub errors: Vec<String>,
}

impl Default for ItemEditorState {
    fn default() -> Self {
        Self {
            open: true, // Start open for demo purposes
            selected_item: None,
            form: ItemForm::default(),
            creating_new: false,
            delete_confirm: None,
            errors: Vec::new(),
        }
    }
}

/// Form data for item editing.
#[derive(Clone)]
pub struct ItemForm {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub description: String,
    pub stackable: bool,
    pub max_stack: u32,
    pub value: u32,
    pub effects: Vec<EffectForm>,
}

impl Default for ItemForm {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            item_type: ItemType::Misc,
            description: String::new(),
            stackable: false,
            max_stack: 1,
            value: 0,
            effects: Vec::new(),
        }
    }
}

/// Form data for an effect.
#[derive(Clone)]
pub struct EffectForm {
    pub effect_type: EffectType,
    pub stat: String,
    pub amount: i32,
    pub status: String,
    pub duration: f32,
    pub event: String,
}

impl Default for EffectForm {
    fn default() -> Self {
        Self {
            effect_type: EffectType::ModifyStat,
            stat: "health".to_string(),
            amount: 10,
            status: String::new(),
            duration: 0.0,
            event: String::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EffectType {
    #[default]
    ModifyStat,
    ApplyStatus,
    TriggerEvent,
}

impl ItemForm {
    /// Create form from an existing item definition.
    pub fn from_definition(item: &ItemDefinition) -> Self {
        Self {
            id: item.id.0.clone(),
            name: item.name.clone(),
            item_type: item.item_type,
            description: item.description.clone().unwrap_or_default(),
            stackable: item.stackable,
            max_stack: item.max_stack.unwrap_or(1),
            value: item.value,
            effects: item.effects.iter().map(EffectForm::from_effect).collect(),
        }
    }

    /// Convert form to item definition.
    pub fn to_definition(&self) -> ItemDefinition {
        let mut item = ItemDefinition::new(&self.id, &self.name, self.item_type);

        if !self.description.is_empty() {
            item.description = Some(self.description.clone());
        }

        item.stackable = self.stackable;
        if self.stackable {
            item.max_stack = Some(self.max_stack);
        }
        item.value = self.value;

        for effect_form in &self.effects {
            if let Some(effect) = effect_form.to_effect() {
                item.effects.push(effect);
            }
        }

        item
    }

    /// Validate the form and return errors.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.id.is_empty() {
            errors.push("ID is required".to_string());
        } else if !self.id.chars().all(|c| c.is_alphanumeric() || c == '_') {
            errors.push("ID must be alphanumeric with underscores only".to_string());
        }

        if self.name.is_empty() {
            errors.push("Name is required".to_string());
        }

        errors
    }
}

impl EffectForm {
    fn from_effect(effect: &Effect) -> Self {
        match effect {
            Effect::ModifyStat { stat, amount } => Self {
                effect_type: EffectType::ModifyStat,
                stat: stat.clone(),
                amount: *amount,
                ..Default::default()
            },
            Effect::ApplyStatus { status, duration_secs } => Self {
                effect_type: EffectType::ApplyStatus,
                status: status.clone(),
                duration: *duration_secs,
                ..Default::default()
            },
            Effect::TriggerEvent { event } => Self {
                effect_type: EffectType::TriggerEvent,
                event: event.clone(),
                ..Default::default()
            },
        }
    }

    fn to_effect(&self) -> Option<Effect> {
        match self.effect_type {
            EffectType::ModifyStat if !self.stat.is_empty() => {
                Some(Effect::ModifyStat {
                    stat: self.stat.clone(),
                    amount: self.amount,
                })
            }
            EffectType::ApplyStatus if !self.status.is_empty() => {
                Some(Effect::ApplyStatus {
                    status: self.status.clone(),
                    duration_secs: self.duration,
                })
            }
            EffectType::TriggerEvent if !self.event.is_empty() => {
                Some(Effect::TriggerEvent {
                    event: self.event.clone(),
                })
            }
            _ => None,
        }
    }
}

/// System to toggle the editor window with F1.
fn toggle_editor_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ItemEditorState>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        state.open = !state.open;
    }
}

/// Main UI system for the item editor.
fn item_editor_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<ItemEditorState>,
    mut registry: ResMut<ItemRegistry>,
) {
    if !state.open {
        return;
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("Item Editor")
        .default_width(600.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left panel: item list
                ui.vertical(|ui| {
                    ui.set_min_width(150.0);
                    ui.heading("Items");

                    if ui.button("+ New Item").clicked() {
                        state.creating_new = true;
                        state.selected_item = None;
                        state.form = ItemForm::default();
                        state.errors.clear();
                    }

                    ui.separator();

                    egui::ScrollArea::vertical()
                        .max_height(400.0)
                        .show(ui, |ui| {
                            let items: Vec<_> = registry.all().collect();
                            for item in items {
                                let selected = state.selected_item.as_ref() == Some(&item.id.0);
                                if ui.selectable_label(selected, &item.name).clicked() {
                                    state.selected_item = Some(item.id.0.clone());
                                    state.form = ItemForm::from_definition(item);
                                    state.creating_new = false;
                                    state.errors.clear();
                                    state.delete_confirm = None;
                                }
                            }
                        });
                });

                ui.separator();

                // Right panel: form
                ui.vertical(|ui| {
                    ui.set_min_width(400.0);

                    if state.creating_new || state.selected_item.is_some() {
                        let title = if state.creating_new {
                            "New Item"
                        } else {
                            "Edit Item"
                        };
                        ui.heading(title);
                        ui.separator();

                        // Show validation errors
                        if !state.errors.is_empty() {
                            ui.colored_label(egui::Color32::RED, "Errors:");
                            for error in &state.errors {
                                ui.colored_label(egui::Color32::RED, format!("  • {}", error));
                            }
                            ui.separator();
                        }

                        // ID field (only editable for new items)
                        ui.horizontal(|ui| {
                            ui.label("ID:");
                            ui.add_enabled(
                                state.creating_new,
                                egui::TextEdit::singleline(&mut state.form.id)
                                    .hint_text("unique_identifier"),
                            );
                        });

                        // Name field
                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut state.form.name);
                        });

                        // Type dropdown
                        ui.horizontal(|ui| {
                            ui.label("Type:");
                            egui::ComboBox::from_id_salt("item_type")
                                .selected_text(format!("{:?}", state.form.item_type))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut state.form.item_type,
                                        ItemType::Consumable,
                                        "Consumable",
                                    );
                                    ui.selectable_value(
                                        &mut state.form.item_type,
                                        ItemType::Equipment,
                                        "Equipment",
                                    );
                                    ui.selectable_value(
                                        &mut state.form.item_type,
                                        ItemType::Key,
                                        "Key",
                                    );
                                    ui.selectable_value(
                                        &mut state.form.item_type,
                                        ItemType::Currency,
                                        "Currency",
                                    );
                                    ui.selectable_value(
                                        &mut state.form.item_type,
                                        ItemType::Misc,
                                        "Misc",
                                    );
                                });
                        });

                        // Description
                        ui.label("Description:");
                        ui.text_edit_multiline(&mut state.form.description);

                        // Stackable
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut state.form.stackable, "Stackable");
                            if state.form.stackable {
                                ui.label("Max:");
                                ui.add(egui::DragValue::new(&mut state.form.max_stack).range(1..=9999));
                            }
                        });

                        // Value
                        ui.horizontal(|ui| {
                            ui.label("Value:");
                            ui.add(egui::DragValue::new(&mut state.form.value).range(0..=999999));
                        });

                        ui.separator();

                        // Effects section
                        ui.collapsing("Effects", |ui| {
                            let mut to_remove = None;
                            for (i, effect) in state.form.effects.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    egui::ComboBox::from_id_salt(format!("effect_type_{}", i))
                                        .selected_text(match effect.effect_type {
                                            EffectType::ModifyStat => "Modify Stat",
                                            EffectType::ApplyStatus => "Apply Status",
                                            EffectType::TriggerEvent => "Trigger Event",
                                        })
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut effect.effect_type,
                                                EffectType::ModifyStat,
                                                "Modify Stat",
                                            );
                                            ui.selectable_value(
                                                &mut effect.effect_type,
                                                EffectType::ApplyStatus,
                                                "Apply Status",
                                            );
                                            ui.selectable_value(
                                                &mut effect.effect_type,
                                                EffectType::TriggerEvent,
                                                "Trigger Event",
                                            );
                                        });

                                    match effect.effect_type {
                                        EffectType::ModifyStat => {
                                            ui.text_edit_singleline(&mut effect.stat);
                                            ui.add(egui::DragValue::new(&mut effect.amount));
                                        }
                                        EffectType::ApplyStatus => {
                                            ui.text_edit_singleline(&mut effect.status);
                                            ui.label("for");
                                            ui.add(egui::DragValue::new(&mut effect.duration).suffix("s"));
                                        }
                                        EffectType::TriggerEvent => {
                                            ui.text_edit_singleline(&mut effect.event);
                                        }
                                    }

                                    if ui.button("×").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            }
                            if let Some(i) = to_remove {
                                state.form.effects.remove(i);
                            }

                            if ui.button("+ Add Effect").clicked() {
                                state.form.effects.push(EffectForm::default());
                            }
                        });

                        ui.separator();

                        // Action buttons
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                let errors = state.form.validate();
                                if errors.is_empty() {
                                    let item = state.form.to_definition();
                                    registry.insert(item);
                                    state.selected_item = Some(state.form.id.clone());
                                    state.creating_new = false;
                                    state.errors.clear();
                                } else {
                                    state.errors = errors;
                                }
                            }

                            if ui.button("Cancel").clicked() {
                                if state.creating_new {
                                    state.creating_new = false;
                                    state.form = ItemForm::default();
                                } else if let Some(id) = &state.selected_item
                                    && let Some(item) = registry.get(&ItemId::new(id))
                                {
                                    state.form = ItemForm::from_definition(item);
                                }
                                state.errors.clear();
                            }

                            // Delete button (only for existing items)
                            if !state.creating_new {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if state.delete_confirm.is_some() {
                                        ui.colored_label(egui::Color32::RED, "Confirm?");
                                        if ui.button("Yes").clicked() {
                                            state.selected_item = None;
                                            state.form = ItemForm::default();
                                            state.delete_confirm = None;
                                        }
                                        if ui.button("No").clicked() {
                                            state.delete_confirm = None;
                                        }
                                    } else if ui.button("Delete").clicked() {
                                        state.delete_confirm = state.selected_item.clone();
                                    }
                                });
                            }
                        });
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("Select an item or create a new one");
                        });
                    }
                });
            });

            // Close button in title bar area
            ui.horizontal(|ui| {
                ui.label("Press F1 to toggle");
            });
        });
}
