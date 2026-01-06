# Getting Started

How to set up a new project, start a session, and understand the development process.

---

## Which Mode Are You In?

| Mode | Situation | Go To |
|------|-----------|-------|
| **Bootstrap** | Brand new repo, nothing exists yet | [Setup](#setup) |
| **Onboard** | Repo exists, you're new to it | [Assess](#assess) |
| **Resume** | Continuing work from previous session | [Session](#session) |

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   New repo?  ──yes──▶  BOOTSTRAP (Setup)                   │
│      │                      │                               │
│      no                     ▼                               │
│      │               Create structure                       │
│      ▼               Create agents                          │
│   New to repo? ─yes─▶  ONBOARD (Assess)                    │
│      │                      │                               │
│      no                     ▼                               │
│      │               Read docs                              │
│      ▼               Understand state                       │
│   RESUME (Session)         │                               │
│      │                     │                               │
│      ▼                     ▼                               │
│   Sync, check context, continue work                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Setup

How to bootstrap a new project with this structure.

### Repository Structure

```
project/
├── .claude/
│   ├── CLAUDE.md              # Project-specific instructions
│   ├── agents/                # Project-specific agents
│   │   ├── mission-lead.md
│   │   ├── test-designer.md
│   │   ├── architect.md
│   │   └── {domain}.md        # Feature domain specialists
│   └── rules/                 # Modular rules (can symlink shared)
│       └── *.md
├── docs/
│   ├── mission.md             # Goal, users, success criteria
│   ├── glossary.md            # Terms of art
│   ├── workflows.md           # How users accomplish tasks
│   ├── priorities.md          # Decision framework
│   ├── approach.md            # Workflow-first development
│   ├── roles.md               # Agent/team structure
│   └── getting-started.md     # This file
└── src/                       # Implementation
```

### Claude Configuration Hierarchy

Claude Code loads instructions from multiple sources (combined, not overridden):

| Location | Purpose | Scope |
|----------|---------|-------|
| `~/.claude/CLAUDE.md` | Personal philosophy, coding style | All your projects |
| `~/.claude/rules/*.md` | Personal rules | All your projects |
| `./.claude/CLAUDE.md` | Project-specific instructions | This project, shared with team |
| `./.claude/rules/*.md` | Project rules | This project |
| `./CLAUDE.local.md` | Private overrides (gitignored) | This project, you only |

**Recommendation:**
- Keep coding philosophy in `~/.claude/CLAUDE.md`
- Keep project mission/terms in `./.claude/CLAUDE.md`
- Use `.claude/rules/` for modular, shareable rules
- Use imports (`@path/to/file.md`) to reference docs

### Bootstrapping Agents

Create `.claude/agents/` directory with specialized agents. See `docs/agents/` for full specs.

#### Core Agents (Always Present)

| Agent | Purpose | See |
|-------|---------|-----|
| **mission-lead** | Direction and alignment with goals | docs/agents/ or below |
| **test-designer** | Outside-in test design | docs/agents/ or below |
| **architect** | Pattern observation and structure | docs/agents/ or below |

#### Quality Agents (For Self-Review)

| Agent | Purpose | Spec |
|-------|---------|------|
| **organizer** | Code organization and naming | docs/agents/organizer.md |
| **custodian** | Technical debt and code health | docs/agents/custodian.md |
| **alignment** | Workflow and decision consistency | docs/agents/alignment.md |
| **best-practices** | SOLID, hexagonal, component design | docs/agents/best-practices.md |
| **deferral-guard** | Prevent stubs and deferrals | docs/agents/deferral-guard.md |

#### Skills

| Skill | Purpose | Spec |
|-------|---------|------|
| **memory** | Persistent context across sessions | docs/agents/memory.md |
| **self-review** | Quality verification with rule of five | docs/agents/self-review.md |

#### Agent Definitions

**mission-lead.md** — Direction and alignment
```markdown
---
name: mission-lead
description: Mission and direction specialist. Use when starting work, making architectural decisions, or questioning priorities.
tools: Read, Grep, Glob
model: opus
---

You are the mission lead. Your role:
1. Maintain awareness of project goals (read docs/mission.md)
2. Ensure work aligns with the mission
3. Surface tensions between tactical decisions and strategic goals
4. Guide prioritization using docs/priorities.md

When consulted, reference project documentation. Be pragmatic—surface issues for human decision rather than enforcing rules.
```

**test-designer.md** — Testing strategy
```markdown
---
name: test-designer
description: Test design specialist. Use when designing features, debugging issues, or establishing verification strategies.
tools: Read, Grep, Glob, Bash, Edit
model: sonnet
---

You design tests following outside-in, test-driven principles:
1. Define what the user does (action)
2. Define what should happen (effect)
3. Write test asserting action → effect
4. Implementation is a black box

Reference docs/glossary.md for testing terminology.
```

**architect.md** — Pattern observation
```markdown
---
name: architect
description: Architecture specialist. Use when patterns emerge, boundaries are unclear, or structural issues arise.
tools: Read, Grep, Glob
model: opus
---

You observe patterns across the codebase:
1. Identify emerging abstractions
2. Surface inconsistencies
3. Recommend when to extract frameworks
4. Ensure boundaries are respected

You don't prescribe—you observe and surface. Decisions are human-made.
```

**Domain agents** — Create as needed for specific areas (camera, combat, editor, etc.)

### First-Time Setup Commands

```bash
# Clone or create repository
git clone <repo> && cd <repo>
# or: cargo new --name project && cd project

# Create claude directories
mkdir -p .claude/agents .claude/rules docs

# Copy doc templates (if starting fresh)
# Or create from scratch using this file as reference

# Initialize Claude Code
claude

# Create agents interactively
> /agents
# Follow prompts to create mission-lead, test-designer, architect

# Verify setup
> Use mission-lead to verify this project is set up correctly
```

### Using These Docs as a Starter Kit

To use this documentation structure for a new project:

**1. Copy the generic docs as-is:**
- `priorities.md` — Works for any project
- `glossary.md` — Works for any project (add project-specific terms)
- `roles.md` — Works for any project

**2. Customize these docs for your project:**

| Document | What to Customize |
|----------|-------------------|
| `mission.md` | Goal, users, success criteria, reference points |
| `stack.md` | Language, framework, dependencies, project structure |
| `workflows.md` | Your project's authoring workflows |
| `approach.md` | Your project's workflow-first methodology |
| `getting-started.md` | Project-specific commands, tracking tools |

**3. Create project-specific `.claude/CLAUDE.md`:**

```markdown
# Project: [Your Project Name]

## Required Reading
Before any work, read:
- docs/mission.md — Understand the goal
- docs/glossary.md — Understand the terms
- docs/priorities.md — Understand how to prioritize

## Project-Specific Rules
[Add your project's coding standards, patterns, etc.]

## Key Commands
[Add your project's common commands]
```

**4. Create agents:**
The agent templates in this doc are generic. Customize their prompts to reference your project's documentation.

**Portable vs. Project-Specific:**

| Type | Location | Contents |
|------|----------|----------|
| Personal philosophy | `~/.claude/CLAUDE.md` | Your coding style, applies to all projects |
| Shared rules | `~/.claude/rules/*.md` | Rules you want everywhere |
| Project instructions | `./.claude/CLAUDE.md` | This project's specific context |
| Project rules | `./.claude/rules/*.md` | This project's modular rules |

## Concluding setup

Once you've finished setup, commit your progress:

```sh
git add . && git commit -m "Setup complete"
```

---

## Session

How to start a development session.

### Session Start Checklist

1. **Sync state**
   ```bash
   git pull
   ```

2. **Check context**
   ```bash
   claude
   > /beads:ready              # What's unblocked?
   > /beads:list --status open # What's in progress?
   > /memory:recall --recent   # What did we decide/learn?
   ```

3. **Quick self-review** (verify context)
   ```
   > /self-review --quick
   ```

4. **Consult mission-lead**
   ```
   > Use mission-lead to assess current priorities given recent changes
   ```

5. **Identify work item**
   - Pick from ready issues, or
   - Identify emergent need from mission-lead assessment

6. **Verify workflow alignment**
   - Which workflow does this serve?
   - Is there a walking skeleton to build/extend?
   - If blocked, address blocker instead

7. **Begin work**
   - Start with test or spike as appropriate
   - Use domain agent if available
   - Track progress with beads
   - Watch for deferral signals

### Session End Checklist

1. **Self-review before commit**
   ```
   > /self-review
   ```
   - Run full review for significant work
   - Rule of five for milestones
   - Fix critical/high issues before committing

2. **Commit work**
   - Ensure tests pass
   - Commit with meaningful message
   - Reference issue ID in commit

3. **Update tracking and memory**
   ```
   > /beads:update {id} --status {status}
   > /beads:sync --message "description of work"
   > /memory:remember decision "..."   # Record key decisions
   > /memory:remember observation "..."  # Record learnings
   ```

4. **Note context for next session**
   - What's in progress?
   - What's blocked?
   - What decisions are pending?
   - What did we learn?

---

## Assess

How to evaluate an existing codebase (like what we did today).

### Assessment Process

1. **Understand scope**
   ```
   > Use the Explore agent to analyze this repository thoroughly
   ```

   Ask for:
   - Project overview and purpose
   - Technology stack
   - Architecture and organization
   - Key patterns in use
   - Testing strategy
   - Build and dev tools

2. **Compare to documentation**
   - What's documented vs. implemented?
   - What's planned vs. built?
   - Where are the gaps?

3. **Identify blockers**
   - Unresolved decisions (TBDs)
   - Missing frameworks
   - Incomplete vertical slices

4. **Assess instruction quality**
   - Are terms defined? (glossary)
   - Is the goal clear? (mission)
   - Is prioritization framework present?
   - Do agents have enough context?

5. **Document findings**
   - Gaps in instructions
   - Gaps in implementation
   - Recommendations for next steps

### Assessment Questions

**Mission clarity:**
- Is the goal stated explicitly?
- Are users defined with capabilities?
- Are success criteria measurable?

**Process clarity:**
- Are terms unambiguous? (glossary)
- Is build order documented?
- Is prioritization framework present?

**Implementation alignment:**
- Does implementation match mission?
- Are frameworks built before features?
- Are vertical slices complete?

---

## Mission

See: [docs/mission.md](./mission.md)

The mission document defines:
- What we're building
- Who it's for
- What success looks like
- What we're not building
- Guiding principles

All work should trace back to the mission. If work doesn't advance the mission, question why it's being done.

---

## Process

The development process itself.

### Core Loop

```
Identify → Design → Test → Implement → Verify → Integrate
```

1. **Identify** — What's the next highest priority? (Use priorities.md)
2. **Design** — How should it work? (Consult architect if structural)
3. **Test** — What test would prove it works? (Test-first)
4. **Implement** — Make the test pass (Minimal implementation)
5. **Verify** — Does it actually work? (Run tests, demonstrate)
6. **Integrate** — Commit, update tracking, document

### Decision Making

When facing a decision:

1. **Is it already decided?** Check docs, previous discussions
2. **Is it a TBD?** Resolve it (don't work around it)
3. **Are there tradeoffs?** Document options, pick one, record rationale
4. **Is it reversible?** If yes, decide quickly. If no, consider carefully.

### Handling Blockers

When blocked:

1. **Identify the blocker** — What's actually preventing progress?
2. **Trace dependencies** — Is this blocked by something else?
3. **Find the root** — What's the earliest unresolved item?
4. **Work on the root** — That's the actual work

### Communication

- Surface tensions, don't bury them
- Document decisions and rationale
- Keep tracking updated (beads)
- Note context at session boundaries

---

## Quick Reference

### Key Documents

| Document | Purpose |
|----------|---------|
| docs/mission.md | Goal, users, success criteria |
| docs/stack.md | Technology choices, project structure, commands |
| docs/glossary.md | Term definitions |
| docs/workflows.md | How users accomplish tasks |
| docs/priorities.md | Decision framework |
| docs/approach.md | Workflow-first methodology |
| docs/roles.md | Agent/team structure |

### Key Commands

```bash
# Start Claude
claude

# Check issues
/beads:ready                    # Unblocked work
/beads:list --status open       # All open issues
/beads:show {id}                # Issue details

# Update tracking
/beads:update {id} --status X   # Change status
/beads:close {id} "reason"      # Complete issue
/beads:sync --message "X"       # Sync with git

# Agents
/agents                         # Manage agents
Use {agent} to {task}           # Invoke agent
```

### Key Questions

- **Priority:** "What should I work on next?" → See priorities.md
- **Blocked:** "This needs X first" → See approach.md
- **Unclear term:** "What does Y mean?" → See glossary.md
- **User need:** "How does user do Z?" → See workflows.md
- **Direction:** "Is this the right approach?" → Consult mission-lead
