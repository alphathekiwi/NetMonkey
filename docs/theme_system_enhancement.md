# Theme System Enhancement: Hierarchical Theme Override Architecture

## Overview

This document outlines the enhanced theme system for Net Monkey that introduces hierarchical theme inheritance with override capabilities. The new system provides platform-specific theming while maintaining COSMIC desktop integration and allowing users to create custom themes through inheritance.

## Current State

The existing theme system uses a binary approach:
- COSMIC themes when in COSMIC desktop environment
- Standard Net Monkey themes elsewhere
- No inheritance or override capabilities
- Limited customization options

## Proposed Architecture

### Core Principles

1. **Platform-Aware Theming**: Automatically select appropriate base themes based on platform and desktop environment
2. **Hierarchical Inheritance**: Themes can inherit from parent themes with single-level inheritance only
3. **Override-Based Customization**: Child themes only store properties that differ from their parent
4. **Flattening on Save**: When all properties are overridden, remove parent relationship for optimization
5. **User Customization**: Allow users to create and edit themes regardless of platform

### Theme Structure

```json
{
  "name": "My Custom Dark Theme",
  "description": "Custom dark theme based on COSMIC Dark",
  "parent_theme": "cosmic_dark",  // Optional: inherit from this theme
  "overrides": {                  // Only properties that differ from parent
    "primary": { "r": 0.2, "g": 0.8, "b": 0.4, "a": 1.0 },
    "success": { "r": 0.0, "g": 0.9, "b": 0.2, "a": 1.0 }
  },
  "created_at": "2024-01-15T10:30:00Z",
  "modified_at": "2024-01-15T10:30:00Z",
  "is_dark": true
}
```

### Base Theme Types

#### COSMIC Integration Themes
- **`cosmic_dark`**: Dynamically generated from COSMIC desktop dark theme
- **`cosmic_light`**: Dynamically generated from COSMIC desktop light theme
- **Available only in COSMIC desktop environments**

#### Platform Base Themes
- **`linux_dark`**: LibCOSMIC-compatible dark theme for non-COSMIC Linux
- **`linux_light`**: LibCOSMIC-compatible light theme for non-COSMIC Linux
- **`windows_dark`**: Windows-optimized dark theme
- **`windows_light`**: Windows-optimized light theme
- **`macos_dark`**: macOS-optimized dark theme
- **`macos_light`**: macOS-optimized light theme

#### Universal Base Themes
- **`modern_dark`**: Cross-platform modern dark theme
- **`modern_light`**: Cross-platform modern light theme
- **`high_contrast`**: Accessibility-focused high contrast theme

## Implementation Details

### Theme Resolution Algorithm

```rust
pub fn resolve_theme(theme_name: &str, platform: Platform, desktop: Option<Desktop>) -> ResolvedTheme {
    1. Load theme definition
    2. If no parent_theme, return as complete theme
    3. If parent_theme exists:
       a. Resolve parent theme (recursive, but only one level deep)
       b. Merge parent properties with overrides
       c. Return merged theme
    4. Cache resolved theme for performance
}
```

### Default Theme Selection Logic

```rust
pub fn get_default_theme() -> String {
    // First detect system theme preference (light/dark)
    let is_dark_mode = match dark_light::detect() {
        dark_light::Mode::Dark => true,
        dark_light::Mode::Light => false,
        dark_light::Mode::Default => true, // Default to dark if unable to detect
    };
    
    let theme_suffix = if is_dark_mode { "_dark" } else { "_light" };
    
    match (current_platform(), detect_desktop_environment()) {
        (Platform::Linux, Some(Desktop::COSMIC)) => format!("cosmic{}", theme_suffix),
        (Platform::Linux, _) => format!("linux{}", theme_suffix),
        (Platform::Windows, _) => format!("windows{}", theme_suffix),
        (Platform::MacOS, _) => format!("macos{}", theme_suffix),
        _ => format!("modern{}", theme_suffix)
    }
}
```

**Note on Light/Dark Detection**: The `dark-light` crate provides the most reliable cross-platform method for detecting system theme preferences. It handles the platform-specific implementations:
- **Windows**: Reads registry keys for system theme settings
- **macOS**: Queries NSUserDefaults for appearance preferences  
- **Linux**: Checks various desktop environment configurations (GNOME gsettings, KDE configs, etc.)

This approach ensures users get the appropriate light or dark theme variant that matches their system preferences automatically.

### Theme Inheritance Rules

1. **Single Level Only**: Themes can inherit from one parent, parents cannot have parents
2. **Override Validation**: Child themes can only override properties that exist in parent
3. **Complete Override Detection**: If child overrides all parent properties, flatten and remove parent relationship
4. **Parent Availability**: If parent theme becomes unavailable, gracefully fallback to platform default

### User Theme Creation Workflow

#### Creating New Theme
1. User selects "Create New Theme"
2. System presents list of available base themes for their platform
3. User selects base theme and provides name/description
4. Theme editor opens with all properties from base theme
5. User modifies desired properties
6. System saves only overridden properties with parent reference

