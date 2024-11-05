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

**Code Quality & Architecture** are essential elements for a **Highly Maintainable & Extensible Codebase**. Due to the nature of the community, Modularization is the essential element for Architecture, as Tests and Design Patterns are for Code Quality.

Codebases with low standards of Code Quality & Architecture are painfully known for unnecessary High Levels of Complexity which leads to many points that goes against the nature of the OT Community:
- Limits both **Progress & Creativity** as Editing, Adding, or Experimenting with the code becomes extremelly difficult requiring significant **Time, Expertise, and Familiarity** with increasingly bigger chuncks of the Codebase
- Bugs becomes common occurrences, hard to solve, easy to create, and hard to predict even in small changes.
- Difficult to understand how anything works as everything is connected and depends on multiple other parts, which often doesn't necessarily makes sense intuitively/conceptually
- Hard to Document, Test, and Maintain.

Essentially, the Codebase becomes **Calcified** and becomes a major impediment to Creativity, Customization, Collaboration and Newcomers.

The problem is that Architectural problems that lingers for many years are well known for being hard to simply correct it by refactoring the code over time. These kinds of Codebases begin to be referred as **Legacy Systems** meaning that they extremelly hard to change, big enough to prove a challenge to start anew, but still valuable to use even if inconvenient.

Such essential and fundamental problems can only be tackled by starting anew, with a fresh set of technologies, tools, and new eyes from all the knowledge gathered from the pains and problems of the old system.

An expected but mistaken reaction about starting anew, is people imagining how big and complex the current system is and how hard and how long it takes for them to make even simple edits, and projecting that same difficulty to the new system.

The reality is, with the advantage of modern technologies, tools, and knowledge from the old system, together with the benefit of higher code quality, design patterns, modularized architecture, documentation and tests, it will become 100x easier to develop, edit, and collaborate on the project, and things can start to flow way sooner than anyone fixated on the old ways imagine.

Although this all pertains to problems of organization, it is not all about the lack of that and keeping a higher standard to it. Essential to the choice is also the improvement of **Developer Experience & Development Environment**, which will be better explained in the next section about the choice of a new language.

## Why Rust?

Although C++ is a very solid language, its Development Environment and limitations are in odds with most of the needs of the Open Tibia Community.

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

# Some Perspectives for Improvements for TFS Old Ways
## **Modularization**
Modularization makes it very simple to edit/add anything, with minimal risk to impact anything else in the code.

It is possible to modularize virtually all parts of a system to make them extensible so they can be customized or replaced with alternatives as easily and fast as possible. Examples of parts on Tibia to be Modularized:
- Database Connection (Alternative DBs)
- Client-Server Communication Protocol (Custom Protocols)
- Cryptography Algorithms
- Map Loader
- Script Manager (Open to Multiple Languages)
- Player Setups (Vocs, Skills, Mont, Quest, Aura, Wing, etc...)
- Command Setups (Access Hierarchies)
- House System
- Market System
- Log System (+ Better Logs & Easy Configuration)
- etc...

All very extensible and easy to share with the community to add/remove at will.

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
