# Getting Started

How to set up a new project, start a session, and understand the development process.

---

## Which Mode Are You In?

| Mode | Situation | Go To |
|------|-----------|-------|
| **Bootstrap** | Brand new repo, nothing exists yet | [Setup](#setup) |
| **Reboot** | Existing repo, cleared out | [Setup](#setup) |
| **Onboard** | Repo exists, you're new to it | [Assess](#assess) |
| **Resume** | Continuing work from previous session | [Session](#session) |

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   New or clean repo?  ──yes──▶  BOOTSTRAP (Setup)                   │
│      │                      │                               │
│      no                     ▼                               │
│      │               Create structure                       │
│      |               Create agents                          │
│      │                      │                               │
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

### Install beads skill

If `/beads` (preferred, Claude skill) or `bd` (secondary, CLI tool) aren't present, install using: https://github.com/steveyegge/beads/blob/main/docs/INSTALLING.md#claude-code-plugin-optional

### Repository Structure

```
project/
├── .claude/
│   ├── CLAUDE.md              # Project-specific instructions
│   ├── agents/                # Project-specific agents (from `docs/agents/`)
│   ├── skills/                # Project-specific skills (from `docs/skills/`)
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

### Bootstrapping Agents

Create `.claude/agents/` directory with specialized agents. See `docs/agents/` for full specs. Copy these over.

#### Agent Overview

| Agent | Purpose | Spec |
|-------|---------|------|
| **architect** | Pattern design and structural guidance | docs/agents/architect.md |
| **alignment** | Workflow and decision consistency | docs/agents/alignment.md |
| **best-practices** | SOLID, hexagonal, component design | docs/agents/best-practices.md |
| **bevy** | Bevy design and implementation guidance | docs/agents/bevy.md |
| **custodian** | Technical debt and code health | docs/agents/custodian.md |
| **deferral-guard** | Prevent stubs and deferrals | docs/agents/deferral-guard.md |
| **organizer** | Code organization and naming | docs/agents/organizer.md |

#### Domain agents

**Domain agents** — Create as needed for specific areas (camera, combat, editor, etc.)

### Bootstrapping Skills

Create `.claude/skills/` directory with specialized skills. See `docs/skills/` for full specs. Copy these over.

#### Skills Overview

| Skill | Purpose | Spec |
|-------|---------|------|
| **memory** | Persistent context across sessions | docs/skills/memory.md |
| **self-review** | Quality verification with rule of five | docs/skills/self-review.md |

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

4. **Identify work item**
   - Pick from ready issues, or
   - Identify emergent need from context

---

### Begin Work

Once mission-lead approves:

1. **Verify workflow alignment**
   - Which workflow does this serve?
   - Is there a walking skeleton to build/extend?
   - If blocked, address blocker instead

2. **Start implementation**
   - Start with test or spike as appropriate
   - Use domain agent if available
   - Track progress with beads

3. **Watch for deferral signals**
   - If deferral-guard blocks, stop and resolve
   - Do not proceed around blocked deferrals

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

How to evaluate an existing codebase.

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

2. **Run local agents**
   - Run alignment, architect, best practices, and mission-lead agents
   - Gather feedback
   - Use feedback to inform assessment

3. **Compare to documentation**
   - What's documented vs. implemented?
   - What's planned vs. built?
   - Where are the gaps?

4. **Identify blockers**
   - Unresolved decisions (TBDs)
   - Missing frameworks
   - Incomplete vertical slices

5. **Assess instruction quality**
   - Are terms defined? (glossary)
   - Is the goal clear? (mission)
   - Is prioritization framework present?
   - Do agents have enough context?

6. **Document findings**
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

1. **Identify** — What's the next highest priority? (Consult mission lead)
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
