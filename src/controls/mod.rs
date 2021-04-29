//! Query the player's input and turn it into controls.
//!
//! This module defines two main structs: `PollingInputHandler` and `EventInputHandler`.
//!
//! `PollingInputHandler` is used when your game engine provides player input by polling for it. The GGEZ library
//! uses this method; you can call `keyboard::get_keyboard_input` and get what keys are pressed that way.
//!
//! `EventInputHandler` is used when your game engine provides player input by producing Input events. SDL2 and
//! browsers with WASM work this way. I believe Piston also works this way, but I've never worked with Piston.
//!
//! # API
//!
//! Both styles of input handling implement the same trait, `InputHandler`. It has functions for querying the state of the
//! controls, but not for updating them.
//! This lets you write your main loop function generic over the kind of input handler you have, in case you want
//! to port your game to platforms with different input styles.
//!
//! # Updating the Inputs
//!
//! It is VERY IMPORTANT that before you do *any* processing in your main loop you call `InputHandler::update`.
//! This tells the input handler that the next frame has elapsed.
//!
//! For `PollingInputHandler`, you must pass in a `&HashSet` containing all the pressed keys to `update`.
//!
//! `EventInputHandler` also exposes `input_down` and `input_up`, which you must call upon getting an input event.
//!
//! # Generics
//!
//! To work with all the ways game libraries deal with input handling, the Input Handlers are generic over
//! the inputs `I` that the game engine gives you, and the controls `C` that your game uses.
//!
//! Inputs must be hashable (so, `Hash + Eq + PartialEq`).
//!
//! Controls must be useable with the EnumMap crate. You can do that easily enough with `#[derive(Enum)]`. (Internally,
//! the handlers use EnumMap to map each control to how long it has been held down.).
//!
//! Both of them must impl Clone (or Copy).
//!
//! It will probably be useful to define some type aliases to prevent having to type out your generics over and over
//! again. For example:
//!
// we ignore this block of code because i really don't want to bother with importing
// all those game libraries...
//! ```ignore
//! ##[derive(Enum)]
//! enum MyControls {
//!     Up,
//!     Down,
//!     Left,
//!     Right,
//!     Jump,
//!     Pause,
//! }
//!
//! /// When you're writing a game with GGEZ
//! mod ggez_game {
//!     type InputHandler = PollingInputHandler<ggez::input::keyboard::KeyCode, super::MyControls>;
//! }
//!
//! /// When you're writing a game with Piston
//! mod piston_game {
//!     type InputHandler = EventInputHandler<piston::input::keyboard::Key, super::MyControls>;
//! }
//!
//! /// When you're writing a game for the browser with wasm-pack or similar
//! mod browser_wasm_js_something {
//!     type InputHandler = EventInputHandler<String, super::MyControls>;
//! }
//!
//! ```
//!
//! # Changing Controls on the Fly
//!
//! Both input handlers support changing controls on the fly (perhaps through some sort of menu).
//! Call `listen_for_control_change` with the control you want to update the input for, and the next time an input
//! is received, that control will be associated with that input.
//!
//! If multiple controls are pressed at the same time during a frame where the input handler is
//! listening for a control change, it's undefined which one the control will be set to.
//! It will be set to one of them, however.

mod polling;
pub use polling::PollingInputHandler;
mod event;
pub use event::EventInputHandler;

use std::hash::Hash;

use enum_map::Enum;

/// The InputHandler trait, makng sure that both styles of input handling
/// expose the same API.
pub trait InputHandler<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Clone> {
    /// Is this input pressed down?
    /// i.e. is the player pressing the button?
    fn pressed(&self, control: C) -> bool;

    /// Is this input released?
    /// i.e. is the player *not* pressing the button?
    fn released(&self, control: C) -> bool;

    /// Is this input being clicked down?
    /// i.e. was it up last frame, but down this frame?
    fn clicked_down(&self, control: C) -> bool;
}
