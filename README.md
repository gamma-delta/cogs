# COGS

> common, obnoxious game stuff

---

I find myself re-typing all sorts of stuff when making games in Rust.

Cogs has a bunch of those things that are obnoxious to type over and over. It's designed to be engine-agnostic.

## Module List

This crate is broken down into lots of modules. (Or, it will be, once I get around to writing them.)

They are:

- `controls` handles player input & controls.
  It works both with polling-style (like ggez) and event-style (like macroquad) input handling.
  You can query for both state and transitions (aka, detect when the player has *just* pressed a button.)
- `grids` has helper types for grid-based games: stuff like coordinates, directions, and rectangles.
- `ease` has a suite of easing and interpolation helpers.

## Why is the crate called `cogs-gamedev`?

Because someone already took the name `cogs` ;-;

## Serde Support

By enabling the crate feature `serde`, most things in the crate can be (de)serialized.
