# Roguebench

A spec for building a roguelike workbench with agent assistance.

## For the humans

For a human starting a new project:

1. Create repo.

```sh
cargo new my-project && cd my-project
```

2. Copy docs.

```sh
cp -r /path/to/starter/docs ./docs
```

3. Create claude structure.

```sh
mkdir -p .claude/agents
```

4. Start Claude.

```sh
claude
```

5. Bootstrap with agent.

```
> Read docs/getting-started.md - I'm starting a new project
```
