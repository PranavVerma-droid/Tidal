#!/usr/bin/bash


sudo apt update

sudo apt install -y build-essential bison flex libgmp3-dev libmpc-dev libmpfr-dev texinfo wget \
                   nasm mtools python3 python3-pip python3-parted scons dosfstools libguestfs-tools qemu-system-x86

sudo apt-get install llvm clang

sudo apt install llvm-14 llvm-14-dev cmake

sudo apt-get install -y linux-image-generic
