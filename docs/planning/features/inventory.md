# Inventory

Unlimited categorized item storage for entities.

## Core Logic

**Concept**

- Entities have inventory of items
- No slot limits, organized by category
- Items can stack or be unique
- Simple list, no spatial management

**Item Entry**

| Property | Type | Description |
|----------|------|-------------|
| item_id | string | Item definition reference |
| quantity | int | Stack count |
| instance_data | option | Unique instance data |

**Categories**

- Designer-defined categories
- Items belong to one category
- UI organizes by category
- Examples: Weapons, Armor, Consumables, Materials, Quest

**Operations**

`add_item(item_id, quantity)` - Add to inventory

- Stack with existing if stackable
- Create new entry if not
- Return success/overflow

`remove_item(item_id, quantity)` - Remove from inventory

- Reduce stack or remove entry
- Fail if insufficient
- Return success/failure

`has_item(item_id, quantity)` - Check for item

- Return if sufficient quantity exists

`get_items(category)` - List items

- Return items in category
- Or all items if no category

`get_quantity(item_id)` - Query amount

- Return total quantity across stacks

`transfer_item(to, item_id, quantity)` - Move item

- Remove from self
- Add to target
- Atomic operation

**Invariants**

- Quantities always positive
- Unknown items handled gracefully
- Categories from item definition
- No duplicate entries for same item (stacked)

**Design Notes**

- Categories left to design
- Stacking rules per item definition
- UI presentation left to design

---

## Bevy Integration

**Components**

- Inventory { items: Vec<InventoryEntry> }
- InventoryEntry { item_id, quantity, instance_data }

**Messages/Commands**

- AddItem { entity, item_id, quantity }
- RemoveItem { entity, item_id, quantity }
- TransferItem { from, to, item_id, quantity }

**Events**

- ItemAdded { entity, item_id, quantity }
- ItemRemoved { entity, item_id, quantity }
- InventoryChanged { entity }

**UI**

- Category tabs or filters
- Item icons with quantities
- Context menu (use, drop, etc.)

**Scripting Compatibility**

- Add/remove/transfer as commands
- Inventory contents readable
- Events hookable

---

## Framework Dependencies

- `framework/command_bus.md` - Inventory operations as commands
- `framework/data_loading.md` - Item definitions from data
- `framework/event_hooks.md` - Item add/remove hooks
- `framework/ui_framework.md` - Inventory UI display

*See: architecture/scripting.md, architecture/data.md*
