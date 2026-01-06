# Localization Architecture

Constraints and patterns for supporting multiple languages throughout the game.

## Principles

1. **Text separation** - No hardcoded user-facing strings
2. **Key-based lookup** - All text referenced by keys
3. **Fallback chain** - Missing translations fall back gracefully
4. **Runtime switching** - Language changeable without restart
5. **Asset awareness** - Some assets may need localization

## Text Architecture

**String Keys**
- All user-facing text uses keys: `"ui.menu.start"`
- Keys are hierarchical: `category.subcategory.item`
- Code never contains display strings

**Translation Files**
- One file per language
- Format: JSON, YAML, or Fluent
- Stored as assets, hot-reloadable

**Example Structure:**
```
locales/
├── en.json      # English (default)
├── es.json      # Spanish
├── ja.json      # Japanese
├── de.json      # German
└── ...
```

**File Format:**
```json
{
  "ui": {
    "menu": {
      "start": "Start Game",
      "options": "Options",
      "quit": "Quit"
    }
  },
  "dialogue": {
    "npc_greeting": "Hello, traveler!"
  }
}
```

## String Features

**Variables**
- Placeholders in strings: `"damage.dealt": "Dealt {amount} damage"`
- Named parameters for flexibility
- Type-safe insertion

**Pluralization**
- Handle singular/plural: `"items.count": "{count} item(s)"`
- Language-specific rules (some languages have multiple plural forms)

**Gender/Context**
- Some languages need grammatical context
- Support via key variants or Fluent-style selection

## Localized Assets

**Types that may need localization:**
- Textures with text (signs, UI elements)
- Audio with speech
- Videos with subtitles

**Strategy:**
- Asset path includes locale: `textures/signs/en/welcome.png`
- Or asset override system
- Minimal localized assets preferred

## Fallback Chain

```
Requested locale → Default locale → Key itself
     "es"       →      "en"       → "ui.menu.start"
```

## Constraints for Features

Features with user-facing text should:
- Use localization keys, never hardcoded strings
- Document all keys they introduce
- Consider text length variation (German ~30% longer)
- Handle missing translations gracefully
- Support variable substitution where needed

## Editor Integration

- Editor manages translation files
- Shows missing translations
- Import/export for translators
- Preview text in context

## Implementation Notes

**Libraries:**
- fluent-rs for Fluent format
- Custom JSON loader
- bevy_fluent if available

**Performance:**
- Cache translations at load
- Key lookup is hashmap
- Minimal runtime overhead
