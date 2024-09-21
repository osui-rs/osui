# osui

`osui` is a Text User Interface (TUI) library written in Rust. 

## Features

- **Cross-Platform Support**: Works on various operating systems.
- **Flexible Layout**: Easily create and manage complex UIs.
- **Efficient Performance**: Designed for high performance and low latency.

## Installation

To use `osui` in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
osui = "^0.0.2"
```

```rust
use osui::{self, render_frame, ui};

fn main() {
    let txt = ui::text("Hello World!");
    render_frame(vec![txt.clone()]);
}

```

