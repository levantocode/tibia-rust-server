# tibia-rust-server
An Open Tibia server focused on infrastructure modernization and the fundamental needs of the community.

Inspired by The Forgotten Server (TFS) with enhancements in code quality, testing, and architecture.

Developed in Rust and using a Game Engine (Bevy).

## Current Nature of the Community

The Open Tibia Community has been around for decades representing an open-source ecosystem of fans, developers, and server owners.

On aggregate, the community is extremelly passionate about the game, with a strong sense of creativity and collaboration. Focused on the development of Scripts, Bots, Custom Mechanics and even entire Custom Versions of the game. 

**Customization & Collaboration** is an essential part of the community, where a lot people new to programming try to get involved as well. Nowadays, customizations range from all versions since 7.0 up to recent 14. And includes many alternative versions of the game like:
- Pokémon
- Dragon Ball
- Naruto
- One Piece
- Seasonal
- IDLE
- NFT
- and more

And many new systems float around the community, like:
- Custom Vocations, Spells, and Monsters
- Lottery & Online Rewards
- Weapon Rarity & Attributes
- Rebirth
- Crafting
- Mining
- Raids
- Dungeon
- Events & Mini-games
- UIs
- much more

## Why "another" Project?
It has everything to do with the **Nature of the OT Community** and the availability of **Modern Technologies**.

We believe as the game and the community evolved to be very dynamic, the Development Environment and Codebase should evolve to match their demands as well.

As the essence of the Open Tibia Community is Creativity, Experimentation, Collaboration and Newer Developers, the current state of **Codebases & Development Environment** makes extremelly difficult for it to flourish and introduces an unnecessarily high barrier of entry that frustrates every single person that tries to get involved in the Open Tibia Community or start a Server as they grew from an old solution for an old game (although some of them are well maintained by competent developers). 

The nature of the community requires for maximum **Customization**, **Ease of Use** and **Collaboration**, which can be translated to high levels of:
- Modularization & Clean Design Patterns
- Ease of Including & Sharing Libraries
- Ease of Setup & Experimentation
- Centralization of Resources
- Organization & Documentation
- Test Creation & Maintenance

## Why Rust?

Although C++ is a very solid language, its development environment and limitations are in odds with most of the needs of the Open Tibia Community.

The Rust Ecosystem is essential in the modernization of the Server Infrastructure, and with the language also comes substantial improved Developer Experience including significant improvements in the Development Environment.

Developer Experience Improvements:
- Expressive Debugger: easier to find and know the solution to bugs
- Higher Memory Safety: no NULL Problems and related bugs
- Better Code Constructs: Simpler & More Expressive
- Much Easier to Add & Share Modules as Libraries
 
Included in the improvement of Development Experience, and with significant scope of being extremelly important for the nature of the Open Tibia Community is the improvements on the Development Environment:
- Much Lighter with No Configuration
- Automated & Integrated Builder (+ no CMake)
- Simple & Integrated Dependency Manager (awesome)

All things considered, Maintainability also improves significantly.

</br>

# Some TFS Spaces for Improvements
## **Modularization** (Very simple to edit/add anything, no risk to impact anything else)
  1. Better File Modules & Integration Layers
  2. Modularized Database Connection (easy to change/include new ones)
  3. Modularized Client-Server Communication Protocol (easy to change/include new ones)
  4. Modularized Cryptography (+ Easier to setup)
  5. Modularized & Complete Script Manager
  6. Modularized Map Loader
  7. Modularized Player Setups (Vocs, Skills, Mont, Quest, Aura, Wing, etc...)
  8. Modularized Command Setups
  9. Modularized House System
  10. Modularized Market System
  11. Modularized Log System (+ Better Logs & Easy Configuration)

## Better **Database Infrastructure**
- Multi-thread Database
  - Auto-saves + no freezes
  - No rollbacks
  - No resets to apply changes
- Better Database Structure (Tables & Columns)
- Better Database Setup/Config
- Better Interface with Code Functionalities + Easy to Add/Edit

## Etc
- **Initialization** Algorithm (Simpler & Straightforward)
- Better **Interface Protocols** between Client-Server Commands
- Integrated Map-Hunt-Monster setup/configuration
- Better Item & NPC Configuration Format
  - Possibly a database table
  - Simpler to Access
  - Allows for Dynamic Changes from Code
