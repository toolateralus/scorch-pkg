#!/bin/bash

current_dir=$(pwd)

echo "installing 'scc' and dependencies"

if ! command -v rustup &> /dev/null; then
    echo "rustup is not installed. Installing..."
    sudo apt update
    sudo apt install rustup -y
    source $HOME/.cargo/env
fi

if ! command -v cargo &> /dev/null; then
    echo "cargo is not installed. Installing..."
    rustup install stable
    source $HOME/.cargo/env
fi

if ! grep -q "alias scc=\"cd $current_dir && cargo run --release --quiet\"" ~/.bashrc; then
    echo "alias scc=\"cd $current_dir && cargo run --release --quiet\"" >> ~/.bashrc
fi

source ~/.bashrc