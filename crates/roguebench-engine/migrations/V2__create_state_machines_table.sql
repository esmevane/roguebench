-- State machine definitions for data-driven AI and behaviors
CREATE TABLE state_machines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    initial_state TEXT NOT NULL,
    data TEXT NOT NULL,  -- JSON blob with full definition
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX idx_state_machines_name ON state_machines(name);
