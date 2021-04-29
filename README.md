# COGS

> common, obnoxious game stuff

---

I find myself re-typing all sorts of stuff when making games in Rust.

Cogs has a bunch of those things that are obnoxious to type over and over. It's designed to be engine-agnostic.

## Module List

This crate is broken down into lots of modules. (Or, it will be, once I get around to writing them.)

They are:

- `controls` handles player input & controls. It works both with polling-style (like ggez) and event-style (like piston) input handling. You can query for both state and transitions (aka, detect when the player has *just* pressed a button.)
- `directions` defines 4-way and 8-way direction enums, as well as helper functions to rotate and flip them.
- `int_coords` has type definitions for integer-based coordinates

## Why is the crate called `cogs-gamedev`?

Because someone already took the name `cogs` ;-;
