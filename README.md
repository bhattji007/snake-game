# Snake Game

Welcome to the Snake Game! This is a simple implementation of the classic Snake game using Rust programming language and the Piston game engine.

<p align="left">  
https://github.com/bhattji007/snake-game/assets/53906268/407a6baf-6205-44fc-90f8-804eeb111fc5
   </p>

## How to Play

- **Objective:** Control the snake to eat as much food as possible without colliding with the walls or itself.

- **Controls:**
  - Use the arrow keys (Up, Down, Left, Right) to control the direction of the snake.
  - The snake will move continuously in the direction it's facing until it hits a wall or itself.

- **Gameplay:**
  - The snake starts with a length of 3 blocks.
  - Each time the snake eats food, its length increases by 1 block.
  - The game ends if the snake collides with the walls or itself.

## Getting Started

To play the game locally, follow these steps:

1. Make sure you have Rust installed on your system. If not, you can download it from [rust-lang.org](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine:
   ```bash
   git clone https://github.com/yourusername/snake-game.git
3. Navigate to the project :
   ```bash
   cd snake-game
4. Build and run :
   ```bash
   cargo run
5. Use the arrow keys and enjoy the game
   
## Features
  - Simple and intuitive controls.
  - Colorful graphics using the Piston game engine.
  - Game over detection for collisions with walls or self.
  - Automatic increasing difficulty as the snake grows longer.

## Complex Enums and Advanced Rust Concepts

In this project, several complex enums and advanced Rust concepts are utilized to manage game mechanics and state efficiently:

### Enums:
The `Direction` enum represents the possible directions the snake can move.

### Structs:
Structs like `Block` and `Snake` are used to represent the components of the game, such as individual blocks of the snake's body and the snake itself.

### Linked List:
The `LinkedList` data structure from the standard library is used to manage the snake's body efficiently.

### Pattern Matching:
Pattern matching is extensively used, especially in the implementation of the `Direction` enum's `opposite` method and the `move_forward` method of the `Snake` struct.

### Ownership and Borrowing:
Rust's ownership and borrowing system is leveraged to ensure memory safety and prevent runtime errors, particularly in methods that manipulate the snake's body.

### Option Type:
The `Option` type is used to handle optional values, such as the direction of the snake's movement and the tail of the snake.

### Lifetime Annotations:
Lifetime annotations are implicitly used throughout the code to ensure that references remain valid for the duration they are used.

## Contributions
Contributions are welcome! Feel free to open issues or pull requests.
