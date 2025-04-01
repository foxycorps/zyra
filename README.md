# Zyra

Zyra is a powerful Git workflow management tool that helps you manage stacked changes and navigate through your development workflow with ease. Built in Rust, it provides an intuitive CLI interface for managing Git branches and commits in a stack-based workflow.

## Features

- 🌳 **Stack Management**: Initialize and manage stacks of related changes
- 🔀 **Branch Navigation**: Easily move between branches in your stack
- 📝 **Stack Visualization**: View your current stack and its structure
- ⚡ **Quick Commands**: Short aliases for common operations
- 🎯 **Flexible Navigation**: Jump to any branch, stack, or commit

## Installation

### From Source

1. Ensure you have Rust installed ([rustup](https://rustup.rs/))
2. Clone the repository
3. Build and install:

```bash
cargo install --path .
```

## Usage

Zyra provides several commands to help manage your Git workflow:

### Basic Commands

- `zyra init` (alias: `i`): Initialize a new stack
- `zyra branch` (alias: `b`): Create a new branch
- `zyra log` (alias: `l`): Display the current stack
- `zyra prev` (alias: `p`): Navigate to the previous branch in the stack
- `zyra next` (alias: `n`): Navigate to the next branch in the stack
- `zyra goto` (alias: `g`): Navigate to a specific branch, stack, or commit

### Examples

```bash
# Initialize a new stack
zyra init

# Create a new branch
zyra branch feature/new-component

# View the current stack
zyra log

# Navigate through the stack
zyra next
zyra prev

# Jump to a specific branch
zyra goto feature/specific-branch
```

## Dependencies

- Git (via `git2`)
- Tokio for async operations
- Clap for CLI argument parsing
- Various utility crates for enhanced functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your chosen license here]

## Version

Current version: 0.1.0 