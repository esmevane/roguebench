# Tutorial System

Framework for teaching players game mechanics through contextual hints and guided experiences.

## Core Logic

**Concept**

- Teach mechanics when relevant
- Non-intrusive by default
- Skippable for experienced players
- Tracks what's been taught

**Tutorial Types**

| Type | Trigger | Format |
|------|---------|--------|
| Contextual | First encounter | Tooltip/popup |
| Guided | Specific moment | Step-by-step |
| Practice | Safe space | Sandbox room |
| Reference | On demand | Help menu |

**Tutorial Entry**

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique identifier |
| trigger | enum | What activates it |
| content | text | What to show |
| input_demo | option | Control visualization |
| required_action | option | Must do to dismiss |
| prerequisite | option | Must learn first |
| skippable | bool | Can dismiss immediately |

**Trigger Types**

| Trigger | When |
|---------|------|
| FirstRoom | Enter first gameplay room |
| FirstEnemy | First enemy encounter |
| FirstDamage | Take damage first time |
| FirstPickup | Pick up first item |
| FirstDash | Use dash ability |
| FirstDeath | Die for first time |
| FirstShop | Enter shop |
| MenuOpen | Open specific menu |

**Operations**

`show_tutorial(id)` - Display tutorial

- Show content
- Pause if needed
- Track as shown

`complete_tutorial(id)` - Mark learned

- Record completion
- Unlock dependent tutorials

`skip_all_tutorials()` - Experienced player

- Mark all as complete
- No more prompts

`reset_tutorials()` - Replay tutorials

- Clear completion records
- Fresh start

`is_complete(id)` - Check status

- Has player seen this

**Invariants**

- Each tutorial shown once (unless reset)
- Prerequisites respected
- Skipping doesn't break game
- Completion persists

**Design Notes**

- Specific tutorials left to design
- Content pacing left to design
- Consider cognitive accessibility

---

## Bevy Integration

**Resources**

- TutorialProgress { completed: HashSet<TutorialId> }
- TutorialConfig { enabled, show_hints }

**Data**

- TutorialEntry { id, trigger, content, ... }

**Messages/Commands**

- ShowTutorial { id }
- CompleteTutorial { id }
- SkipAllTutorials
- ResetTutorials

**Events**

- TutorialShown { id }
- TutorialCompleted { id }
- TutorialSkipped { id }

**Systems**

- Listen for trigger conditions
- Display tutorial UI
- Track completion
- Check prerequisites

**UI**

- Overlay popup
- Control visualization
- Dismissible
- Optional pause

**Scripting Compatibility**

- Tutorials definable in data
- Triggers hookable
- Custom tutorials via scripting

*See: architecture/editor.md, accessibility_cognitive.md*