#### Editing Existing Theme
1. User selects existing theme
2. System resolves full theme (parent + overrides)
3. Theme editor shows all properties with inheritance indicators
4. User modifies properties
5. On save:
   - If theme has parent: save only changes as new overrides
   - If all properties are overridden: remove parent, save as complete theme
   - If saving as new theme: flatten all properties into new complete theme

### API Changes

#### New Theme Definition Structure
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThemeDefinition {
    pub name: String,
    pub description: String,
    pub parent_theme: Option<String>,
    pub overrides: Option<NetMonkeyColors>,  // Only overridden properties
    pub complete_colors: Option<NetMonkeyColors>,  // Full theme if no parent
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub is_dark: bool,
}
```

#### Theme Manager Enhancements
```rust
impl ThemeManager {
    pub fn resolve_theme(name: &str) -> Result<ResolvedTheme, ThemeError>;
    pub fn get_base_themes_for_platform() -> Vec<String>;
    pub fn create_child_theme(parent: &str, name: &str, overrides: NetMonkeyColors) -> Result<(), ThemeError>;
    pub fn flatten_theme(name: &str) -> Result<(), ThemeError>;
    pub fn validate_theme_hierarchy() -> Result<(), ThemeError>;
}
```

## User Experience

### Theme Selection Interface
```
┌─ Theme Selection ─────────────────────────┐
│ Current Theme: My Custom Dark             │
│                                           │
│ Available Themes:                         │
│ ● My Custom Dark (based on cosmic_dark)   │
│   Corporate Brand (based on windows_dark) │
│   COSMIC Dark (system)                   │
│   Windows Dark (system)                  │
│   Modern Dark (universal)                │
│                                           │
│ [Create New Theme] [Edit Current Theme]   │
└───────────────────────────────────────────┘
```

### Theme Editor Interface
```
┌─ Edit Theme: My Custom Dark ──────────────┐
│ Based on: cosmic_dark [Show only Overridden] │
│                                           │
│ Primary Color:    [████] #33CC66 ⚫       │
│ Success Color:    [████] #00E632 ⚫       │
│ Background:       [████] inherited        │
│ Text Color:       [████] inherited        │
│                                           │
│ ⚫ = Overridden from parent               │
│                                           │
│ [Save Changes] [Save As New] [Reset]      │
└───────────────────────────────────────────┘
```

## Benefits

### For Users
- **Familiar Base Themes**: Automatically get platform-appropriate defaults
- **COSMIC Integration**: Seamless system theme integration when available
- **Easy Customization**: Start with good base and modify only what you want
- **Reduced Complexity**: Don't need to configure every color property
- **Cross-Platform Consistency**: Themes work across platforms with appropriate base themes

### For Developers
- **Reduced Storage**: Store only differences, not complete themes
- **Platform Optimization**: Different base themes optimized for each platform
- **Maintainability**: Easier to update base themes without affecting custom themes
- **Performance**: Cached resolved themes for faster loading
- **Flexibility**: Support both simple and complex theming scenarios

## Migration Strategy

### Phase 1: Infrastructure
1. Implement new theme definition structure
2. Create base themes for each platform
3. Add theme resolution and inheritance logic
4. Maintain backward compatibility with existing themes

### Phase 2: UI Enhancement
1. Update theme selection interface
2. Create theme editor with inheritance indicators
3. Add theme creation wizard
4. Implement theme preview functionality

### Phase 3: Platform Integration
1. Enhance COSMIC desktop detection
2. Implement platform-specific base theme selection
3. Add automatic theme switching based on environment
4. Create migration tool for existing custom themes

## File Structure

```
net_monkey/
├── theme/src/
│   ├── inheritance.rs          # New: Theme inheritance logic
│   ├── platform_detection.rs   # New: Platform and desktop detection
│   ├── base_themes.rs          # New: Platform-specific base themes
│   └── resolver.rs             # New: Theme resolution and caching
├── app/data/themes/
│   ├── base/                   # Base themes (read-only)
│   │   ├── cosmic_dark.json
│   │   ├── linux_dark.json
│   │   ├── windows_dark.json
│   │   └── macos_dark.json
│   └── custom/                 # User themes (read-write)
│       ├── my_theme.json
│       └── corporate.json
```

## Technical Considerations

### Performance
- Cache resolved themes to avoid repeated inheritance resolution
- Lazy load base themes only when needed
- Validate theme hierarchy on startup to catch circular dependencies

### Error Handling
- Graceful fallback when parent themes are missing
- Validation of override properties against parent schema
- Clear error messages for theme resolution failures

### Security
- Validate theme file contents to prevent injection
- Restrict theme file locations to prevent path traversal
- Sanitize theme names and descriptions

This enhanced theme system provides the flexibility and platform integration you requested while maintaining simplicity for end users and powerful customization options for advanced users.
