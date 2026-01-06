# Item Definitions

Framework for designer-defined item types and their properties.

## Core Logic

**Concept**

- Items defined as data, not code
- Definitions specify all item properties
- Instances reference definitions
- Supports any item type

**Definition Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| name | string | Display name |
| description | string | Flavor text |
| icon | asset | Visual representation |
| category | string | Inventory category |
| rarity | enum | Rarity tier |
| stackable | bool | Can stack in inventory |
| max_stack | int | Stack limit |
| slot_types | list | Equipment compatibility |
| effects | list | Effects when equipped/used |
| value | int | Base trade value |
| tags | list | Arbitrary tags |

**Item Types**

| Type | Behavior |
|------|----------|
| Equipment | Equippable, grants effects |
| Consumable | Usable, consumed on use |
| Material | Crafting ingredient |
| Quest | Quest-related, often unique |
| Currency | Treated as currency |
| Key | Unlocks something |

**Operations**

`get_definition(item_id)` - Fetch definition

- Return full item definition
- Used for display, logic

`create_instance(item_id)` - Make instance

- Create item with definition reference
- May include instance-specific data

`get_by_tag(tag)` - Query items

- Return all items with tag
- For filtering, logic

**Instance Data**

- Some items have unique instance data
- Examples: durability, enchantments, quality
- Definition + instance = full item

**Invariants**

- All items have valid definition
- Unknown definitions handled gracefully
- Definitions immutable at runtime
- Instance data mutable

**Design Notes**

- Specific items left to design
- Categories left to design
- Effects left to design

---

## Bevy Integration

**Assets**

- ItemDefinition loaded as asset
- Hot-reloadable in development

**Data**

- ItemDefinition { all properties }
- ItemInstance { definition_id, instance_data }

**Resources**

- ItemRegistry { definitions: HashMap<ItemId, Handle<ItemDefinition>> }

**Loading**

- Definitions from RON/JSON files
- Or fetched from content server

**Scripting Compatibility**

- Definitions readable
- Can query by properties/tags
- Instance creation as command

*See: architecture/scripting.md, architecture/data.md*
