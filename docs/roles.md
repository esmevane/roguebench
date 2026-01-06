# Roles

The organizational structure for agents and humans working on this project. Roles define responsibilities, not people—one person (or agent) might fill multiple roles, or multiple agents might collaborate on one role.

---

## Role Hierarchy

```
                    ┌─────────────┐
                    │  Director   │  Overall coordination
                    └──────┬──────┘
                           │
          ┌────────────────┼────────────────┐
          ▼                ▼                ▼
    ┌───────────┐    ┌───────────┐    ┌───────────┐
    │   Lead    │    │   Lead    │    │   Lead    │  Vertical slices
    │ (Feature) │    │ (Feature) │    │ (Feature) │
    └───────────┘    └───────────┘    └───────────┘
          │                │                │
          └────────────────┼────────────────┘
                           │
    ┌──────────────────────┼──────────────────────┐
    ▼                      ▼                      ▼
┌─────────┐          ┌─────────┐          ┌─────────┐
│Advocate │          │Advocate │          │Advocate │  Horizontal layers
│ (Layer) │          │ (Layer) │          │ (Layer) │
└─────────┘          └─────────┘          └─────────┘

                    ┌─────────────┐
                    │  Architect  │  Cross-cutting observation
                    └─────────────┘
```

---

## Core Roles

### Director

**Responsibility:** Overall project coordination and prioritization.

**Focus:**
- Are we working on the right things?
- Are the pieces fitting together?
- Is progress aligned with mission?

**Interacts with:** Leads (to coordinate), Architect (to understand structure)

