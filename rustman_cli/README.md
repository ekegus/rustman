# RustMan - Hangman Game in Rust

## Introduction

RustMan is a simple hangman game implemented in the Rust programming language.

This project was created with the primary goal of learning and improving my Rust programming skills.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Todo](#todo)

## Getting Started

To get started with RustMan, you'll need to have Rust and Cargo (Rust's package manager) installed on your system. If you haven't already, you can download and install them from the official [Rust website](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, follow these steps to run RustMan:

1. Clone the RustMan repository to your local machine:

   ```bash
   git clone https://github.com/yourusername/rustman.git
   ```

2. Change your working directory to the project folder:

   ```bash
   cd rustman
   ```

3. Build the RustMan executable:

   ```bash
   cargo build --release
   ```

4. Start the game:

   ```bash
   cargo run --release
   ```

That's it! You're now ready to play RustMan.

## Todo

- [x] Fetch the words from an API.
- [x] The game logic into game.rs ?
- [x] Reshuffle directory structure so I have a lib and a bin crate
- [x] Separate concerns: Keep CLI logic in the rustman_cli and the game logic in rustman_core.
- [] Refactor main function so that it calls a play_game function.
- [x] Move the api helpers into a separate lib module.
- [] Write some tests
