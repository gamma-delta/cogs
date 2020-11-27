//! Query the player's input and turn it into controls.
//!
//! **NOTE: EVENT-STYLE CONTROLS ARE NOT IMPLEMENTED YET!**
//!
//! This module defines two main structs: `PollingInputHandler` and `EventInputHandler`.
//!
//! `PollingInputHandler` is used when your game engine provides player input by polling for it. The GGEZ library
//! uses this method; you can call `keyboard::get_keyboard_input` and get what keys are pressed that way.
//!
//! `EventInputHandler` is used when your game engine provides player input by producing Input events. SDL2 and
//! browsers with WASM work this way. I believe Piston also works this way, but I've never worked with Piston.
//!
//! Both styles of input handling should expose the same API for querying inputs.
//!
//! # Updating the Inputs
//!
//! It is VERY IMPORTANT that before you do *any* processing in your main loop you call `InputHandler::update`.
//! This tells the input handler that the next frame has elapsed.
//!
//! For `PollingInputHandler`, you must pass in a `&HashSet` containing all the pressed keys to `update`.
//!
//! `EventInputHandler` also exposes `TODO`, which you must call upon getting an input event.
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

use std::{collections::HashMap, collections::HashSet, hash::Hash};

use enum_map::{Enum, EnumMap};

/// Polling-based input handler.
/// See module-level documentation for more.
pub struct PollingInputHandler<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Clone> {
    /// Maps inputs to the controls they activate
    control_config: HashMap<I, C>,
    /// How long each control has been pressed
    input_time: EnumMap<C, u32>,
    /// If this is Some, we're waiting for a new control config.
    listening_for_input: Option<C>,
}

impl<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Clone> PollingInputHandler<I, C> {
    /// Create a new PollingInputHandler without any controls.
    pub fn new_empty() -> Self {
        Self {
            control_config: HashMap::new(),
            // conveniently, the default value for u32 is 0!
            // and we want the map to start full of zeros.
            // (zeroes?)
            input_time: EnumMap::new(),
            listening_for_input: None,
        }
    }

    /// Create a new PollingInputHandler with the specified controls.
    /// The HashMap in should map inputs to the controls you want them to actuate.
    pub fn new(control_config: HashMap<I, C>) -> Self {
        Self {
            control_config,
            input_time: EnumMap::new(),
            listening_for_input: None,
        }
    }

    /// Update the input handler. You MUST CALL THIS FIRST THING in your game loop.
    /// Otherwise things won't get updated correctly.
    pub fn update(&mut self, new_inputs: &HashSet<I>) {
        match &self.listening_for_input {
            None => {
                for (input, control) in self.control_config.iter() {
                    if new_inputs.contains(input) {
                        // this input is getting pressed!
                        // increment our timer
                        self.input_time[control.to_owned()] += 1;
                    } else {
                        // this input is not getting pressed
                        // reset our timer
                        self.input_time[control.to_owned()] = 0;
                    }
                }
            }
            Some(ctrl) => {
                if let Some(input) = new_inputs.iter().next() {
                    // we're pressing something!
                    self.control_config
                        .insert(input.to_owned(), ctrl.to_owned());
                }
            }
        }
    }

    /// Is this input pressed down?
    /// i.e. is the player pressing the button?
    pub fn pressed(&self, control: C) -> bool {
        self.input_time[control] >= 1
    }

    /// Is this input released?
    /// i.e. is the player *not* pressing the button?
    pub fn released(&self, control: C) -> bool {
        self.input_time[control] == 0
    }

    /// Is this input being clicked down?
    /// i.e. was it up last frame, but down this frame?
    pub fn clicked_down(&self, control: C) -> bool {
        self.input_time[control] == 1
    }
}
