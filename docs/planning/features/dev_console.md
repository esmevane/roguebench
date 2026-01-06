# Dev Console

Command-line interface for runtime debugging and manipulation.

## Core Logic

**Concept**

- Text input for commands
- Runtime state manipulation
- Debugging assistance
- Development builds only

**Command Categories**

| Category | Examples |
|----------|----------|
| Spawn | spawn enemy, spawn item |
| State | set health, give currency |
| Teleport | goto room, warp position |
| Debug | show colliders, toggle ai |
| Time | pause, set timescale |
| Network | disconnect, simulate lag |
| Script | run script, reload scripts |

**Command Structure**

```
command [subcommand] [arguments] [--flags]

spawn enemy grunt --position 100,200
set player health 100
toggle debug colliders
reload scripts
```

**Operations**

`execute(command_string)` - Run command

- Parse input
- Find handler
- Execute
- Return result

`register_command(name, handler)` - Add command

- Systems register their commands

`get_commands()` - List available

- For autocomplete

`get_history()` - Previous commands

- For navigation

**Console Features**

| Feature | Description |
|---------|-------------|
| Autocomplete | Tab completion |
| History | Up/down navigation |
| Help | `help command` for usage |
| Output | Command results displayed |
| Logging | Game logs visible |

**Invariants**

- Commands validated before execution
- Errors displayed, don't crash
- History persists during session
- Dev builds only

**Design Notes**

- Command set extensible
- Scripting can add commands
- Consider command permissions

---

## Bevy Integration

**Resources**

- ConsoleState { visible, input, history, output }
- ConsoleCommands { handlers: HashMap<String, CommandHandler> }

**Conditional Compilation**

```rust
#[cfg(debug_assertions)]
app.add_plugins(ConsolePlugin);
```

**Command Registration**

```rust
fn register_spawn_commands(mut commands: ResMut<ConsoleCommands>) {
    commands.register("spawn", |args| {
        // Parse and execute
    });
}
```

**Systems**

- Handle console toggle (tilde key)
- Process input
- Execute commands
- Display output

**UI**

- Overlay text input
- Scrollable output log
- Autocomplete dropdown
- Command history

**Scripting Integration**

- `run <script>` command
- `reload` for hot reload
- Script errors to console

**Network Commands**

- Simulate latency
- Force disconnect
- Show network stats

*See: dev_metrics.md, dev_control_center.md, architecture/scripting.md*
