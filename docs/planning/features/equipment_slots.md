# Equipment Slots

Slot system for equipping items that provide stat bonuses and abilities.

## Core Logic

**Concept**

- Entities have named equipment slots
- Items equip into matching slots
- Equipped items grant effects
- Unequipping returns to inventory

**Slot Types**

- Designer-defined slot names
- Items specify compatible slots
- Examples: Weapon, Offhand, Head, Body, Accessory

**Slot Properties**

| Property | Type | Description |
|----------|------|-------------|
| name | string | Slot identifier |
| item | option | Currently equipped item |
| locked | bool | Prevent changes |

**Item Compatibility**

- Items define which slots they fit
- Some items fit multiple slots
- Some slots accept multiple item types

**Operations**

`equip(slot, item_id)` - Put item in slot

- Remove from inventory
- Place in slot
- Apply item effects
- Return previous item to inventory

`unequip(slot)` - Remove from slot

- Remove item effects
- Return item to inventory
- Slot becomes empty

`get_equipped(slot)` - Query slot

- Return equipped item or none

`get_all_equipped()` - List equipment

- Return all slot-item pairs

`can_equip(slot, item_id)` - Check compatibility

- Item fits slot type
- Requirements met (level, stats)

**Invariants**

- One item per slot
- Equipping auto-unequips previous
- Effects applied/removed atomically
- Locked slots unchangeable

**Design Notes**

- Slot types left to design
- Item-slot compatibility left to design
- Requirements left to design

---

## Bevy Integration

**Components**

- Equipment { slots: HashMap<SlotName, Option<ItemId>> }
- EquipmentSlotDef { name, accepted_types }

**Messages/Commands**

- EquipItem { entity, slot, item_id }
- UnequipItem { entity, slot }
- SwapEquipment { entity, slot_a, slot_b }

**Events**

- ItemEquipped { entity, slot, item_id }
- ItemUnequipped { entity, slot, item_id }
- EquipmentChanged { entity }

**Effect Application**

- On equip: apply item's stat modifiers
- On unequip: remove stat modifiers
- Integrate with stat_system

**Scripting Compatibility**

- Equip/unequip as commands
- Equipment state readable
- Events hookable

*See: architecture/scripting.md, architecture/data.md*
