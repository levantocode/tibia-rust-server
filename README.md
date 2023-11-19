# tibia-rust-server
Tibia server developed on the Rust Language inspired on TFS (TheForgottenServer) with code quality, tests, and architecture improvements.

</br>

# Why?
TFS C++     →     TFS Rust

## Terrible **Architecture**
- Leads to many unnecessary bugs & limitations
- Simply **Bad Code**
  - Very badly modularized
  - Almost no organization
  - Lots of things unnecessarily interconnected (higher complexity, unexpected bugs)
  - Requires a great degree of knowledge and familiarity to change even small things
- Terrible for the **Community**
  - Makes it Hard to edit/add or even try anything
  - Even those who can, takes way longer do edit/add anything
- Tests (amazing)

## Rust is in many ways an improvement over C++ (and more "attractive" as well)
- **Infrastructure**
  - Simple & Integrated Dependency Manager (makes a ton of difference)
  - Much lighter development environment, with basically no configuration
  - Automated & Integrated Builder (no CMake bullshit)
- **Development**
  - Much more expressive Debugger, easy to find and know the solution to Bugs
  - Easy to add any Dependency, or share any module as lib
  - Much higher memory safety
  - More expressive and simpler constructs to code

</br>

# Space for Improvements
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
