<p align="center">
  <img src="github/osui.png" alt="OSUI" width="200px"/>
</p>

<h1 align="center">osui</h1>

<p align="center">
  <a href="https://crates.io/crates/osui">
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/osui?style=flat">
  </a>
  <a href="https://github.com/osui-rs">
    <img alt="GitHub Org's stars" src="https://img.shields.io/github/stars/osui-rs?style=flat">
  </a>
  <a href="https://github.com/osui-rs/osui">
    <img alt="GitHub License" src="https://img.shields.io/github/license/osui-rs/osui?style=flat">
  </a>
</p>

<p align="center">
  <b>OSUI is a customizable terminal user interface (TUI) library written in Rust. It provides a set of components and rsx to build interactive command-line interfaces with ease.</b>
</p>

```rust
use osui::{style::Transform, Screen};

fn main() -> std::io::Result<()> {
    let screen = Screen::new();

    screen
        .draw(format!("Hello, World"))
        .component(Transform::center());

    screen.run()
}
```

## [Documentation](https://osui.netlify.app/docs)

## Features

- Custom rsx syntax.
- Define and manage UI components.
- Handle keyboard input seamlessly.
- Create complex layouts using nested elements.
- Customizable element sizes and styles.

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to submit a pull request or open an issue.

## License

This project is licensed under the Apache License 2.0. See the LICENSE file for details.
