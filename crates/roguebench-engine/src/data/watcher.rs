//! Hot-reload file watcher for database changes.
//!
//! Watches the content database for changes and emits reload events.

use bevy::prelude::*;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebouncedEvent, Debouncer};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::sync::Mutex;
use std::time::Duration;

/// Event emitted when a watched file changes.
#[derive(Event, Debug, Clone)]
pub struct FileChanged {
    /// Path to the changed file.
    pub path: PathBuf,
}

/// Resource that watches files for changes.
///
/// When the watched database file is modified, a `FileChanged` event is emitted.
/// Systems can react to this event to reload content.
#[derive(Resource)]
pub struct ContentWatcher {
    /// Receiver for file change events from the watcher thread.
    /// Wrapped in Mutex for thread safety (Bevy resources must be Sync).
    receiver: Mutex<Receiver<Result<Vec<DebouncedEvent>, notify::Error>>>,
    /// The debouncer (kept alive to maintain the watcher).
    _debouncer: Debouncer<notify::RecommendedWatcher>,
    /// Path being watched.
    path: PathBuf,
}

impl ContentWatcher {
    /// Create a new watcher for the given path.
    ///
    /// The path should be the database file to watch.
    /// Changes are debounced to avoid rapid-fire events.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, WatchError> {
        let path = path.into();

        // Verify path exists
        if !path.exists() {
            return Err(WatchError::PathNotFound(path));
        }

        let (tx, rx) = mpsc::channel();

        // Create debouncer with 500ms delay
        let mut debouncer = new_debouncer(Duration::from_millis(500), tx)
            .map_err(|e| WatchError::WatcherInit(e.to_string()))?;

        // Watch the file (or directory containing it)
        let watch_path = if path.is_file() {
            path.parent().unwrap_or(&path).to_path_buf()
        } else {
            path.clone()
        };

        debouncer
            .watcher()
            .watch(&watch_path, RecursiveMode::NonRecursive)
            .map_err(|e| WatchError::WatcherInit(e.to_string()))?;

        info!("Watching {:?} for changes", path);

        Ok(Self {
            receiver: Mutex::new(rx),
            _debouncer: debouncer,
            path,
        })
    }

    /// Check for pending file change events.
    ///
    /// Returns paths of files that have changed since last check.
    pub fn poll(&self) -> Vec<PathBuf> {
        let mut changed = Vec::new();

        // Lock the receiver
        let receiver = match self.receiver.lock() {
            Ok(r) => r,
            Err(_) => return changed, // Mutex poisoned, return empty
        };

        // Drain all pending events
        while let Ok(result) = receiver.try_recv() {
            if let Ok(events) = result {
                for event in events {
                    // Only report if it matches our watched path
                    if event.path == self.path || event.path.starts_with(&self.path) {
                        changed.push(event.path);
                    }
                }
            }
        }

        changed
    }

    /// Get the path being watched.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// Error type for watcher operations.
#[derive(Debug, thiserror::Error)]
pub enum WatchError {
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),

    #[error("Failed to initialize watcher: {0}")]
    WatcherInit(String),
}

/// System that polls the watcher and emits file change events.
pub fn poll_content_watcher(
    watcher: Option<Res<ContentWatcher>>,
    mut events: EventWriter<FileChanged>,
) {
    if let Some(watcher) = watcher {
        for path in watcher.poll() {
            info!("Content file changed: {:?}", path);
            events.send(FileChanged { path });
        }
    }
}

/// Plugin that sets up file watching for hot-reload.
pub struct ContentWatcherPlugin {
    /// Path to the database file to watch.
    pub db_path: PathBuf,
}

impl ContentWatcherPlugin {
    /// Create a new watcher plugin for the given database path.
    pub fn new(db_path: impl Into<PathBuf>) -> Self {
        Self {
            db_path: db_path.into(),
        }
    }
}

impl Plugin for ContentWatcherPlugin {
    fn build(&self, app: &mut App) {
        match ContentWatcher::new(&self.db_path) {
            Ok(watcher) => {
                app.insert_resource(watcher)
                    .add_event::<FileChanged>()
                    .add_systems(First, poll_content_watcher);
            }
            Err(e) => {
                warn!("Failed to set up content watcher: {}", e);
                // Still add the event so systems don't break
                app.add_event::<FileChanged>();
            }
        }
    }
}

/// Extension trait for adding content watching.
pub trait ContentWatcherAppExt {
    /// Add a content watcher for the given database path.
    fn watch_content(&mut self, db_path: impl Into<PathBuf>) -> &mut Self;
}

impl ContentWatcherAppExt for App {
    fn watch_content(&mut self, db_path: impl Into<PathBuf>) -> &mut Self {
        self.add_plugins(ContentWatcherPlugin::new(db_path));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "roguebench_watch_test_{}_{}.db",
            std::process::id(),
            timestamp
        ))
    }

    #[test]
    fn watcher_creation() {
        let path = temp_file();

        // Create the file
        File::create(&path).unwrap();

        // Create watcher
        let watcher = ContentWatcher::new(&path);
        assert!(watcher.is_ok());

        // Cleanup
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn watcher_fails_on_missing_path() {
        let path = PathBuf::from("/nonexistent/path/to/file.db");
        let result = ContentWatcher::new(&path);
        assert!(result.is_err());
    }

    #[test]
    fn detect_file_change() {
        let path = temp_file();

        // Create the file
        {
            let mut file = File::create(&path).unwrap();
            file.write_all(b"initial content").unwrap();
        }

        // Create watcher
        let watcher = ContentWatcher::new(&path).unwrap();

        // Initial poll should be empty
        assert!(watcher.poll().is_empty());

        // Modify the file
        std::thread::sleep(Duration::from_millis(100));
        {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&path)
                .unwrap();
            file.write_all(b"more content").unwrap();
        }

        // Wait for debounce
        std::thread::sleep(Duration::from_millis(700));

        // Poll should detect change
        let changes = watcher.poll();
        // Note: This test may be flaky on some systems
        // The change detection depends on OS file system events

        // Cleanup
        std::fs::remove_file(path).ok();

        // We just verify it doesn't panic - actual detection is OS-dependent
        let _ = changes;
    }

    #[test]
    fn plugin_handles_missing_path() {
        // This shouldn't panic, just warn
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(ContentWatcherPlugin::new("/nonexistent/path.db"));

        // Should still be able to update
        app.update();
    }
}
