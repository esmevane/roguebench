//! ContentType implementations for core types.

use super::content::{ContentType, LoadError};
use super::Database;
use roguebench_core::items::{ItemDefinition, ItemId};

impl ContentType for ItemDefinition {
    type Id = ItemId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn load_all(db: &Database) -> Result<Vec<Self>, LoadError> {
        db.get_all_items().map_err(LoadError::from)
    }

    fn type_name() -> &'static str {
        "items"
    }
}
