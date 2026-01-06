# Display Settings

Screen resolution, window mode, and visual quality options.

## Core Logic

**Settings**

| Setting | Type | Options | Default |
|---------|------|---------|---------|
| Resolution | enum | List of resolutions | Native |
| Window Mode | enum | Windowed, Borderless, Fullscreen | Borderless |
| VSync | bool | On/Off | On |
| Frame Limit | int | 30, 60, 120, 144, Unlimited | 60 |
| UI Scale | float | 0.5 - 2.0 | 1.0 |
| Pixel Perfect | bool | On/Off | On |

**Resolutions**

- 1280x720 (720p)
- 1920x1080 (1080p)
- 2560x1440 (1440p)
- 3840x2160 (4K)
- Native (monitor resolution)
- Custom (if supported)

**Window Modes**

| Mode | Behavior |
|------|----------|
| Windowed | Resizable window with borders |
| Borderless | Fullscreen window, easy alt-tab |
| Fullscreen | Exclusive, best performance |

**Operations**

`set_resolution(width, height)` - Change resolution

- Apply to window
- Update camera viewport
- Persist setting

`set_window_mode(mode)` - Change window mode

- Apply immediately
- Handle transition smoothly

`set_vsync(enabled)` - Toggle VSync

- Update present mode
- May require restart

`apply_settings()` - Apply all pending

- Batch apply changes
- Minimize flickering

**Invariants**

- Settings persist across sessions
- Invalid resolutions rejected
- Aspect ratio maintained or letterboxed
- UI scales with resolution

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Resolution | Native | Match monitor |
| Mode | Borderless | Best UX |
| VSync | On | Prevent tearing |

---

## Bevy Integration

**Resources**

- DisplaySettings { resolution, mode, vsync, ... }
- PendingDisplayChanges (for preview)

**Window Configuration**

```rust
Window {
    resolution: (width, height).into(),
    mode: WindowMode::BorderlessFullscreen,
    present_mode: PresentMode::AutoVsync,
    ..default()
}
```

**Systems**

- Apply settings to Window component
- Update camera on resolution change
- Handle window resize events
- Persist to config file

**UI**

- Resolution dropdown
- Window mode selector
- VSync toggle
- Frame limit selector
- UI scale slider
- Apply/Cancel buttons
- Preview before apply (optional)

**Platform Considerations**

- Query available resolutions
- Handle monitor changes
- Support multiple monitors (future)

**Camera Adjustment**

- Update OrthographicProjection
- Maintain game world scale
- Letterbox or pillarbox if needed
