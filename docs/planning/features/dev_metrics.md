# Dev Metrics

Runtime performance metrics and diagnostics panel for development.

## Core Logic

**Concept**

- Track performance metrics
- Display diagnostic information
- Development builds only
- Non-intrusive overlay

**Metrics Tracked**

| Metric | Type | Description |
|--------|------|-------------|
| FPS | float | Frames per second |
| Frame time | ms | Time per frame |
| Entity count | int | Active entities |
| Draw calls | int | Render batches |
| Memory | MB | Heap usage |
| Network RTT | ms | Round-trip time |
| Packet loss | % | Network reliability |

**System Metrics**

| System | Metrics |
|--------|---------|
| Physics | Bodies, collisions/frame |
| Particles | Active particles |
| Audio | Playing sources |
| Scripts | Execution time |
| AI | Active agents, updates/frame |

**Display Modes**

| Mode | Content |
|------|---------|
| Minimal | FPS only |
| Standard | FPS, frame time, entities |
| Detailed | All metrics |
| Graph | Historical graph view |

**Operations**

`toggle_metrics()` - Show/hide panel

- Cycle through modes

`record_metric(name, value)` - Track custom

- Systems report their metrics

`get_metric(name)` - Query value

- For other systems

`export_metrics()` - Dump to file

- For analysis

**Invariants**

- Minimal performance impact
- Dev builds only (cfg)
- Metrics accurate
- History preserved for graphs

**Design Notes**

- Panel position configurable
- Color coding for warnings
- Threshold alerts optional

---

## Bevy Integration

**Resources**

- MetricsState { mode, visible }
- MetricsData { values: HashMap<String, MetricValue> }
- MetricsHistory { frames: VecDeque<MetricsSnapshot> }

**Conditional Compilation**

```rust
#[cfg(debug_assertions)]
app.add_plugins(MetricsPlugin);
```

**Systems**

- Collect metrics each frame
- Update display
- Maintain history
- Handle toggle input

**UI**

- Overlay panel
- Text readout
- Optional graphs
- Warning highlights

**Integration Points**

- Each system can register metrics
- Scripting can add custom metrics
- Network exposes its metrics

**Scripting Compatibility**

- Metrics readable from scripts
- Custom metrics registerable
- For debugging script performance

*See: debug_visualization.md, dev_console.md*
