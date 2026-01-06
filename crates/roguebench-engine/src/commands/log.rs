//! Command logging for replay and debugging.

use bevy::prelude::*;
use roguebench_core::commands::{Command, CommandId, CommandMeta};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// A single entry in the command log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry<C> {
    /// The command that was executed.
    pub command: C,
    /// Metadata from when the command was sent.
    pub meta: CommandMeta,
    /// Whether execution succeeded.
    pub succeeded: bool,
}

impl<C: Command> LogEntry<C> {
    /// Create a new log entry for a successful command.
    pub fn success(command: C, meta: CommandMeta) -> Self {
        Self {
            command,
            meta,
            succeeded: true,
        }
    }

    /// Create a new log entry for a failed command.
    pub fn failed(command: C, meta: CommandMeta) -> Self {
        Self {
            command,
            meta,
            succeeded: false,
        }
    }
}

/// In-memory command log for a specific command type.
///
/// Stores all commands that have been executed, in order. Can be
/// persisted to disk for replay and debugging.
#[derive(Resource)]
pub struct CommandLog<C: Command> {
    entries: Vec<LogEntry<C>>,
    /// Whether to auto-persist after each append.
    auto_persist: bool,
    /// File path for persistence (if enabled).
    persist_path: Option<String>,
}

impl<C: Command> Default for CommandLog<C> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            auto_persist: false,
            persist_path: None,
        }
    }
}

impl<C: Command> CommandLog<C> {
    /// Create a new empty command log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a command log that auto-persists to a file.
    pub fn with_persistence(path: impl Into<String>) -> Self {
        Self {
            entries: Vec::new(),
            auto_persist: true,
            persist_path: Some(path.into()),
        }
    }

    /// Append an entry to the log.
    pub fn append(&mut self, entry: LogEntry<C>) {
        self.entries.push(entry);

        if self.auto_persist
            && let Some(path) = &self.persist_path
            && let Ok(file) = OpenOptions::new().create(true).append(true).open(path)
        {
            let mut writer = BufWriter::new(file);
            if let Ok(json) = serde_json::to_string(self.entries.last().unwrap()) {
                let _ = writeln!(writer, "{}", json);
            }
        }
    }

    /// Log a successful command execution.
    pub fn log_success(&mut self, command: C, meta: CommandMeta) {
        self.append(LogEntry::success(command, meta));
    }

    /// Log a failed command execution.
    pub fn log_failure(&mut self, command: C, meta: CommandMeta) {
        self.append(LogEntry::failed(command, meta));
    }

    /// Get all entries.
    pub fn entries(&self) -> &[LogEntry<C>] {
        &self.entries
    }

    /// Get entry count.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get entry by command ID.
    pub fn get_by_id(&self, id: CommandId) -> Option<&LogEntry<C>> {
        self.entries.iter().find(|e| e.meta.id == id)
    }

    /// Iterate over entries in order.
    pub fn iter(&self) -> impl Iterator<Item = &LogEntry<C>> {
        self.entries.iter()
    }

    /// Iterate over successful commands only.
    pub fn successes(&self) -> impl Iterator<Item = &LogEntry<C>> {
        self.entries.iter().filter(|e| e.succeeded)
    }

    /// Iterate over failed commands only.
    pub fn failures(&self) -> impl Iterator<Item = &LogEntry<C>> {
        self.entries.iter().filter(|e| !e.succeeded)
    }

    /// Get entries within a frame range.
    pub fn in_frame_range(&self, start: u64, end: u64) -> impl Iterator<Item = &LogEntry<C>> {
        self.entries.iter().filter(move |e| {
            e.meta
                .frame
                .map(|f| f >= start && f <= end)
                .unwrap_or(false)
        })
    }

    /// Save all entries to a file (JSONL format).
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for entry in &self.entries {
            let json = serde_json::to_string(entry)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            writeln!(writer, "{}", json)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Load entries from a file (JSONL format).
    pub fn load_from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let entry: LogEntry<C> = serde_json::from_str(&line)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            entries.push(entry);
        }

        Ok(Self {
            entries,
            auto_persist: false,
            persist_path: None,
        })
    }
}

/// Iterator for replaying commands from a log.
///
/// This provides commands in order, ready to be re-sent through
/// the command bus.
pub struct ReplayIterator<'a, C: Command> {
    entries: std::slice::Iter<'a, LogEntry<C>>,
    filter_successes: bool,
}

impl<'a, C: Command> ReplayIterator<'a, C> {
    /// Create a replay iterator from a log.
    pub fn new(log: &'a CommandLog<C>) -> Self {
        Self {
            entries: log.entries.iter(),
            filter_successes: false,
        }
    }

    /// Only replay successful commands.
    pub fn successes_only(mut self) -> Self {
        self.filter_successes = true;
        self
    }
}

