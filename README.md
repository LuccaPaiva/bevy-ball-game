# Bevy Ball Game (Bevy 0.18 Port)

A simple 3D ball game built with the Rust game engine **Bevy**.

This project started from the excellent tutorial series by Frederick Joubert and was later migrated and updated to work with **Bevy 0.18**, adapting the codebase to the latest engine APIs and architectural changes.


<img width="800" height="447" alt="bevy-ball-game-gif" src="https://github.com/user-attachments/assets/ba6e9a00-c821-42e6-b40f-e8d9f2ece34d" />

## Project Overview

The goal of this project was to gain hands-on experience with:

* Rust programming
* ECS (Entity Component System) architecture
* Game development concepts
* Physics and collision handling
* Input systems
* Camera control
* Migration and maintenance of existing codebases

Rather than creating a game from scratch, the focus was on understanding an existing project and successfully updating it to a newer version of Bevy while preserving functionality.

## What I Worked On

### Bevy 0.18 Migration

The original project was developed for an older version of Bevy. This repository contains a complete migration to Bevy 0.18, including updates required by API and engine changes.

Key tasks included:

* Updating deprecated APIs
* Adapting system registration to the newer scheduling model
* Updating component and resource handling
* Resolving breaking changes introduced between Bevy releases
* Refactoring code where required to align with current Bevy patterns

### Learning Objectives

Through this project I explored:

* Rust ownership and borrowing concepts
* ECS-based software architecture
* Event-driven programming
* State management
* Modular code organization
* Debugging and troubleshooting during framework upgrades

## Technologies

* Rust
* Bevy 0.18
* ECS (Entity Component System)

## Running the Project

Clone the repository:

```bash
git clone <repository-url>
cd <repository-name>
```

Run:

```bash
cargo run
```

## Screenshots
Main Game

<img width="800" height="447" alt="bevy-ball-game-gif" src="https://github.com/user-attachments/assets/ba6e9a00-c821-42e6-b40f-e8d9f2ece34d" />


Main Menu

<img width="482" height="311" alt="image" src="https://github.com/user-attachments/assets/6f96f3b6-3cc4-41d8-8ef2-6df623fcedf6" />

Game Over

<img width="482" height="473" alt="image" src="https://github.com/user-attachments/assets/a9c2263a-0b17-4280-a52c-9a0d119c316d" />


## Future Improvements

Potential future enhancements include:

* Improved game mechanics
* UI and HUD improvements
* Audio effects
* Additional levels
* Performance optimizations
* Automated testing where applicable

## Acknowledgements

Original tutorial and project inspiration:
https://github.com/frederickjjoubert/bevy-ball-game

Frederick Joubert's Bevy Ball Game tutorial series:
https://taintedcoders.com/bevy/how-to/use-app-state


This repository is an independent learning project and Bevy 0.18 migration effort created for educational and portfolio purposes.
