# Insight

Permanent investable resource pool for unlocking progression. Earned slowly, invested deliberately, recoverable.

## Core Logic

**Concept**

- Permanent currency earned across runs
- Small amounts gained per run
- Invested to unlock aptitudes/abilities
- Recoverable via respec at designated location

**State**

- Available pool (unspent)
- Invested total (in unlocks)
- Lifetime earned (tracking)

**Operations**

`earn(amount)` - Add to available pool

- Gained from run completion, milestones
- Small increments by design

`invest(amount, target)` - Spend on unlock

- Deduct from available
- Add to invested
- Grant unlock benefit

`recover(target)` - Respec unlock

- Return invested amount to available
- Remove unlock benefit
- Only at designated location

`get_available()` - Query spendable

- Available pool amount

`get_invested()` - Query committed

- Total in active unlocks

**Invariants**

- Available + Invested = Lifetime - Spent on consumables (if any)
- Investment requires sufficient available
- Recovery requires being at respec location
- Persists across runs and deaths

**Design Notes**

- Specific earn rates left to design
- Unlock costs left to design
- Respec location left to design

---

## Bevy Integration

**Resources**

- InsightPool { available, invested, lifetime }
- RespecLocation marker component

**Messages/Commands**

- EarnInsight { amount }
- InvestInsight { amount, target_id }
- RecoverInsight { target_id }

**Persistence**

- Saved with player progress
- Survives death/run reset

**Scripting Compatibility**

- Earn/invest/recover exposed as commands
- Pool values readable by scripts
- Events emitted on changes

*See: architecture/scripting.md, architecture/data.md*
