# Wave Function Collapse Generation

Constraint-based procedural generation using Wave Function Collapse algorithm for coherent terrain.

## Core Logic

**Concept**

- Grid starts with all tiles possible in each cell
- Pick most constrained cell (fewest options)
- Collapse to single tile choice
- Propagate constraints to neighbors
- Repeat until complete

**Algorithm Steps**

1. Initialize grid with all tiles possible
2. Select cell with minimum remaining values
3. Choose tile based on weighted probability
4. Remove incompatible options from neighbors
5. Repeat steps 2-4 until solved
6. Restart if contradiction (no valid options)

**Constraint Propagation**

- When tile placed, neighbors update
- Remove tiles that can't connect
- Cascade until stable
- Like solving Sudoku

**Heuristics**

| Heuristic | Purpose |
|-----------|---------|
| MinimumRemainingValue | Select most constrained cell |
| WeightedProbability | Choose tile by spawn weight |
| RandomSeed | Deterministic generation |

**Operations**

`generate(width, height, seed)` - Create terrain

- Initialize WFC grid
- Run collapse loop
- Return tile grid or retry on failure

`restart()` - Handle contradiction

- Clear grid state
- Begin fresh (no backtracking)
- Simpler than backtrack

**Invariants**

- All placed tiles satisfy constraints
- Generation eventually succeeds or times out
- Same seed produces same result
- No invalid tile adjacencies

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Max attempts | 10 | Retries before fail |
| Seed mode | Random | Or fixed for testing |

---

## Bevy Integration

**Libraries**

- wfc crate or custom implementation
- Handles algorithm internals

**Resources**

- WfcConfig { seed_mode, max_attempts }
- WfcTileSet { tiles, rules }

**Systems**

- Run generation on world init
- Convert WFC output to tilemap
- Spawn tile entities

**Integration**

```rust
let wfc = WfcGenerator::new(tileset);
let grid = wfc.generate(64, 64, seed)?;
for (x, y, tile_id) in grid.iter() {
    spawn_tile(commands, x, y, tile_id);
}
```

**Failure Handling**

- Retry with different seed
- Fall back to simpler generation
- Log failure for debugging
