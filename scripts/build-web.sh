#!/bin/bash
set -e

# Install wasm target if not installed
rustup target add wasm32-unknown-unknown

# Build for web
wasm-pack build --target web

# Setup web directory
mkdir -p www
cp -r pkg www/
cat > www/index.html << EOL
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>RustUI Web App</title>
</head>
<body>
    <script type="module">
        import init from './pkg/rust_ui.js';
        init();
    </script>
</body>
</html>
EOL

echo "Web build completed"
