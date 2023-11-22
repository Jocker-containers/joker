#!/bin/bash

# Check if Rust is installed
if command -v rustc &>/dev/null; then
    echo "Rust is already installed."
else
    # Rust is not installed, proceed with installation
    echo "Rust is not installed. Installing Rust..."

    # Download and run the official Rust installation script
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Add Rust to the user's PATH
    # source "$HOME"/.cargo/env

    # Display Rust version after installation
    rustc --version

    echo "Rust has been successfully installed."
fi

cargo build --release
echo '{"current_daemon":{"name":"localhost","socket_address":"127.0.0.1:8080"},"daemons":{}}' > config.cfg
cargo install --path .