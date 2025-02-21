# RustUI - Cross-Platform UI Framework

A modern, cross-platform UI framework written in Rust that supports desktop, web, iOS, and Android platforms.

## Quick Start

### Installation

```bash
# Install the rust-native development tools
cargo install rust-native

# Add rust-native to your project
cargo add rust-native
```

### Configuration

Create a `rust-native.toml` in your project root:

```toml
name = "my-app"
target_platforms = ["Desktop"]
build_command = "cargo run"  # Optional: custom build command
```

### Development Server

Start the development server:

```bash
# From your project directory
rust-native dev

# Or specify a custom path
rust-native dev --path ./src
```

### Server Controls

- `q` - Quit the server
- `1` - Switch to Desktop mode (currently active)
- `r` - Manual rebuild (coming soon)
- `s` - Restart server (coming soon)

### Status Indicators

The server displays build status with color coding:
- 🟢 Green: Ready/Success
- 🟡 Yellow: Building
- 🔴 Red: Error (with details)

## Project Structure

```
your-project/
├── src/
│   └── main.rs           # Your application code
├── rust-native.toml      # Development configuration
└── Cargo.toml           # Project dependencies
```

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
    rust_native.run(|| {
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

## Development Configuration Guide

### rust-native.toml Configuration

The `rust-native.toml` file is used to configure your RustUI development environment. Place this file in your project's root directory.

#### Basic Configuration

```toml
# Basic configuration
name = "my-app"
target_platforms = ["Desktop"]
build_command = "cargo run"
```

#### Advanced Configuration

```toml
# Advanced configuration
name = "my-complex-app"
target_platforms = ["Desktop", "Web", "iOS", "Android"]
build_command = "cargo run --example custom_app"
watch_paths = ["src/", "examples/"]
exclude_paths = ["target/", "node_modules/"]

[desktop]
window_size = { width = 800, height = 600 }
enable_hot_reload = true

[ios]
simulator = true
device_family = "iphone"

[android]
emulator = "Pixel_4_API_30"
target_sdk = 30

[web]
port = 8080
serve_dir = "public"
```

#### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `name` | String | Project directory name | Your application name |
| `target_platforms` | Array | `["Desktop"]` | Platforms to build for |
| `build_command` | String | `"cargo run"` | Custom build command |
| `watch_paths` | Array | `["."]` | Directories to watch for changes |
| `exclude_paths` | Array | `["target"]` | Directories to ignore |

#### Platform-Specific Options

##### Desktop Configuration
```toml
[desktop]
window_size = { width = 800, height = 600 }
enable_hot_reload = true
custom_args = ["--release", "--features=desktop"]
```

##### iOS Configuration
```toml
[ios]
simulator = true
device_family = "iphone"
deployment_target = "14.0"
```

##### Android Configuration
```toml
[android]
emulator = "Pixel_4_API_30"
target_sdk = 30
min_sdk = 21
```

##### Web Configuration
```toml
[web]
port = 8080
serve_dir = "public"
wasm_features = ["web-sys"]
```

#### Example Configurations

1. **Minimal Development Setup**
```toml
name = "my-app"
target_platforms = ["Desktop"]
```

2. **Desktop Application with Hot Reload**
```toml
name = "my-desktop-app"
target_platforms = ["Desktop"]
build_command = "cargo run --example main"

[desktop]
enable_hot_reload = true
window_size = { width = 1024, height = 768 }
```

3. **Cross-Platform Mobile Development**
```toml
name = "my-mobile-app"
target_platforms = ["iOS", "Android"]
watch_paths = ["src/", "mobile/"]

[ios]
simulator = true
device_family = "universal"

[android]
emulator = "Pixel_4_API_30"
target_sdk = 30
```

### Usage Tips

1. **Hot Reload Configuration**
   - Enable hot reload for faster development
   - Specify watch paths for selective reloading
   ```toml
   watch_paths = ["src/components/", "src/styles/"]
   exclude_paths = ["src/tests/", "target/"]
   ```

2. **Custom Build Commands**
   - Use environment variables
   - Chain multiple commands
   ```toml
   build_command = "RUST_LOG=debug cargo run --example dev"
   ```

3. **Platform-Specific Development**
   - Configure each platform separately
   - Enable only needed platforms
   ```toml
   target_platforms = ["Desktop", "Web"]
   [desktop]
   enable_hot_reload = true
   [web]
   port = 3000
   ```

For more information, visit our [Configuration Documentation](https://docs.rs/rust_native/configuration).

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support

- 📚 [Documentation](https://docs.rs/rust_native)
- 💬 [Discord Community](https://discord.gg/rust-native)
- 📧 [Email Support](mailto:sshahrearhossain@gmail.com)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Rust community
- Cross-platform UI frameworks that inspired this project