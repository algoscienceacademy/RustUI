# RustUI - Cross-Platform UI Framework

A modern, cross-platform UI framework written in Rust that supports desktop, web, iOS, and Android platforms.

## Features

- 🎯 Cross-platform support (Desktop, Web, iOS, Android)
- 🎨 Flexible styling system
- 📱 Responsive layouts
- 🔄 State management
- 🎭 Animations
- 📍 Gesture recognition
- 🧭 Navigation system

## Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional dependencies for different platforms
# For iOS
xcode-select --install
rustup target add aarch64-apple-ios x86_64-apple-ios

# For Android
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-ndk

# For Web (WebAssembly)
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Building and Running

#### Desktop

```bash
# Run desktop example
cargo run --example basic_app

# Build for release
cargo build --release
```

#### iOS

```bash
# Build for iOS simulator
cargo build --target x86_64-apple-ios
# Build for iOS device
cargo build --target aarch64-apple-ios

# Open Xcode project
open ios/RustUI.xcodeproj
```

#### Android

```bash
# Build Android libraries
cargo ndk -t armeabi-v7a -t arm64-v8a build --release

# Open Android project in Android Studio
cd android && ./gradlew assembleDebug
```

#### Web

```bash
# Build WebAssembly package
wasm-pack build --target web

# Serve example
cd www && npm install && npm start
```

## Examples

### Basic App

```rust
use rust_native::*;

fn main() {
    let rust_native = RustUI::new();
    rust_ui.run(|| {
        View::new()
            .child(Text::new("Hello, RustUI!"))
            .child(Button::new("Click Me"))
            .into()
    });
}
```

### Styled Components

```rust
View::new()
    .child(
        Text::new("Welcome")
            .style(Style::new()
                .set("font-size", 24.0)
                .set("color", Color::new(0.1, 0.1, 0.1, 1.0)))
    )
```

### Responsive Layout

```rust
View::new()
    .with_responsive_layout(&responsive, vec![
        (Breakpoint::Small, Stack::new(Direction::Vertical)),
        (Breakpoint::Large, Stack::new(Direction::Horizontal)),
    ])
```

## Project Structure

```
RustUI/
├── src/
│   ├── components/     # UI components
│   ├── platform/      # Platform-specific implementations
│   ├── animation/     # Animation system
│   ├── gesture/       # Gesture recognition
│   ├── layout/        # Layout system
│   ├── navigation/    # Navigation system
│   ├── renderer/      # Rendering backend
│   └── style/         # Styling system
├── examples/          # Example applications
├── ios/              # iOS project files
├── android/          # Android project files
└── www/              # Web (WebAssembly) files
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Rust community
- Cross-platform UI frameworks that inspired this project