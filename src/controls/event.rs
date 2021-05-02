use std::{collections::HashMap, hash::Hash};

use enum_map::{Enum, EnumMap};

use super::InputHandler;

/// Event-based input handler
/// See module-level documentation for more detail.
pub struct EventInputHandler<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Enum<bool> + Clone> {
    /// Maps inputs to the controls they activate
    control_config: HashMap<I, C>,
    /// How long each control has been pressed
    input_time: EnumMap<C, u32>,
    /// If this is Some, we're waiting for a new control config.
    listening_for_input: Option<C>,
    /// The set of all the control events we've gotten since we last called `update`
    pressed_controls: EnumMap<C, bool>,
}

impl<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Enum<bool> + Clone> EventInputHandler<I, C> {
    /// Create a new EventInputHandler without any controls.
    pub fn new_empty() -> Self {
        Self::new(HashMap::new())
    }

    /// Create a new EventInputHandler with the specified controls.
    /// The HashMap in should map inputs to the controls you want them to actuate.
    pub fn new(control_config: HashMap<I, C>) -> Self {
        Self {
            control_config,
            input_time: EnumMap::default(),
            listening_for_input: None,
            pressed_controls: EnumMap::default(),
        }
    }

    /// Call this function when your game engine gives you a KeyDown event,
    /// or any event signaling that an input is newly pressed down.
    pub fn input_down(&mut self, input: I) {
        match &self.listening_for_input {
            None => {
                if let Some(control) = self.control_config.get(&input) {
                    self.pressed_controls[control.to_owned()] = true;
                }
            }
            Some(ctrl) => {
                // Update the control ...
                self.control_config.insert(input, ctrl.to_owned());
                // and stop listening for inputs
                self.listening_for_input = None;
            }
        }
    }

    /// Call this function when your game engine gives you a KeyUp event,
    /// or any event signaling that an input has been released.
    pub fn input_up(&mut self, input: I) {
        if let Some(control) = self.control_config.get(&input) {
            self.pressed_controls[control.to_owned()] = false;
        }
    }

    /// Manually clear all the inputs the handler has received.
    ///
    /// (I'm not sure why you would want to do this, but hey, might as well
    /// expose the functionality.)
    pub fn clear_inputs(&mut self) {
        self.pressed_controls.clear();
    }

    /// Update the input handler. You MUST CALL THIS FIRST THING in your game loop.
    /// Otherwise things won't get updated correctly.
    pub fn update(&mut self) {
        if self.listening_for_input.is_none() {
            for (control, pressed) in self.pressed_controls.iter() {
                if *pressed {
                    // this input is getting pressed!
                    // increment our timer
                    self.input_time[control] += 1;
                } else {
                    // this input is not getting pressed
                    // reset our timer
                    self.input_time[control] = 0;
                }
            }
        }
    }
}

// there's gotta be a better way to do these generics
impl<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Enum<bool> + Clone> InputHandler<I, C>
    for EventInputHandler<I, C>
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

/// EnumMap doesn't implement Clone so we do it ourselves
impl<I: Hash + Eq + PartialEq + Clone, C: Enum<u32> + Enum<bool> + Clone> Clone
    for EventInputHandler<I, C>
{
    fn clone(&self) -> Self {
        let control_config = self.control_config.clone();
        let listening_for_input = self.listening_for_input.clone();

        let mut pressed_controls = EnumMap::default();
        for (k, v) in self.pressed_controls.iter() {
            pressed_controls[k] = *v;
        }

        let mut input_time = EnumMap::default();
        for (k, v) in self.input_time.iter() {
            input_time[k] = *v;
        }

        Self {
            control_config,
            input_time,
            listening_for_input,
            pressed_controls,
        }
    }
}
