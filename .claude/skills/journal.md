---
name: journal
description: Persistent context across sessions. Track decisions, assumptions, observations, and lessons.
---

# Journal Skill

Manage persistent journal for tracking decisions, observations, and learnings across sessions.

**Database location:** `.claude/journal.db`

## Commands

### init
Initialize the journal database. Run this once per project.

```
/journal:init
```

**Implementation:**
```bash
sqlite3 .claude/journal.db << 'EOF'
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kind TEXT NOT NULL CHECK(kind IN ('decision', 'assumption', 'observation', 'question', 'workflow', 'pattern', 'anti_pattern', 'blocker')),
    content TEXT NOT NULL,
    confidence REAL DEFAULT 1.0,
    tags TEXT DEFAULT '',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_entries_kind ON entries(kind);
CREATE INDEX IF NOT EXISTS idx_entries_created_at ON entries(created_at);
CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts USING fts5(content, tags, content='entries', content_rowid='id');
CREATE TRIGGER IF NOT EXISTS entries_ai AFTER INSERT ON entries BEGIN
    INSERT INTO entries_fts(rowid, content, tags) VALUES (new.id, new.content, new.tags);
END;
CREATE TRIGGER IF NOT EXISTS entries_ad AFTER DELETE ON entries BEGIN
    INSERT INTO entries_fts(entries_fts, rowid, content, tags) VALUES('delete', old.id, old.content, old.tags);
END;
CREATE TRIGGER IF NOT EXISTS entries_au AFTER UPDATE ON entries BEGIN
    INSERT INTO entries_fts(entries_fts, rowid, content, tags) VALUES('delete', old.id, old.content, old.tags);
    INSERT INTO entries_fts(rowid, content, tags) VALUES (new.id, new.content, new.tags);
END;
EOF
```

After running, confirm: "Journal initialized at .claude/journal.db"

---

### remember
Store a new journal entry.

```
/journal:remember <kind> "<content>" [--confidence N] [--tags "tag1,tag2"]
```

**Kinds:** decision, assumption, observation, question, workflow, pattern, anti_pattern, blocker

**Examples:**
- `/journal:remember decision "Chose SQLite over RON for content storage"`
- `/journal:remember assumption "Players have stable internet" --confidence 0.7`
- `/journal:remember observation "State machine tests are slower than expected" --tags "performance"`

**Implementation:**

Parse arguments:
- `$1` = kind (required)
- `$2` = content (required, quoted string)
- `--confidence N` = confidence value 0.0-1.0 (default 1.0)
- `--tags "x,y"` = comma-separated tags (default empty)

```bash
sqlite3 .claude/journal.db "INSERT INTO entries (kind, content, confidence, tags) VALUES ('KIND', 'CONTENT', CONFIDENCE, 'TAGS');"
```

After inserting, confirm with the entry ID and echo the content back.

---

### recall
Search and retrieve journal entries.

```
/journal:recall [query] [--kind X] [--tags X] [--recent] [--since DATE]
```

**Examples:**
- `/journal:recall` or `/journal:recall --recent` - Last 10 entries
- `/journal:recall --kind decision` - All decisions
- `/journal:recall "hot reload"` - Full-text search
- `/journal:recall --since 2024-01-01` - Entries since date

**Implementation:**

Base query for `--recent` or no args (last 10):
```bash
sqlite3 -header -column .claude/journal.db "SELECT id, kind, content, confidence, tags, datetime(created_at, 'localtime') as created FROM entries ORDER BY created_at DESC LIMIT 10;"
```

Filter by kind:
```bash
sqlite3 -header -column .claude/journal.db "SELECT id, kind, content, confidence, tags, datetime(created_at, 'localtime') as created FROM entries WHERE kind = 'KIND' ORDER BY created_at DESC;"
```

Full-text search:
```bash
sqlite3 -header -column .claude/journal.db "SELECT e.id, e.kind, e.content, e.confidence, e.tags, datetime(e.created_at, 'localtime') as created FROM entries e JOIN entries_fts fts ON e.id = fts.rowid WHERE entries_fts MATCH 'QUERY' ORDER BY e.created_at DESC;"
```

Filter by date:
```bash
sqlite3 -header -column .claude/journal.db "SELECT id, kind, content, confidence, tags, datetime(created_at, 'localtime') as created FROM entries WHERE created_at >= 'DATE' ORDER BY created_at DESC;"
```

Format output as a readable table for the user.

---

### review
Show entries needing attention.

```
/journal:review [--stale] [--contradictions]
```

**Implementation:**

Default (open questions and blockers):
```bash
sqlite3 -header -column .claude/journal.db "SELECT id, kind, content, confidence, tags, datetime(created_at, 'localtime') as created FROM entries WHERE kind IN ('question', 'blocker') ORDER BY created_at DESC;"
```

`--stale` (entries older than 7 days that are questions/assumptions):
```bash
sqlite3 -header -column .claude/journal.db "SELECT id, kind, content, confidence, tags, datetime(created_at, 'localtime') as created FROM entries WHERE kind IN ('question', 'assumption') AND created_at < datetime('now', '-7 days') ORDER BY created_at DESC;"
```

`--contradictions` (low confidence assumptions):
```bash
sqlite3 -header -column .claude/journal.db "SELECT id, kind, content, confidence, tags, datetime(created_at, 'localtime') as created FROM entries WHERE kind = 'assumption' AND confidence < 0.8 ORDER BY confidence ASC;"
```

Summarize findings and suggest actions.

---

### synthesize
Generate insights from accumulated entries.

```
/journal:synthesize [--kind X]
```

**Implementation:**

1. Query entries (all or filtered by kind):
```bash
sqlite3 -header -column .claude/journal.db "SELECT kind, content, tags FROM entries ORDER BY kind, created_at;"
```

2. Group entries by kind and analyze:
   - **Decisions:** List key choices made
   - **Assumptions:** Flag low-confidence ones needing validation
   - **Observations:** Look for patterns or recurring themes
   - **Questions:** Highlight unanswered ones
   - **Blockers:** List active blockers
   - **Patterns/Anti-patterns:** Summarize learnings

3. Generate a synthesis report with:
   - Summary statistics (count by kind)
   - Key themes across entries
   - Open items needing attention
   - Recommendations

---

### stats
Show journal statistics.

```
/journal:stats
```

**Implementation:**
```bash
sqlite3 -header -column .claude/journal.db "SELECT kind, COUNT(*) as count FROM entries GROUP BY kind ORDER BY count DESC;"
sqlite3 .claude/journal.db "SELECT COUNT(*) as total FROM entries;"
```

---

## Storage

Journal entries are stored in `.claude/journal.db` (SQLite).

**Schema:**
- `id` - Auto-incrementing primary key
- `kind` - Entry type (decision, assumption, observation, question, workflow, pattern, anti_pattern, blocker)
- `content` - The journal entry text
- `confidence` - Confidence level 0.0-1.0 (default 1.0)
- `tags` - Comma-separated tags
- `created_at` - Timestamp

**Indexes:** kind, created_at
**Full-text search:** FTS5 on content and tags

## Session Integration

**At session start:**
- Run `/journal:recall --recent` to load context
- Run `/journal:review` to surface open questions

**During work:**
- Remember decisions as they're made
- Remember observations as work progresses

**At session end:**
- Synthesize if enough entries accumulated
- Note open questions for next session
