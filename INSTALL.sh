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

cmd="alias scc='(cd $current_dir; cargo build --release); $current_dir/target/release/scorch-pkg'"

if ! grep -q "$cmd" ~/.bashrc; then
    echo "$cmd" >> ~/.bashrc
fi

source ~/.bashrc