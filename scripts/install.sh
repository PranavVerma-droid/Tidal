#!/usr/bin/bash


sudo apt update

sudo apt-get install llvm clang

sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo apt-get install -y linux-image-generic