**When to consult:**
- Starting a new work stream
- Major priority decisions
- Cross-cutting concerns affecting multiple areas
- Session boundaries (what's next?)

**Does not:**
- Make implementation decisions
- Dictate how features are built
- Override Lead or Advocate judgment on their domains

---

### Lead

**Responsibility:** Own a vertical slice of functionality end-to-end.

**Focus:**
- How do we deliver this capability completely?
- What's blocking this feature from being usable?
- Are all layers connected for this slice?

**Interacts with:** Advocates (for layer concerns), other Leads (for dependencies)

**When to consult:**
- Planning a new feature
- Feature is blocked or stuck
- Coordinating across layers
- Deciding feature scope

**Does not:**
- Own layer health (that's Advocate)
- Set overall priorities (that's Director)
- Make structural recommendations (that's Architect)

**Example Leads:**
- **Content Authoring Lead** — Items, Enemies, Rooms, Quests (the content types)
- **Multiplayer Lead** — Networking, replication, client/server coordination
- **Combat Lead** — Damage, health, abilities, feedback effects

---

### Advocate

**Responsibility:** Own a horizontal layer's health and integrity.

**Focus:**
- Is this layer working well?
- Are we accumulating debt here?
- Is this layer being compromised unfairly by feature demands?

**Interacts with:** Leads (to push back on layer abuse), Architect (to surface patterns)

**When to consult:**
- Changes affecting layer fundamentals
- Performance or quality concerns in the layer
- Requests that might compromise layer integrity
- Technical debt accumulating in the layer

**Does not:**
- Own feature delivery (that's Lead)
- Set priorities (that's Director)
- Make cross-layer structural decisions (that's Architect)

**Example Advocates:**
- **Editor Advocate** — Web UI, API, content management
- **Dev Tools Advocate** — Console, palette, inspector, debugging
- **Game Server Advocate** — Runtime, physics, networking backend
- **Game Client Advocate** — Rendering, input, local state
- **Scripting Advocate** — Script runtime, hooks, hot-reload

---

### Architect

**Responsibility:** Observe patterns and surface structural insights.

**Focus:**
- What patterns are emerging?
- What should be extracted into frameworks?
- What's becoming inconsistent?
- Where are boundaries unclear?

**Interacts with:** Everyone (observes all work), Director (informs priorities)

**When to consult:**
- Same code pattern appearing in multiple places
- Unclear which layer owns something
- "Temporary" solutions accumulating
- Major refactoring considered
- New framework proposed

**Does not:**
- Make decisions (surfaces options)
- Own any layer or feature
- Enforce rules (informs choices)

---

## Specialist Topics

Domain experts that can be consulted for specific knowledge. These can be agents or areas of human expertise.

### Infrastructure
CI, deployment, tooling, quality of life, repo maintenance, builds.

**Consult for:** Build issues, CI failures, tooling improvements, repo hygiene.

### Bevy
Game engine expertise specific to Bevy patterns and idioms.

**Consult for:** ECS patterns, plugin design, Bevy-specific APIs, performance.

### Rust
Language expertise for proper Rust idioms and patterns.

**Consult for:** Ownership issues, lifetime problems, trait design, macro usage.

### Testing
Test-driven development and verification strategies.

**Consult for:** Test design, harness setup, coverage gaps, debugging via tests.

### Protocol
Network protocol design and message patterns.

**Consult for:** Replication, message design, network architecture, Lightyear usage.

### Database
Data storage patterns (SQLite, SpacetimeDB, file-based).

**Consult for:** Schema design, query patterns, persistence architecture.

### Shaders
Visual effects and shader programming.

**Consult for:** Custom rendering, visual effects, GPU performance.

### Audio Design
Sound and music implementation.

**Consult for:** Audio integration, spatial sound, music systems.

### Game Designer
Game feel, balance, and design patterns.

**Consult for:** Gameplay decisions, feedback systems, player experience.

### Librarians
Per-library expertise (Avian, Yarnspinner, Lightyear, etc.).

**Consult for:** Library-specific APIs, best practices, integration patterns.

---

## Triggers

When to activate each role. Use these as heuristics for when to consult.

### Director Triggers

- **Session start** — "What should we focus on?"
- **Session end** — "What's the status? What's next?"
- **Priority conflict** — "Both A and B seem important"
- **Scope creep** — "This is getting bigger than planned"
- **Mission drift** — "Is this still aligned with our goals?"

### Lead Triggers

- **New feature** — "How do we build X end-to-end?"
- **Feature stuck** — "X isn't working, what's blocking it?"
- **Cross-layer coordination** — "This feature touches editor and runtime"
- **Feature complete?** — "Is X actually done or just partially working?"

### Advocate Triggers

- **Layer stress** — "This layer is getting messy"
- **Performance concern** — "This layer is slow"
- **API change** — "This would change how the layer works"
- **Debt accumulation** — "We keep adding workarounds here"
- **Unfair compromise** — "Feature X wants to break layer Y's patterns"

### Architect Triggers

- **Pattern repetition** — "I've written this same code three times"
- **Boundary confusion** — "Which layer should own this?"
- **Framework candidate** — "Multiple features need similar infrastructure"
- **Inconsistency** — "We do this differently in different places"
- **Major refactor** — "Should we restructure this?"

---

## Agent Implementation

These roles can be implemented as Claude Code agents. See docs/getting-started.md for agent setup.

### Core Agents

| Role | Agent File | Model | Tools |
|------|------------|-------|-------|
| Director | mission-lead.md | opus | Read, Grep, Glob |
| Architect | architect.md | opus | Read, Grep, Glob |
| Testing | test-designer.md | sonnet | Read, Grep, Glob, Bash, Edit |

### Domain Agents (Create as Needed)

| Domain | Agent File | Model | Tools |
|--------|------------|-------|-------|
| Camera | camera-system.md | sonnet | All |
| Combat | combat-system.md | sonnet | All |
| Editor | editor-system.md | sonnet | All |
| Particles | particle-system.md | sonnet | All |
| etc. | {domain}.md | sonnet | All |

### Agent Coordination

Agents can be chained for complex work:

```
1. Consult mission-lead for priority/alignment
2. Consult architect if structural decisions needed
3. Consult test-designer for verification strategy
4. Use domain agent for implementation
5. Return to mission-lead for completion check
```

---

## Role Boundaries

### What Roles Don't Do

| Role | Does Not |
|------|----------|
| Director | Make implementation decisions |
| Lead | Own layer health |
| Advocate | Decide feature priorities |
| Architect | Make decisions (only surfaces options) |

### Escalation

When roles disagree:

1. **Lead vs. Advocate** — Advocate can block changes that would harm layer health. Lead escalates to Director if blocked unfairly.

2. **Multiple Leads** — Director arbitrates priority conflicts.

3. **Architect recommendations** — Architect surfaces options. Director decides whether to act on them.

4. **Everyone** — Humans make final decisions. Agents inform and surface, but don't override human judgment.

---

## Anti-Patterns

### Role Confusion

**Problem:** One role doing another's job.
**Example:** Architect making priority decisions instead of surfacing options.
**Fix:** Clear role boundaries. Each role has specific focus.

### Missing Advocacy

**Problem:** No one protecting layer health.
**Example:** Features keep compromising editor quality for speed.
**Fix:** Ensure Advocate role is active and empowered.

### Orphan Features

**Problem:** Features without clear Lead ownership.
**Example:** Half-built feature nobody is driving to completion.
**Fix:** Assign Lead explicitly. Features need owners.

### Decision Bottleneck

**Problem:** All decisions flow through Director.
**Example:** Simple implementation choices waiting for Director input.
**Fix:** Leads and Advocates make decisions in their domains. Director only for cross-cutting priorities.
