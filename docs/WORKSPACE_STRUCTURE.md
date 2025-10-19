# Net Monkey Workspace Structure

This document describes the workspace structure for the Net Monkey network monitoring application.

## Overview

Net Monkey has been reorganized into a Rust workspace with multiple crates to improve modularity, reusability, and maintainability. The workspace follows a clear separation of concerns:

- **Core**: Networking logic and utilities
- **Theme**: Custom theming system
- **Components**: Reusable UI components
- **App**: Main application
- **Examples**: Component demonstrations

## Workspace Structure

```
net_monkey/
├── Cargo.toml                 # Workspace root configuration
├── WORKSPACE_STRUCTURE.md     # This file
├── core/                      # Core networking functionality
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Core library exports
│       ├── adaptor.rs        # Network adapter discovery
│       ├── scanner.rs        # Network scanning utilities
│       └── tasks.rs          # Task management
├── theme/                     # Custom theming system
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Theme library exports
│       └── colors.rs         # Color palettes and themes
├── components/               # Reusable UI components
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Component library exports
│       ├── dropdown.rs       # Text input with dropdown
│       ├── label_with_hint.rs # Label with help tooltip
│       ├── selection_overlay.rs # Selection overlay component
│       ├── subnet_slider.rs  # Subnet mask slider
│       └── text_input_with_hint.rs # Text input with help
├── app/                      # Main application
│   ├── Cargo.toml
│   ├── build.rs             # Build script for Windows resources
│   ├── assets/              # Application assets
│   │   ├── icons.ttf
│   │   ├── background.png
│   │   └── net_monkey.ico
│   └── src/
│       ├── main.rs          # Application entry point
│       └── views/           # Application views
│           ├── mod.rs
│           ├── ip_scan.rs
│           ├── settings.rs
│           ├── tcp_client.rs
│           └── udp_client.rs
└── examples/                 # Component examples and demos
    ├── Cargo.toml
    └── src/
        ├── text_input_demo.rs
        ├── subnet_slider_demo.rs
        ├── dropdown_demo.rs
        └── all_components_demo.rs
```

## Crate Descriptions

### Core (`net_monkey_core`)
**Purpose**: Core networking functionality and utilities
**Dependencies**: `tokio`, `surge-ping`, `if-addrs`, `mac_address2`, `futures`
**Exports**:
- `NetworkAdapter` - Network adapter representation
- `get_network_adapters()` - Discover network adapters
- Task management types and utilities

### Theme (`net_monkey_theme`)
**Purpose**: Custom theming system for consistent UI styling
**Dependencies**: `iced`, `serde`
**Exports**:
- `NetMonkeyTheme` - Theme variants (Dark, Light, HighContrast)
- `NetMonkeyColors` - Color palettes
- Helper functions for themed containers and text

### Components (`net_monkey_components`)
**Purpose**: Reusable UI components with theming support
**Dependencies**: `iced`, `net_monkey_core`, `net_monkey_theme`
**Exports**:
- `TextInputWithHint` - Text input field with help tooltip
- `LabelWithHint` - Label with help tooltip
- `SubnetSlider` - Slider for subnet mask selection
- `TextInputDropdown` - Text input with dropdown suggestions
- Helper functions for component creation

### App (`net_monkey`)
**Purpose**: Main application binary
**Dependencies**: All workspace crates + `image`, `directories`, `tracing-subscriber`
**Features**:
- Network adapter selection
- IP scanning functionality
- TCP/UDP client tools
- Settings management
- Multi-tab interface

### Examples (`net_monkey_examples`)
**Purpose**: Demonstrations and examples of components
**Dependencies**: `iced`, `net_monkey_components`, `net_monkey_core`
**Binaries**:
- `text_input_demo` - TextInputWithHint component demo
- `subnet_slider_demo` - SubnetSlider component demo
- `dropdown_demo` - TextInputDropdown component demo
- `all_components_demo` - Comprehensive component showcase

## Building and Running

### Build the entire workspace
```bash
cargo build
```

### Build specific crate
```bash
cargo build -p net_monkey_core
cargo build -p net_monkey_components
cargo build -p net_monkey
```

### Run the main application
```bash
cargo run -p net_monkey
```

### Run component examples
```bash
cargo run -p net_monkey_examples --bin text_input_demo
cargo run -p net_monkey_examples --bin all_components_demo
```

### Check all crates
```bash
cargo check
```

## Benefits of Workspace Structure

### 1. **Modularity**
- Clear separation of concerns
- Each crate has a specific purpose
- Easier to understand and maintain

### 2. **Reusability**
- Components can be used in other projects
- Core networking logic is portable
- Theme system is standalone

### 3. **Build Optimization**
- Incremental compilation
- Only changed crates are rebuilt
- Better dependency management

### 4. **Testing and Examples**
- Components can be tested in isolation
- Dedicated examples for each component
- Easier to demonstrate functionality

### 5. **Team Development**
- Different teams can work on different crates
- Clear API boundaries
- Independent versioning possible

## Dependency Graph

```
app (net_monkey)
├── core (net_monkey_core)
├── components (net_monkey_components)
│   ├── core (net_monkey_core)
│   └── theme (net_monkey_theme)
└── theme (net_monkey_theme)

examples (net_monkey_examples)
├── components (net_monkey_components)
└── core (net_monkey_core)
```

## Development Guidelines

### Adding New Components
1. Create component in `components/src/`
2. Export from `components/src/lib.rs`
3. Add example in `examples/src/`
4. Update component documentation

### Adding Core Functionality
1. Add module to `core/src/`
2. Export from `core/src/lib.rs`
3. Update dependencies in other crates as needed

### Theme Modifications
1. Update color palettes in `theme/src/colors.rs`
2. Test with component examples
3. Verify application appearance

### Workspace Dependencies
- Shared dependencies are defined in workspace root `Cargo.toml`
- Use `dependency.workspace = true` in crate `Cargo.toml`
- This ensures version consistency across all crates

## Migration Notes

This workspace structure was migrated from a single-crate project. Key changes:

1. **Module reorganization**: Modules became separate crates
2. **Import updates**: `crate::` imports became external crate imports
3. **Dependency distribution**: Dependencies moved to appropriate crates
4. **Asset relocation**: Assets moved to app crate
5. **Example extraction**: Examples moved to dedicated crate

The migration maintains full functionality while providing better organization and development experience.