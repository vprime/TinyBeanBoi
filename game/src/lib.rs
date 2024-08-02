

pub struct Game {
    prev_input: InputState,
}

#[derive(Copy, Clone)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
}
impl Default for InputState {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
        }
    }
}

pub struct OutputState {

}

impl Default for Game {
    fn default() -> Self {
        Self {
            prev_input: InputState { left: false, right: false, },
        }
    }
}

impl Game {
    pub fn update(&mut self, input_state: InputState) -> OutputState {

        if input_state.left != self.prev_input.left {
            println!("Left button updated to {:?} this frame", input_state.left);
        }

        if input_state.right != self.prev_input.right {
            println!("Right button updated to {:?} this frame", input_state.right);
        }

        self.prev_input = input_state;

        OutputState {

        }
    }
}




pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
