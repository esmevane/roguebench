# Currency

Framework for designer-defined currency types including trade currencies, crafting reagents, and quest items.

## Core Logic

**Concept**

- Currency is any countable resource
- Types defined by design, not code
- Supports multiple distinct currencies
- Some persist across death, some don't

**Currency Properties**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| name | string | Display name |
| icon | asset | Visual representation |
| max_stack | int | Maximum amount (optional) |
| persists | bool | Survives death |
| tradeable | bool | Can be traded |

**Categories**

| Category | Examples | Persistence |
|----------|----------|-------------|
| Trade | Gold, gems | Persists |
| Reagent | Ore, herbs | Design choice |
| Quest | Keys, tokens | Design choice |
| Temporary | Run-specific | Cleared on death |

**Operations**

`add(currency_id, amount)` - Increase balance

- Add to current amount
- Cap at max if defined
- Return overflow if any

`remove(currency_id, amount)` - Decrease balance

- Subtract from current
- Fail if insufficient
- Return success/failure

`get(currency_id)` - Query balance

- Return current amount

`has(currency_id, amount)` - Check availability

- Return if sufficient

`transfer(from, to, currency_id, amount)` - Move between entities

- Remove from source
- Add to destination
- Atomic operation

**Invariants**

- Balance never negative
- Operations atomic
- Persistence respected on death
- Unknown currency types handled gracefully

**Design Notes**

- Specific currencies left to design
- Persistence rules left to design
- Earn/spend rates left to design

---

## Bevy Integration

**Data**

- CurrencyDefinition { id, name, icon, max_stack, persists, tradeable }
- CurrencyWallet { balances: HashMap<CurrencyId, u64> }

**Messages/Commands**

- AddCurrency { entity, currency_id, amount }
- RemoveCurrency { entity, currency_id, amount }
- TransferCurrency { from, to, currency_id, amount }

**Events**

- CurrencyChanged { entity, currency_id, old, new }

**Persistence**

- Wallet saved with entity
- Non-persistent currencies cleared on death

**Scripting Compatibility**

- Add/remove/transfer as commands
- Balances readable
- Events hookable for reactions

*See: architecture/scripting.md, architecture/data.md*
