# AGENTS.md - Developer Guide for Dispatcher

This document provides guidance for agents working on the Dispatcher codebase.

## Project Overview

Dispatcher is a voice command to keyboard input dispatcher for Linux (X11). It uses:
- **iced** - GUI framework
- **vosk** - Voice recognition
- **rdev** - Keyboard/mouse input capture (X11)
- **tokio** - Async runtime
- **serde** - Serialization (JSON)

## Build, Lint, and Test Commands

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running the Application
```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release
```

### Testing
```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_serialze_profile

# Run tests in a specific module
cargo test action_profile

# Run tests with output
cargo test -- --nocapture
```

### Linting and Formatting
```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all

# Run clippy on release build
cargo clippy --release
```

### Documentation
```bash
# Generate documentation
cargo doc

# Generate documentation (open in browser)
cargo doc --open
```

### Other Useful Commands
```bash
# Check for errors without building
cargo check

# Build with all features
cargo build --all-features

# Clean build artifacts
cargo clean
```

## Code Style Guidelines

### General Principles
- Follow standard Rust idioms and conventions
- Use Rust 2024 edition (as specified in Cargo.toml)
- Keep code clear and readable over clever

### Naming Conventions
- **Structs/Enums/Traits**: `PascalCase` (e.g., `ActionProfile`, `WindowManager`)
- **Functions/Methods/Variables**: `snake_case` (e.g., `record_sequence`, `action_list`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `CONTROLKEYS`)
- **Modules**: `snake_case` (e.g., `ui`, `profile_manager`)

### File Organization
- One public module per file where possible
- Use `pub mod` for public modules, `mod` for private
- Group related functionality in subdirectories (e.g., `ui/profile.rs`)
- Main entry point in `main.rs`, library code in `lib.rs`

### Imports
- Use absolute imports from crate root: `use crate::module::Item`
- Group std, external crates, then local imports
- Use `use` statements for frequently used items
- Prefer bringing specific items into scope rather than globs

```rust
// Good
use crate::action_record::ActionRecord;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Avoid
use crate::*;
```

### Types and Generics
- Use explicit type annotations for public APIs
- Prefer generics for reusable code
- Use `Result<T, E>` for fallible operations
- Use `Box<dyn Error>` for errors that need dynamic dispatch

```rust
// Error handling pattern
pub fn from_file<T: de::DeserializeOwned>(file_path: &str) -> Result<T, Box<dyn Error>> {
    // ...
}
```

### Documentation
- Use doc comments (`///`) for public types and functions
- Use module-level docs (`//!`) in lib.rs/main.rs
- Document behavior, not implementation details
- Include examples where helpful

```rust
/// ActionProfile is essentially a list of any
/// events dispatcher listens for, and the actions
/// associated with that event.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ActionProfile {
    pub actions: Vec<ActionRecord>,
    pub name: String,
}
```

### Error Handling
- Use `Result` types for fallible operations
- Use `Box<dyn Error>` for flexible error types
- Use `eprintln!` for logging errors in examples/tests
- Return meaningful error messages

```rust
// Function returning Result
pub fn add_action(
    &mut self,
    name: String,
    activator_text: String,
) -> Result<(), Box<dyn Error>> {
    // ... implementation
}

// Error logging
if let Err(e) = input_dispatcher::send_input_sequence(guard, Duration::from_millis(20)) {
    eprintln!("Got an Error during voice processing for command {}: {}", a.name, e);
}
```

### Derive Macros
Use derive macros appropriately:
```rust
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ActionProfile { ... }
```

### Async Code
- Use `tokio` for async runtime (as this project does)
- Prefer `async fn` for functions that await
- Use channels for communication between async tasks

```rust
pub async fn listener_loop(action_list: Vec<ActionRecord>) {
    let (tx, mut rx) = mpsc::channel(50);
    // ...
}
```

### GUI (iced)
- Follow iced's Elm-like architecture (update, view, new)
- Use `Task` for side effects
- Use `Element<'_, Message>` for views
- Define message enums for user interactions

```rust
pub enum Message {
    MainUI(main_ui::MainUIMessage),
    Profile(profile_manager::Message),
}

pub struct WindowManager {
    window: Window,
}

impl WindowManager {
    pub fn new() -> Self { ... }
    pub fn update(&mut self, message: Message) -> Task<Message> { ... }
    pub fn view(&self) -> Element<'_, Message> { ... }
}
```

### Testing
- Place tests in `#[cfg(test)]` modules within the same file
- Use `#[test]` attribute for test functions
- Name tests descriptively: `test_<what_is_being_tested>`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_serialze_profile() -> Result<(), Box<dyn Error>> {
        let ap1 = ActionProfile::new(vec![], "Test Profile");
        // ... test implementation
        Ok(())
    }
}
```

### Dependencies
- Keep dependencies minimal
- Pin versions where necessary (e.g., `ort-sys = "=2.0.0-rc.4"`)
- Use path dependencies for local crates (e.g., `rdev = { path="../rdev/", features=["serialize", "x11"] }`)

## Platform-Specific Notes

- This project requires **X11** (does not support Wayland)
- Requires Linux (keyboard input capture)
- Vosk model directory should not be committed (already in .gitignore)
- Test outputs should not be committed

## Running Without Full Build

To quickly check for errors:
```bash
cargo check
```

To run a specific binary:
```bash
cargo run --bin dispatcher
```
