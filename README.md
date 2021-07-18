# COGS

> common, obnoxious game stuff

---

I find myself re-typing all sorts of stuff when making games in Rust. When I get too frusturated of typing
something, I stick it in this crate.

To that end, this crate is a grab-bag of game utilities. It's designed to be engine-agnostic.

## Feature List

This crate is broken down into lots of modules, each of which has a loosely
thematically connected set of stuff in it.

They are:

- `chance` currently only has `WeightedPicker<T>`, which lets you do weighted averages.
- `controls` handles player input & controls.
  It works both with polling-style (like ggez) and event-style (like macroquad) input handling.
  You can query for both state and transitions (aka, detect when the player has *just* pressed a button.)
- `ease` has a suite of easing and interpolation helpers.
- `grids` has helper types for grid-based games: stuff like coordinates, directions, and rectangles.
- `hash` lets you do quick-and-dirty hashing for things like variagated tilesets.

## Why is the crate called `cogs-gamedev`?

Because someone already took the name `cogs` ;-;

## Serde Support

By enabling the crate feature `serde`, most things in the crate can be (de)serialized.
