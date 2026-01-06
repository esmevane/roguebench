# Aptitudes

Cascade-style progression trees unlocked via Insight investment. Permanent character advancement.

## Core Logic

**Concept**

- Tree/graph of unlockable nodes
- Nodes unlocked by investing Insight
- Prerequisites form cascading dependencies
- Represents character build/specialization

**Node Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| name | string | Display name |
| cost | int | Insight required |
| prerequisites | list | Required unlocks |
| effects | list | Benefits when unlocked |

**Tree Structure**

- Root nodes have no prerequisites
- Child nodes require parent(s)
- Multiple trees possible (combat, utility, etc.)
- Designer-defined structure

**Operations**

`can_unlock(node_id)` - Check availability

- Has enough Insight
- Prerequisites met

`unlock(node_id)` - Invest in node

- Spend Insight
- Mark as unlocked
- Apply effects

`lock(node_id)` - Respec node

- Recover Insight
- Remove effects
- May cascade to dependents

`get_unlocked()` - Query active nodes

- List of unlocked aptitudes

`get_available()` - Query unlockable

- Nodes where prerequisites met

**Invariants**

- Can't unlock without prerequisites
- Locking cascades to dependents
- Effects apply/remove cleanly
- Tree structure defined by design

**Design Notes**

- Specific trees left to design
- Node effects left to design
- Cost scaling left to design

---

## Bevy Integration

**Data**

- AptitudeTree { nodes: Vec<AptitudeNode> }
- AptitudeNode { id, name, cost, prereqs, effects }
- PlayerAptitudes { unlocked: HashSet<AptitudeId> }

**Messages/Commands**

- UnlockAptitude { node_id }
- LockAptitude { node_id }

**Events**

- AptitudeUnlocked { node_id }
- AptitudeLocked { node_id }

**Persistence**

- Unlocked set saved with player
- Tree definitions loaded as assets

**Scripting Compatibility**

- Unlock/lock exposed as commands
- Unlocked state readable
- Events hookable for reactions

*See: architecture/scripting.md, architecture/data.md*
