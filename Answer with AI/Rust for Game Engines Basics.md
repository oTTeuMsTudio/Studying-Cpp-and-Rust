# Rust for Game Engines
Basics
Rust is a systems programming language gaining popularity in the game development community due to its focus on performance, reliability, and safety. Here are the basics for using Rust in game engines:

## Modules and Libraries

### 1. simple_game_engine: 
A minimal game engine that wraps SDL2 for visuals and provides a simple API for creating games.

### 2. fyrox: 
A feature-rich, production-ready 2D/3D game engine with a scene editor, formerly known as rg3d.

### 3. hotham: 
A lightweight, high-performance game engine for standalone VR headsets, recommended for 2D graphics.

### 4. Bevy: 
A simple, data-driven game engine with a strong focus on entity-component-system (ECS) architecture.

## Game Engine Features

### 1. Multi-threading: 
Rust’s ownership system and borrow checker enable safe and efficient multi-threading, making it well-suited for game engines.

### 2. Memory Safety: 
Rust’s memory safety features, such as borrowing and ownership, help prevent common errors like null pointer dereferences and use-after-free bugs.

### 3. Performance: 
Rust’s compiled language and lack of runtime overhead make it a good choice for performance-critical game engine components.

### 4. ECS Architecture: 
Many Rust game engines, including Bevy and Fyrox, adopt ECS architecture, which provides a flexible and scalable way to manage game objects and components.

## Getting Started

### 1. Cargo: 
Use Cargo, Rust’s package manager, to install and manage dependencies for your game engine project.

### 2. Rust Standard Library: 
Familiarize yourself with the Rust standard library, which provides a comprehensive set of APIs for tasks like input handling, graphics, and networking.

### 3. Game Engine Documentation: 
Consult the documentation for your chosen game engine to learn about its API, features, and usage.

## Example Code

Here’s a simple example using the simple_game_engine:

```rust
use simple_game_engine::{self as sge, prelude::*};

struct App {}

impl sge::Application for App {
    fn on_create(&mut self, canvas: &mut WindowCanvas, input: &InputState) -> sge::ApplicationResult {
        // Initialize game state here
        Ok(true)
    }

    fn on_update(&mut self, canvas: &mut WindowCanvas, input: &InputState, elapsed_time: f64) -> sge::ApplicationResult {
        // Update game state and render here
        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App {};
    let mut engine = sge::Engine::new(&mut app, "Test App", 640, 480)?;
    engine.start(true)?; // Start the app with vsync enabled
    Ok(())
}
```

This example demonstrates the basic structure of a game engine application, including the on_create and on_update methods, which are called by the engine at initialization and each frame, respectively.

## Resources

- Are We Game Yet?: A community-driven website tracking Rust game development projects and libraries.
- Rust Game Development Frameworks: A curated list of Rust game engines and frameworks on GitHub.
- RustConf 2018 - Closing Keynote - Using Rust For Game Development: A presentation by Catherine West covering the basics of using Rust for game development.

By understanding these basics and exploring the resources provided, you’ll be well on your way to building games with Rust.

