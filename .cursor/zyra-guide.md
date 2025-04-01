# Zyra Guide for Cursor Editor

This guide helps Cursor understand how to effectively use Zyra for managing Git stacks in development workflows.

## Overview

Zyra is a Git stack management tool that helps organize and navigate through stacked changes in a Git repository. When working in Cursor, Zyra can be used to:

1. Manage feature branches in a stack-based workflow
2. Navigate between related changes
3. Visualize the current development stack
4. Maintain clean commit history

## Command Integration

### Basic Commands

When working in Cursor, use these Zyra commands in the integrated terminal:

```bash
# Initialize a new stack (at the start of a feature)
zyra init

# Create a new branch for a sub-feature
zyra branch feature/sub-component

# View the stack structure
zyra log

# Navigate through the stack
zyra prev  # Move to previous branch
zyra next  # Move to next branch
```

### Workflow Patterns

#### 1. Starting a New Feature

When beginning work on a new feature in Cursor:

```bash
# 1. Create and switch to a new feature branch
git checkout -b feature/main-component

# 2. Initialize a Zyra stack
zyra init

# 3. Create sub-feature branches as needed
zyra branch feature/sub-component-1
```

#### 2. Making Changes

While editing in Cursor:

1. Make changes in the current branch
2. Stage and commit changes
3. Use `zyra next` or `zyra prev` to navigate between related changes
4. Use `zyra log` to visualize the stack structure

#### 3. Code Review Workflow

When preparing changes for review:

1. Ensure each branch in the stack has a clean, focused set of changes
2. Use `zyra log` to verify the stack structure
3. Submit changes for review in the order of the stack

## Best Practices

### 1. Branch Organization

- Keep branches focused on single, logical changes
- Use descriptive branch names that reflect the change
- Maintain a clear dependency order in the stack

### 2. Navigation

- Use `zyra goto` to jump to specific branches
- Use `zyra prev`/`zyra next` for sequential navigation
- Check `zyra log` before switching branches

### 3. Stack Management

- Initialize new stacks for independent features
- Keep stacks focused on related changes
- Clean up completed stacks after merging

## Integration with Cursor Features

### 1. Terminal Integration

- Use Cursor's integrated terminal for Zyra commands
- Keep the terminal visible when managing stacks
- Use split panes to show both code and stack status

### 2. Source Control

- Use Cursor's source control panel alongside Zyra
- Verify changes in each branch before navigation
- Use Cursor's diff view to review changes

### 3. Keyboard Shortcuts

Consider setting up Cursor keyboard shortcuts for common Zyra commands:

```json
{
    "key": "cmd+shift+l",
    "command": "workbench.action.terminal.sendSequence",
    "args": { "text": "zyra log\n" }
}
```

## Error Handling

Common issues and solutions:

1. **Unclean Working Directory**
   - Save all files in Cursor
   - Commit or stash changes
   - Then use Zyra commands

2. **Navigation Errors**
   - Use `zyra log` to verify stack state
   - Ensure working directory is clean
   - Check Git status in Cursor's source control panel

3. **Branch Conflicts**
   - Use Cursor's merge conflict resolution tools
   - Resolve conflicts before continuing stack navigation
   - Verify changes in each branch

## Debugging Tips

When issues arise:

1. Check the stack structure with `zyra log`
2. Verify Git status in Cursor
3. Ensure all files are saved
4. Check for uncommitted changes
5. Verify branch dependencies

## Performance Considerations

For optimal performance in Cursor:

1. Keep stacks focused and manageable
2. Clean up completed or abandoned stacks
3. Regularly commit changes
4. Use Cursor's Git integration alongside Zyra

## Example Workflows

### Feature Development

```bash
# Start a new feature
git checkout -b feature/auth
zyra init

# Add sub-features
zyra branch feature/auth-ui
# Make UI changes in Cursor
git add .
git commit -m "feat: add authentication UI components"

zyra branch feature/auth-api
# Implement API integration
git add .
git commit -m "feat: integrate authentication API"

# Navigate and verify
zyra log
zyra prev  # Review UI changes
zyra next  # Back to API changes
```

### Bug Fix Stack

```bash
# Start a bug fix stack
git checkout -b fix/performance
zyra init

# Break down the fix
zyra branch fix/memory-leak
# Fix memory leak
git commit -am "fix: resolve memory leak in data processing"

zyra branch fix/optimization
# Implement optimization
git commit -am "perf: optimize data processing algorithm"

# Review changes
zyra log
```

## Maintenance

Regular maintenance tasks:

1. Clean up merged stacks
2. Verify stack integrity
3. Update Zyra when new versions are available
4. Keep branches focused and up-to-date

## Version Control

This guide is for Zyra v0.1.0. Commands and workflows may vary in future versions. 