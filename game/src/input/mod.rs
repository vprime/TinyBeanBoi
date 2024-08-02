use crate::InputState;

const LONG_PRESS:u16 = 1000;
const MULTI_TAP_DURATION:u16 = 500;

#[derive(Default)]
pub struct Input {
    pub left: InputControl,
    pub right: InputControl,
}

impl Input {
    pub fn update(&mut self, input_state: InputState, time: u128, frame: u64) {
        self.left.update(input_state.left, time, frame);
        self.right.update(input_state.right, time, frame);
    }
}

#[derive(Default)]
pub struct InputControl {
    state: bool, // True when pressed
    interacting: bool, // True when tracking an interaction
    frame: u64,

    // Timers
    interaction_start: u128, // Beginning of this interaction
    press_start: u16,
    press_duration: u16, // duration of the last press
    release_start: u16, // duration since last release
    interaction_release_frame: u64,
    clicks: u8, // Number of clicks in succession
}

impl InputControl {
    pub fn update(&mut self, new_state: bool, time: u128, frame: u64) {
        self.frame = frame; // Track frame count for other member functions

        // If there's no interactions, exit
        if !self.interacting && !new_state {
            return;
        }

        // If this is the begining of a new interaction
        if !self.interacting && new_state {
            // Reset the state, and start the timers
            self.interacting = true;
            self.interaction_start = time;
            self.clicks = 0;
        }
        // Calculate the current interaction time
        let interaction_duration = (time - self.interaction_start) as u16;

        // When there's a state change
        if new_state != self.state {
            // If a button is pressed
            if new_state {
                // record the new press time
                self.press_start = interaction_duration;
            } else {
                // If a button was released, record the timers, and click counter.
                self.press_duration = interaction_duration - self.press_start;
                self.release_start = interaction_duration;
                self.clicks += 1;
            }
            // Update the visible state
            self.state = new_state;
        }

        // If the button has been released and the interaction got stale
        if !new_state && interaction_duration - self.release_start > MULTI_TAP_DURATION {
            // Clear interaction watch, and mark the release frame
            self.interacting = false;
            self.interaction_release_frame = frame;
        }
    }

    /// Returns true while the button is pressed down
    pub fn pressed(&self) -> bool {
        self.state
    }

    /// Returns true if button was clicked
    pub fn click(&self) -> bool {
        // Button is clicked if it was recently released, but it wasn't a double or long click
        // If the button was released from interaction this frame
        // and click counter is 1
        // and press duration is less than LONG_PRESS
        self.interaction_release_frame == self.frame
        && self.clicks == 1
        && self.press_duration < LONG_PRESS
    }

    /// Returns true if button was double clicked
    pub fn double_click(&self) -> bool {
        // A double click is when the button is clicked twice in rapid succession
        // If the button was released this frame
        // and click counter is 2
        // and press duration is less than LONG_PRESS

        self.interaction_release_frame == self.frame
        && self.press_duration < LONG_PRESS
        && self.clicks == 2
    }

    /// Returns true if button was long pressed
    pub fn long_press(&self) -> bool {
        // A long press is when the button is held for more than LONG_PRESS
        // If the button was released this frame
        // And was held for longer than LONG_PRESS

        self.interaction_release_frame == self.frame
        && self.press_duration > LONG_PRESS
    }
}