impl<'a, C: Command> Iterator for ReplayIterator<'a, C> {
    type Item = (C, CommandMeta);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.entries.next()?;
            if self.filter_successes && !entry.succeeded {
                continue;
            }
            return Some((entry.command.clone(), entry.meta.clone()));
        }
    }
}

impl<C: Command> CommandLog<C> {
    /// Create a replay iterator.
    pub fn replay(&self) -> ReplayIterator<'_, C> {
        ReplayIterator::new(self)
    }
}

/// Extension trait for registering command logs.
pub trait CommandLogAppExt {
    /// Register a command log for a command type.
    fn register_command_log<C: Command>(&mut self) -> &mut Self;
}

impl CommandLogAppExt for App {
    fn register_command_log<C: Command>(&mut self) -> &mut Self {
        self.init_resource::<CommandLog<C>>();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    struct TestCommand {
        value: i32,
    }

    impl Command for TestCommand {
        type Output = ();
        type Error = ();

        fn name() -> &'static str {
            "test"
        }
    }

    fn make_meta(id: u64) -> CommandMeta {
        CommandMeta::new(CommandId::new(id), 1000).with_frame(id)
    }

    #[test]
    fn log_append_and_iterate() {
        let mut log = CommandLog::<TestCommand>::new();

        log.log_success(TestCommand { value: 1 }, make_meta(1));
        log.log_success(TestCommand { value: 2 }, make_meta(2));
        log.log_failure(TestCommand { value: 3 }, make_meta(3));

        assert_eq!(log.len(), 3);

        let values: Vec<_> = log.iter().map(|e| e.command.value).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn filter_successes_and_failures() {
        let mut log = CommandLog::<TestCommand>::new();

        log.log_success(TestCommand { value: 1 }, make_meta(1));
        log.log_failure(TestCommand { value: 2 }, make_meta(2));
        log.log_success(TestCommand { value: 3 }, make_meta(3));

        let successes: Vec<_> = log.successes().map(|e| e.command.value).collect();
        assert_eq!(successes, vec![1, 3]);

        let failures: Vec<_> = log.failures().map(|e| e.command.value).collect();
        assert_eq!(failures, vec![2]);
    }

    #[test]
    fn get_by_id() {
        let mut log = CommandLog::<TestCommand>::new();

        log.log_success(TestCommand { value: 42 }, make_meta(100));

        let entry = log.get_by_id(CommandId::new(100)).unwrap();
        assert_eq!(entry.command.value, 42);

        assert!(log.get_by_id(CommandId::new(999)).is_none());
    }

    #[test]
    fn frame_range_filter() {
        let mut log = CommandLog::<TestCommand>::new();

        log.log_success(TestCommand { value: 1 }, make_meta(10));
        log.log_success(TestCommand { value: 2 }, make_meta(20));
        log.log_success(TestCommand { value: 3 }, make_meta(30));
        log.log_success(TestCommand { value: 4 }, make_meta(40));

        let in_range: Vec<_> = log.in_frame_range(15, 35).map(|e| e.command.value).collect();
        assert_eq!(in_range, vec![2, 3]);
    }

    #[test]
    fn replay_iterator() {
        let mut log = CommandLog::<TestCommand>::new();

        log.log_success(TestCommand { value: 1 }, make_meta(1));
        log.log_failure(TestCommand { value: 2 }, make_meta(2));
        log.log_success(TestCommand { value: 3 }, make_meta(3));

        // All commands
        let all: Vec<_> = log.replay().map(|(c, _)| c.value).collect();
        assert_eq!(all, vec![1, 2, 3]);

        // Only successes
        let successes: Vec<_> = log.replay().successes_only().map(|(c, _)| c.value).collect();
        assert_eq!(successes, vec![1, 3]);
    }

    #[test]
    fn save_and_load() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir()
            .join(format!("command_log_test_{}.jsonl", timestamp))
            .to_string_lossy()
            .to_string();

        // Create and save
        {
            let mut log = CommandLog::<TestCommand>::new();
            log.log_success(TestCommand { value: 1 }, make_meta(1));
            log.log_success(TestCommand { value: 2 }, make_meta(2));
            log.save_to_file(&path).unwrap();
        }

        // Load and verify
        {
            let log = CommandLog::<TestCommand>::load_from_file(&path).unwrap();
            assert_eq!(log.len(), 2);
            assert_eq!(log.entries()[0].command.value, 1);
            assert_eq!(log.entries()[1].command.value, 2);
        }

        // Cleanup
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn clear_log() {
        let mut log = CommandLog::<TestCommand>::new();
        log.log_success(TestCommand { value: 1 }, make_meta(1));
        log.log_success(TestCommand { value: 2 }, make_meta(2));

        assert!(!log.is_empty());
        log.clear();
        assert!(log.is_empty());
    }
}
