#!/usr/bin/env bash
set -e

# Install system dependencies (Debian/Ubuntu example)
if command -v apt-get >/dev/null; then
    sudo apt-get update
    sudo apt-get install -y curl build-essential libssl-dev libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev patchelf
fi

# Install Node.js LTS if not present
if ! command -v node >/dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
    sudo apt-get install -y nodejs
fi

# Install Rust via rustup if cargo is missing
if ! command -v cargo >/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install Tauri CLI
cargo install tauri-cli || true

# Install yt-dlp
if ! command -v yt-dlp >/dev/null; then
    sudo curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
    sudo chmod a+rx /usr/local/bin/yt-dlp
fi

# Install Node dependencies
npm install
