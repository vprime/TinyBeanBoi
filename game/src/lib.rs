
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Primitive, PrimitiveStyle, Triangle},
};

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

impl Game {
    pub fn new() -> Self {
        Self {
            prev_input: InputState { left: false, right: false, },
        }
    }

    pub fn update<T: DrawTarget<Color = Rgb565>>(&mut self, display: &mut T, input_state: InputState) -> OutputState {

        if input_state.left != self.prev_input.left {
            println!("Left button updated to {:?} at {:?}", input_state.left, chrono::offset::Utc::now());
        }

        if input_state.right != self.prev_input.right {
            println!("Right button updated to {:?} this frame", input_state.right);
        }

        self.prev_input = input_state;

        match draw_smiley(display) {
            Ok(x) => x,
            Err(_) => todo!(),
        };

        OutputState {

        }
    }
}


fn draw_smiley<T: DrawTarget<Color = Rgb565>>(display: &mut T) -> Result<(), T::Error> {
    // Draw the left eye as a circle located at (50, 100), with a diameter of 40, filled with white
    Circle::new(Point::new(25, 40), 15)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    // Draw the right eye as a circle located at (50, 200), with a diameter of 40, filled with white
    Circle::new(Point::new(25, 60), 15)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    // Draw an upside down red triangle to represent a smiling mouth
    Triangle::new(
        Point::new(50, 45),
        Point::new(50, 55),
        Point::new(60, 50),
    )
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(display)?;

    Ok(())
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
