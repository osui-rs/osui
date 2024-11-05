# OSUI - A Terminal User Interface (TUI) Library

OSUI is a customizable terminal user interface (TUI) library written in Rust. It provides a set of components to build interactive command-line interfaces with ease.

## Features

- Define and manage UI components.
- Handle keyboard input seamlessly.
- Create complex layouts using nested elements.
- Customizable element sizes and styles.

## Getting Started

To use OSUI in your project, include it in your `Cargo.toml`:

```toml
[dependencies]
osui = "0.2"  # Replace with the latest version
```

## Example Usage

Here’s a simple example of how to create a basic UI with OSUI:

```rust
use osui::{rsx, app::run, ui::*};

fn main() {
    run(&mut rsx! {
        text { "Hello, World!" }
    });
}
```

## Modules

- **element**: Defines base elements and their behavior.
- **key**: Handles keyboard input and mapping.
- **utils**: Provides utility functions for common TUI tasks.
- **ui**: Contains user interface components and layout management.

## Elements

### ElementSize

Defines the size of an element, either as a default size or a custom size.

### Element Trait

The core trait for all UI components, requiring the implementation of methods to retrieve data, update dimensions, render output, and handle updates based on user input.

### Commands

The library supports several commands that can be triggered during the application’s lifecycle:

- **Render**: Re-render an element with the given state.
- **Update**: Update an element's state based on user input.
- **Sleep**: Pause execution for a specified duration.
- **Exit**: Terminate the application gracefully.

## Macros

OSUI provides several macros to simplify the definition and usage of elements:

- **element!**: Define a new UI element with customizable properties and behaviors.
- **rsx!**: Create a nested structure of UI components easily.
- **command!**: Simplify command creation for updates.

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to submit a pull request or open an issue.

## License

This project is licensed under the Apache License 2.0. See the LICENSE file for details.
