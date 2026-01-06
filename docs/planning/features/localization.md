# Localization

Multi-language support with runtime language switching and translation management.

## Core Logic

**Concept**

- All text localizable
- Language selectable by player
- Translations in data files
- Runtime switching supported

**Localization Scope**

| Content | Localized | Notes |
|---------|-----------|-------|
| UI text | Yes | All interface strings |
| Dialogue | Yes | All conversation text |
| Item names | Yes | All item strings |
| Quest text | Yes | Objectives, descriptions |
| Tutorials | Yes | Hint and guide text |
| System messages | Yes | Errors, notifications |
| Audio | Optional | Subtitles preferred |

**Operations**

`get_text(key)` - Retrieve localized string

- Look up in current locale
- Fallback if missing
- Return localized text

`get_text_with_params(key, params)` - With variables

- Look up and substitute
- Handle pluralization

`set_locale(locale)` - Change language

- Update current locale
- Refresh displayed text
- Persist preference

`get_available_locales()` - List languages

- Return supported locales

`is_key_translated(key, locale)` - Check coverage

- For editor/debug

**Fallback Behavior**

1. Try requested locale
2. Try default locale (en)
3. Return key itself (debug indicator)

**Invariants**

- No hardcoded user-facing strings
- Missing translations visible in dev
- Language persists across sessions
- UI accommodates text length variation

**Design Notes**

- Supported languages left to implementation
- Translation process left to operations
- Consider right-to-left languages

---

## Bevy Integration

**Resources**

- CurrentLocale(String)
- Translations { locales: HashMap<String, TranslationData> }
- LocalizationConfig { default_locale, available }

**Assets**

- Translation files loaded as assets
- Hot-reloadable in development

**Messages/Commands**

- SetLocale { locale }
- ReloadTranslations

**Events**

- LocaleChanged { old, new }

**Systems**

- Load translations on startup
- Handle locale changes
- Refresh text components

**UI Integration**

```rust
// Instead of:
Text::new("Start Game")

// Use:
Text::new(t!("ui.menu.start"))
// Or component:
LocalizedText { key: "ui.menu.start" }
```

**Text Component**

```rust
#[derive(Component)]
struct LocalizedText {
    key: String,
    params: HashMap<String, String>,
}

fn update_localized_text(
    locale: Res<CurrentLocale>,
    translations: Res<Translations>,
    mut query: Query<(&LocalizedText, &mut Text)>,
) {
    for (localized, mut text) in &mut query {
        text.0 = translations.get(&localized.key, &localized.params);
    }
}
```

**Editor Integration**

- Edit translations in editor
- View missing keys
- Export for translators

*See: architecture/localization.md, architecture/editor.md*
