use std::{collections::HashMap, collections::HashSet, hash::Hash};

use enum_map::{Enum, EnumMap};

use super::InputHandler;

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
            input_time: EnumMap::default(),
            listening_for_input: None,
        }
    }

    /// Create a new PollingInputHandler with the specified controls.
    /// The HashMap in should map inputs to the controls you want them to actuate.
    pub fn new(control_config: HashMap<I, C>) -> Self {
        Self {
            control_config,
            input_time: EnumMap::default(),
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
                    self.listening_for_input = None;
                }
            }
        }
    }
}

// there's gotta be a better way to do these generics
impl<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Clone> InputHandler<I, C>
    for PollingInputHandler<I, C>
{
    /// Is this input pressed down?
    /// i.e. is the player pressing the button?
    fn pressed(&self, control: C) -> bool {
        self.input_time[control] >= 1
    }

    /// Is this input released?
    /// i.e. is the player *not* pressing the button?
    fn released(&self, control: C) -> bool {
        self.input_time[control] == 0
    }

    /// Is this input being clicked down?
    /// i.e. was it up last frame, but down this frame?
    fn clicked_down(&self, control: C) -> bool {
        self.input_time[control] == 1
    }
}
