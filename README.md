# Soot Sprint

A charming, minimalist infinite runner game inspired by the whimsical worlds of Studio Ghibli. Built entirely in Rust with the Macroquad game library.

*In a dusty nook, where sunbeams dance no more, a tiny Soot Sprite flickers to life, by a whisper of old magic. Now, a grand adventure awaits! Hop and dash through a forgotten world of giant wonders. How far can your tiny feet carry you?*

  <video src="https://github.com/user-attachments/assets/9242104c-af51-4ee5-93af-060c9d22bad5"></video>


## About The Game

Soot Sprint is a simple, single-button infinite runner designed as a learning project to explore game development in Rust. The player controls a soot sprite whose goal is to survive for as long as possible by jumping over a series of randomly appearing obstacles. The game is designed to be simple, charming, and a fun entry point into using the Rust programming language for creative projects.

## How to Play

* **Objective:** Survive as long as possible by jumping over the obstacles.
* **Desktop Controls:** Press the `Spacebar` to jump.

## Game Flow

The game follows a simple, looping flow:

1.  **Intro Screen:** The game begins by displaying a short, atmospheric text to set the mood and introduce the player to the world.
2.  **Start Game:** The player presses the button to start the game.
3.  **Gameplay Loop:** The Soot Sprite character begins running automatically. Obstacles spawn on the right side of the screen and scroll towards the player.
4.  **Collision:** If the player's character collides with an obstacle's hitbox, the game immediately transitions to the Game Over state.
5.  **Game Over:** The game world freezes, and a "GAME OVER" message is displayed with current and hightes score.
6.  **Restart:** From the Game Over screen, the player can press the `Spacebar` to instantly restart the game from the beginning, trying to beat their previous attempt.

## Core Techniques & Libraries Used

This project was built from the ground up using a specific set of tools and programming techniques.

### Language & Engine
* **Rust:** The entire game logic is written in Rust, a modern, memory-safe, and performant systems programming language. Key language features like `structs`, `enums`, and the `match` control flow are used extensively.
*  The project also demonstrates a core Rust concept: **Ownership and Borrowing**, especially when handling textures in the draw loop.
* **Macroquad:** A simple and easy-to-use Rust game library used for:
    * Window creation and management.
    * Loading and drawing all graphics.
    * Rendering primitive shapes and text.
    * Handling user input for keyboard.

### Game Logic Concepts
* **State Machine:** The game's flow is managed by a simple but effective state machine (`enum GameState { Playing, GameOver, Intro }`) which dictates the game's logic and what is drawn to the screen at any moment.
* **Entity Management:** Obstacles are represented by a custom `struct` and managed in a `Vec<Obstacle>`. This includes logic for:
    * **Procedural Spawning:** Obstacles are created at random intervals using `rand::gen_range`.
    * **Randomization:** The *type* of obstacle that spawns is also randomized, providing visual variety.
    * **Cleanup:** Obstacles that move off-screen are efficiently removed from the `Vec` to prevent memory usage from growing infinitely.
* **Simple Physics:** A basic physics simulation handles player gravity and jumping, managed by updating `velocity` and `position` variables each frame.
* **Collision Detection:** The game uses `Rect`-based collision detection (hitboxes) via Macroquad's built-in `.overlaps()` method to determine if the player has hit an obstacle.

## How to Build and Run

### Prerequisites
* [Rust](https://www.rust-lang.org/tools/install)
* `cargo-apk` (`cargo install cargo-apk`)

### Run on Desktop
Navigate to the project's root directory and run:
```
cargo run
```

## Future Ideas
* Add sound effects for jumping and game over events.
* Add player animations.
