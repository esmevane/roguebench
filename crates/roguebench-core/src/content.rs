//! Content type trait for loadable game content

use crate::{Database, Result};
use std::hash::Hash;

/// Trait for content types that can be loaded from the database.
///
/// Each content type (enemies, items, rooms, etc.) implements this trait
/// to enable generic loading via ContentRegistry.
pub trait ContentType: Clone + Send + Sync + 'static {
    /// The ID type for this content (e.g., EnemyId, ItemId)
    type Id: Hash + Eq + Clone + Send + Sync;

    /// Get the ID of this content instance
    fn id(&self) -> &Self::Id;

    /// Load all instances of this content type from the database
    fn load_all(db: &Database) -> Result<Vec<Self>>;

    /// The name of this content type for logging/errors
    fn type_name() -> &'static str;
}
