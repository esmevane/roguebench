# Loot Tables

Weighted roll system for randomized item drops and rewards.

## Core Logic

**Concept**

- Tables define possible drops
- Entries have weights (probability)
- Roll selects entry based on weights
- Supports nested tables, quantities, rarity

**Table Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Table identifier |
| entries | list | Possible drops |
| rolls | range | How many times to roll |
| guaranteed | list | Always dropped |

**Entry Properties**

| Property | Type | Description |
|----------|------|-------------|
| type | enum | Item, Currency, Table, Nothing |
| id | string | What to drop |
| weight | float | Relative probability |
| quantity | range | Amount range |
| conditions | list | Requirements to include |

**Operations**

`roll(table_id)` - Generate drops

- Evaluate entries (filter by conditions)
- Select based on weights
- Resolve quantities
- Recurse into nested tables
- Return drop list

`add_entry(table_id, entry)` - Modify table

- Runtime table modification
- For dynamic content

`get_probability(table_id, item_id)` - Calculate odds

- For UI display
- Account for weights and conditions

**Weight Calculation**

```
probability = entry_weight / sum(all_weights)
```

**Invariants**

- Weights must be positive
- Empty tables return nothing
- Conditions evaluated at roll time
- Nested tables fully resolved

**Design Notes**

- Specific tables left to design
- Drop rates left to design
- Rarity tiers left to design

---

## Bevy Integration

**Data**

- LootTable { id, entries, rolls, guaranteed }
- LootEntry { entry_type, id, weight, quantity_range, conditions }

**Operations**

```rust
fn roll_table(table_id: &str, rng: &mut Rng) -> Vec<LootDrop>;
```

**Messages/Commands**

- RollLootTable { table_id, recipient }
- DropLoot { position, drops }

**Events**

- LootRolled { table_id, drops }
- LootDropped { entity, drops }

**Use Cases**

- Enemy death → roll enemy's loot table
- Chest opened → roll chest's loot table
- Quest reward → roll reward table

**Scripting Compatibility**

- Roll exposed as command
- Tables readable/modifiable
- Events hookable

*See: architecture/scripting.md, architecture/data.md*
