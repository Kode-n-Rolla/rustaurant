# Rust Restaurant Simulator — Roadmap
## Goal
Build a small console-based restaurant simulator in Rust to practice:
- modules and project structure
- structs and enums
- `Option` and `Result`
- `loops`, `if`, `match`
- ownership and mutable state
- gradual feature growth without overengineering
________________________________________
## Core idea
This is not a real-time app. The program works in ticks.
What is a tick?
A tick is one game step / simulation step. It is not tied to real async time for now.
Example mental model:
- 1 tick = one host action cycle
- after each cycle, occupied tables move one step closer to becoming free
This keeps the project simple and avoids async/threading too early.
________________________________________
## MVP scope
### Included
- open / close restaurant
- console interaction
- tables with different capacities
- guest groups arriving
- seating logic
- waiting queue
- freeing tables after N ticks
- automatically seating queued groups when possible
- end of workday
- option to reopen restaurant or exit
### Explicitly excluded for now
- kitchen
- orders
- payment
- real timers / seconds
- async / multithreading
- random guest generation
- analytics / revenue system
- advanced optimizations
________________________________________
## Project structure
```
src/
├── main.rs
└── restaurant/
    ├── mod.rs
    ├── model.rs
    ├── logic.rs
    └── ui.rs
```

## File responsibilities
### `main.rs`
Application entry point. Responsible for:
- starting the app
- creating the restaurant
- running the top-level loop
- calling UI + logic functions

### `restaurant/mod.rs`
Module entry point. Responsible for:
- exposing submodules
- re-exporting important types later if needed

### `restaurant/model.rs`
Data models. Examples:
- Restaurant
- Table
- GuestGroup
- enums for status / decisions / results

### `restaurant/logic.rs`
Business logic. Examples:
- create restaurant state
- seat a group
- advance one tick
- free tables
- process queue
- close restaurant

### `restaurant/ui.rs`
Console I/O. Examples:
- ask whether to open restaurant
- ask for group size
- ask whether to queue or reject a group
- print current restaurant state
- print messages and errors
________________________________________
## High-level program flow
### main
- ask user whether to open restaurant
- create Restaurant
- start workday loop
    - print current state
    - ask whether a new group arrived
    - if yes: ask group size
    - try seating group
    - if no table: queue or reject
    - advance one tick
    - free tables if needed
    - auto-seat queue if possible
    - check closing condition
- print "restaurant closed"
- ask: reopen or exit
________________________________________
## Main entities
### Restaurant
Top-level state holder for the simulation. Why this is a struct:
- it groups the whole restaurant state in one place
- it prevents scattering many unrelated variables across main.rs
- it acts as the main object of the simulation
Likely responsibilities:
- current tick / time
- closing tick
- open/closed status
- dining room tables
- waiting queue
- service duration config
### Table
Represents one table. Likely stores:
- id
- capacity
- occupancy state
- remaining ticks until free
- current guest group (later via `Option`)
### GuestGroup
Represents one arriving group. Likely stores:
- id
- group size
- maybe waiting time later
________________________________________
## Data structure choices
### Tables
Use:
```
Vec<Table>
```
Because:
- all items are the same type
- easy iteration
- easy search for a suitable table
- order is not a problem
- simple and beginner-friendly
### Queue
Start with:
```
Vec<GuestGroup>
```
Later can be upgraded to:
```
VecDeque<GuestGroup>
```
if queue operations become more important.
For now, keep it simple.
________________________________________
## Seating rule
When a group arrives:
- find the smallest free table that can fit the whole group
- if found: seat the group
- if not found: ask user
  - queue the group
  - or let the group leave
Example:
- group size = 3
- free tables = 5, 4
- choose table 4 if available, otherwise 5
________________________________________
## Tick model
Why ticks instead of real seconds?
Because real-time behavior would require:
- sleep/timers
- more complex console UX
- possible async/threading
- more difficult state coordination
Ticks allow simple logic:
- each occupied table has remaining_ticks
- after each game step: remaining_ticks -= 1
- when it reaches 0, the table becomes free
This is enough for the first version.
________________________________________
## Development stages
### Stage 1 — Project skeleton (done)
Done / in progress:
- `cargo new`
- create restaurant/ module directory
- add mod.rs
### Stage 2 — Data models (done)
Create initial structs and enums:
- `Restaurant`
- `Table`
- `GuestGroup`
- restaurant status enum
- seat result enum
- user choice enum

Goal:
- only define the data model
- no complicated logic yet
### Stage 3 — Basic application loop (done)
Implement:
- ask user whether to open restaurant
- open restaurant
- run basic loop
- close restaurant
- offer reopen / exit (**not yet**) <----------
### Stage 4 — Tables and seating (done)
Implement:
- hardcoded table set
- print tables
- find suitable free table
- seat guest groups
### Stage 5 — Queue
Implement:
- if no table is available, ask user:
  - queue
  - reject
- store groups in waiting queue
### Stage 6 — Tick progression
Implement:
- configurable service duration in ticks
- decrement occupied table timers each turn
- free tables when time expires
### Stage 7 — Auto-seat queue <--- current here
Implement:
- after freeing tables, try seating queued groups automatically
- print notifications when queue groups are seated
### Stage 8 — Refactor and polish
Improve:
- cleaner module boundaries
- more readable output
- minor error handling improvements
- better naming
- polish codebase
- optional release notes / changelog
________________________________________
## Learning focus per stage
### Stage 2
Practice:
- struct
- enum
- module paths
- pub
### Stage 3
Practice:
- loop
- if
- match
- Result for input parsing
### Stage 4
Practice:
- iterating over `Vec<Table>`
- `Option` when searching for a table
- mutable references

### Stage 5–7
Practice:
- state transitions
- queue processing
- methods on structs
- separation between UI and business logic
________________________________________
## Important rules to avoid overengineering
1.	Do not add kitchen yet.
2.	Do not add async yet.
3.	Do not add real-time seconds yet.
4.	Do not use HashMap unless there is a clear need.
5.	Prefer simple, readable data structures first.
6.	Build one stage at a time.
7.	Keep the code compiling after each small step.
________________________________________
## Portfolio angle
This project can grow into a strong portfolio piece if developed iteratively.
Possible future versions:
- v0.1 basic host simulator
- v0.2 queue and auto-seating
- v0.3 kitchen module
- v0.4 statistics / reports
- v0.5 async or more advanced simulation

A simple release/dev diary can show:
- architecture growth
- steady progress
- versioning discipline
- improved Rust skills over time
________________________________________
## Current approved MVP summary
- console-based app
- user opens the restaurant
- workday runs in ticks
- user can add arriving guest groups
- host seats them if possible
- otherwise groups wait or leave
- tables free up after N ticks
- queued groups can be seated automatically
- restaurant closes at end of workday
- user can reopen or exit

