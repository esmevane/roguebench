# Editor Architecture

Constraints and patterns for the web-based content management system embedded in the server.

## Principles

1. **Web-first** - Browser-based UI, no desktop app
2. **Server-embedded** - Runs as part of game server, not separate
3. **CRUD-focused** - Create, read, update, delete content
4. **Minimal ambition** - No visual tile editors, drag-drop, or WYSIWYG
5. **Script-aware** - Manage scripts alongside data

## Scope

**In Scope:**
- Content definition CRUD (items, enemies, quests, etc.)
- Script file management
- Configuration editing
- Import/export for backup and migration
- Basic validation and preview
- Multi-user access (auth)

**Out of Scope:**
- Visual tile/map editors
- Sprite editors
- Drag-and-drop placement
- Real-time visual preview
- Asset pipeline (handled separately)

## Architecture

**Server Integration:**
```
Game Server
├── Game Runtime (Bevy)
├── Content Database
└── Editor Web Service
    ├── REST API
    ├── Web UI (static files)
    └── WebSocket (live updates)
```

**Data Flow:**
```
Editor UI → REST API → Database → Game Runtime (hot reload)
```

## API Surface

**Content Endpoints:**
- `GET/POST/PUT/DELETE /api/{content_type}`
- `GET /api/{content_type}/{id}`
- Content types: items, enemies, quests, dialogues, effects, etc.

**Script Endpoints:**
- `GET/POST/PUT/DELETE /api/scripts`
- `POST /api/scripts/{id}/validate`
- `POST /api/scripts/{id}/reload`

**Admin Endpoints:**
- `GET/POST /api/config`
- `POST /api/export`
- `POST /api/import`
- `GET /api/health`

## Web UI

**Pages:**
- Dashboard (overview, recent changes)
- Content lists (filterable, searchable)
- Content editor (form-based)
- Script editor (text with syntax highlighting)
- Config editor
- Import/Export

**Tech Stack:**
- **axum** for HTTP server (embedded in game server binary)
- Static HTML + vanilla JS (or htmx for interactivity)
- No frontend framework or complex build pipeline
- Served from embedded static files or disk

## Integration Points

**With Scripting:**
- Scripts stored as content
- Validate syntax before save
- Trigger hot reload on change

**With Data Architecture:**
- Same schemas as runtime
- Validation matches runtime rules
- IDs match runtime ID system

**With Persistence:**
- Editor changes = database changes
- Export produces loadable format
- Import validates before applying

**With Game Runtime:**
- Hot reload on content change
- Runtime queries same database
- No restart required for content

## Constraints for Features

Features with editable content should:
- Define their data schema (for editor forms)
- Specify validation rules
- Support hot reload
- Document which fields are editor-managed vs runtime-only
- Consider editor workflow (what does designer need to see/do?)

## Multi-Environment

**Development:**
- Editor always available
- No auth required (local)
- Full access

**Staging:**
- Editor available
- Auth required
- Test content changes

**Production:**
- Editor restricted or disabled
- Auth required if available
- Read-only or limited write
