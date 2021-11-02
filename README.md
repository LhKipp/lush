# LUSH - Lu-Shell

## Goals

The goal of lush is to make writing shell-scripts
1. Easy to do
2. Safe to execute during development phases
3. Assisted by tools

## Status

The shell is currently in its very first steps. While parsing, basic type-checking and evaluation of common constructs found in structured programming-languages and shells work, there is still a lot to do 😉

## Installation

```bash
git clone https://github.com/LhKipp/lush
cd lush
cargo build --workspace
chmod +x target/release/lush
./target/release/lush
```
