# Net Monkey Examples

This directory contains examples demonstrating the usage of Net Monkey's reusable UI components. Each example showcases different components and their features.

## Available Examples

### 1. Text Input Demo (`text_input_demo`)
**File**: `src/text_input_demo.rs`
**Features**:
- Demonstrates `TextInputWithHint` component
- Shows text input fields with helpful tooltips
- Examples for IP address, port, and subnet inputs
- Live preview of entered values

**Run with**:
```bash
cargo run -p net_monkey_examples --bin text_input_demo
```

### 2. Subnet Slider Demo (`subnet_slider_demo`)
**File**: `src/subnet_slider_demo.rs`
**Features**:
- Demonstrates `SubnetSlider` component
- Interactive subnet mask selection (CIDR notation)
- Real-time calculation of host counts and network info
- Conversion to dotted decimal notation

**Run with**:
```bash
cargo run -p net_monkey_examples --bin subnet_slider_demo
```

### 3. Dropdown Demo (`dropdown_demo`)
**File**: `src/dropdown_demo.rs`
**Features**:
- Demonstrates `TextInputDropdown` component
- Combines text input with dropdown selection
- Pre-populated list of common IP addresses
- Shows both typing and selection capabilities
- Interactive instructions and current value display

**Run with**:
```bash
cargo run -p net_monkey_examples --bin dropdown_demo
```

### 4. All Components Demo (`all_components_demo`)
**File**: `src/all_components_demo.rs`
**Features**:
- Comprehensive showcase of all components
- Demonstrates component integration
- Theme system integration with `NetMonkeyTheme`
- Real-time configuration display
- Interactive theme switching (when implemented)
- Professional layout and styling

**Run with**:
```bash
cargo run -p net_monkey_examples --bin all_components_demo
```

## Component Features Demonstrated

### TextInputWithHint
- Custom placeholder text
- Helpful tooltip with usage hints
- Theming support
- Size customization
- Real-time value updates

### SubnetSlider
- Interactive slider for subnet mask selection
- CIDR notation support (/8 to /30)
- Real-time calculations:
  - Number of hosts per network
  - Number of possible subnets
  - Dotted decimal representation
- Custom styling and sizing

### TextInputDropdown
- Hybrid text input and dropdown component
- Type-to-filter functionality
- Click-to-select from predefined options
- Support for custom values
- Keyboard and mouse interaction

### LabelWithHint
- Text labels with help icons
- Tooltip explanations on hover
- Consistent styling with theme system
- Flexible positioning and sizing

## Building and Testing

### Build all examples:
```bash
cargo build -p net_monkey_examples
```

### Check all examples:
```bash
cargo check -p net_monkey_examples
```

### Run specific example:
```bash
cargo run -p net_monkey_examples --bin <example_name>
```

## Dependencies

The examples depend on:
- `iced` - GUI framework
- `net_monkey_components` - The component library
- `net_monkey_core` - Core types and utilities
- `net_monkey_theme` - Theming system

## Architecture

Each example follows the functional Iced application pattern:
- `Default` trait for initial state
- `update` function for message handling
- `view` function for UI rendering
- Clean separation of concerns

## Example Structure

```rust
// State management
impl Default for ExampleDemo {
    fn default() -> Self { /* ... */ }
}

// Message handling
impl ExampleDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        // Handle messages and return tasks
    }
    
    fn view(&self) -> Element<'_, Message> {
        // Build and return UI elements
    }
}

// Application entry point
pub fn main() -> iced::Result {
    iced::application(
        "Example Title",
        ExampleDemo::update,
        ExampleDemo::view,
    )
    .run_with(|| (ExampleDemo::default(), Task::none()))
}
```

## Usage in Your Projects

These examples serve as:
- **Documentation**: How to use each component
- **Templates**: Starting points for your own implementations
- **Testing**: Verification that components work correctly
- **Showcase**: Demonstration of component capabilities

You can copy and modify any example as a starting point for your own applications.

## Contributing

When adding new components to the library:
1. Create a corresponding example in this directory
2. Add the binary configuration to `Cargo.toml`
3. Follow the established patterns for consistency
4. Include comprehensive feature demonstration
5. Update this README with the new example

## Troubleshooting

### Common Issues:

**Build Errors**: Ensure all workspace dependencies are up to date
```bash
cargo update
```

**Runtime Issues**: Check that all required features are enabled in dependencies

**Theme Issues**: Verify that `net_monkey_theme` is properly imported and configured

For more help, refer to the main Net Monkey documentation or workspace structure guide.