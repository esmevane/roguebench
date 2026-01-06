# NPC System

Minimal framework for non-combat characters that interact with players and integrate with scripting.

## Core Logic

**Concept**

- Non-combat entities players interact with
- Scripting-driven behavior
- Light framework, design fills details
- Integrates with dialogue and story

**NPC Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | NPC identifier |
| name | string | Display name |
| sprite | asset | Visual representation |
| dialogue | string | Conversation to start |
| behavior | string | Script to run |
| persistent | bool | Exists across sessions |

**Interaction Types**

| Type | Behavior | Example |
|------|----------|---------|
| Talk | Start dialogue | Quest giver |
| Shop | Open shop | Merchant |
| Service | Provide service | Healer |
| Information | Show info | Sign, memorial |
| Script | Run custom script | Anything |

**Operations**

`spawn_npc(definition)` - Create NPC

- Spawn entity with NPC components
- Initialize behavior script

`interact(npc)` - Player interacts

- Trigger NPC's interaction type
- Run associated script

`despawn_npc(id)` - Remove NPC

- Clean up entity
- Persist state if needed

`get_npc_state(id)` - Query state

- Return NPC's current state
- For conditionals

**Minimal Behavior**

- Idle animation
- Face player on interact
- Run script on interaction
- State persistence (optional)

**Invariants**

- NPCs non-hostile by default
- Interaction uses interaction_system
- Scripts handle complex behavior
- State persists if marked

**Design Notes**

- Specific NPCs left to design
- Behavior scripts left to design
- Visual design left to art

---

## Bevy Integration

**Components**

- NPC { id, name, interaction_type }
- NPCBehavior { script_id }
- NPCState { data: HashMap } - arbitrary state

**Data**

- NPCDefinition { id, name, sprite, dialogue, behavior, ... }

**Messages/Commands**

- SpawnNPC { definition_id, position }
- DespawnNPC { npc_id }
- InteractWithNPC { npc_id }

**Events**

- NPCSpawned { npc_id }
- NPCInteracted { npc_id, player }
- NPCDespawned { npc_id }

**Systems**

- Handle NPC spawning
- Process interactions
- Run behavior scripts
- Manage state persistence

**Scripting Compatibility**

- NPC behavior is scripting primary use case
- All operations exposed to scripts
- State readable/writable by scripts
- Events hookable

*See: architecture/scripting.md, interaction_system.md, dialogue_system.md*
