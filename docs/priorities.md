# Priorities

A decision framework for choosing what to work on next. This is not a task list—it's a set of reasoning tools.

---

## The Priority Stack

When choosing work, apply these filters in order. Earlier filters take precedence.

### 1. Unresolved Decisions Block Everything

If there's a TBD that blocks implementation, resolve it first.

**Ask:** "What decisions are we avoiding?"

**Signs of unresolved decisions:**
- Planning docs with "TBD" or "(choice needed)"
- Work proceeding with placeholder implementations
- Multiple valid approaches, none chosen
- Conversations that keep circling the same question

**What to do:**
1. Surface the decision explicitly
2. Document the options and tradeoffs
3. Make the decision (or escalate if you can't)
4. Record the decision and rationale

### 2. Missing Frameworks Block Features

If a feature needs infrastructure that doesn't exist, build the infrastructure first.

**Ask:** "What would this feature need to exist first?"

**Signs of missing frameworks:**
- Similar code patterns in multiple places
- Features that would need to invent their own version of something
- Tests that are hard to write because infrastructure is missing
- "Temporary" implementations that bypass the system

**What to do:**
1. Identify the framework need
2. Design the interface (minimal, not speculative)
3. Build the framework
4. Then build the feature on top of it

### 3. Vertical Slices Over Horizontal Layers

Completing one thing end-to-end is more valuable than partial progress on many things.

**Ask:** "Can a user do something new when this is done?"

**Signs of horizontal drift:**
- Many features partially implemented
- Infrastructure built but nothing uses it
- "We'll connect it later" appearing in plans
- Users can't test or use anything yet

**What to do:**
1. Pick one content type or capability
2. Build it completely: editor → data → runtime → persistence
3. Ship it (internally) and get feedback
4. Then start the next one

### 4. User Capability Over Technical Elegance

Work that enables authoring is more valuable than work that improves internals.

**Ask:** "Does this help someone create content?"

**Signs of misplaced priorities:**
- Refactoring code that works fine
- Building features users didn't ask for
- Optimizing before there's a performance problem
- "Nice to have" improvements over core needs

**What to do:**
1. Check the mission: does this advance authoring capability?
2. If not, ask: is this blocking something that does?
3. If not, it can wait

### 5. Smallest Testable Increment

The smallest change that can be verified is the right size.

**Ask:** "How would we know this works?"

**Signs of over-scoping:**
- Work items that take multiple sessions
- Changes that touch many files before any testing
- "I'll test it when it's all connected"
- Features that can't be demonstrated in isolation

**What to do:**
1. Break work into pieces that can be individually tested
2. Test each piece before moving to the next
3. Integrate tested pieces
4. Test the integration

---

## Anti-Patterns

### Building Features Before Frameworks

**Pattern:** Starting a feature, realizing it needs infrastructure, building ad-hoc infrastructure inside the feature.

**Result:** Duplicate implementations, inconsistent patterns, hard-to-test code.

**Alternative:** When you discover a framework need, stop feature work. Build the framework. Resume feature work.

### Horizontal Completeness

**Pattern:** Building all of one layer before connecting to other layers. "Let's finish all the editor UI, then do the backend, then do the runtime."

**Result:** Integration problems discovered late. Users can't test anything until everything is done.

**Alternative:** Build one vertical slice completely. Then another. Each slice is usable.

### Deferred Decisions Accumulating

**Pattern:** Leaving TBDs in place and working around them. "We'll figure out the scripting language later."

**Result:** Work built on assumptions that may not hold. Rework when decisions are finally made.

**Alternative:** Resolve decisions before building dependent work. If you can't decide, that's the work: gather information needed to decide.

### Polish Before Function

**Pattern:** Making things nice before they work. Refactoring code that's not tested. UI polish before core flows work.

**Result:** Polished incomplete things. Sunk cost in code that might need to change.

**Alternative:** Working ugly over polished incomplete. Make it work, then make it good, then make it fast.

### Speculative Frameworks

**Pattern:** Building infrastructure for imagined future needs. "We might need this later."

**Result:** Unused abstractions. Complexity without benefit. Wrong abstractions (built without real use cases).

**Alternative:** Let frameworks emerge from real needs. When you see the same pattern three times, extract it.

---

## Decision Heuristics

When two options seem equivalent, prefer:

| Over This | Prefer This | Because |
|-----------|-------------|---------|
| Horizontal progress | Vertical completion | Users can test and use vertical slices |
| Speculative design | Concrete need | Real needs reveal real requirements |
| Internal improvement | User-facing capability | Mission is authoring, not elegance |
| Large change | Small testable change | Smaller feedback loops catch problems earlier |
| Working around TBD | Resolving TBD | Decisions get harder to change over time |
| Building new | Using existing | Existing code is tested and understood |
| Clever solution | Simple solution | Simple is easier to maintain and extend |

---

## Identifying What's Next

A process for any agent or human to determine the next work item:

1. **Check for blockers**
   - Any TBDs that block current work?
   - Any missing frameworks needed for planned features?
   - Resolve blockers before feature work.

2. **Check for active vertical slices**
   - Is there a content type or feature that's partially done?
   - Completing it is likely higher priority than starting something new.

3. **Check the mission**
   - Which incomplete capability would most advance the authoring goal?
   - Pick the one closest to "user can do X."

4. **Scope to testable increment**
   - What's the smallest piece of that capability you can build and test?
   - That's your next task.

5. **Verify before moving on**
   - Did it work? Is there a test? Can you demonstrate it?
   - Only move on when this piece is solid.
