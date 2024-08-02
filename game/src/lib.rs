mod input;
use std::time::{Instant, Duration};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics::image::Image;
use tinybmp::Bmp;
use crate::BlobcatTest::{base, blank, happy, left, right};
use crate::input::Input;

pub struct Game {
    time: Instant,
    frame: u64,
    pub input: Input,
    blobcat: BlobcatTest,
}

#[derive(Copy, Clone, Default)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
}

pub struct OutputState {

}

enum BlobcatTest {
    blank,
    base,
    left,
    right,
    happy,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            frame: 0,
            input: Input::default(),
            blobcat: BlobcatTest::blank,
        }
    }
}

impl Game {
    pub fn update<T: DrawTarget<Color = Rgb565>>(&mut self, display: &mut T, input_state: InputState) -> OutputState {
        // Update the game timer
        self.frame += 1;
        let now = self.time.elapsed().as_millis();

        // Update input from device
        self.input.update(input_state, now, self.frame);


        // Graphic drawing demo code
        if self.input.left.click() || self.input.right.click() {
            self.blobcat = base;
        }
        if self.input.left.double_click() {
            self.blobcat = left;
        }
        if self.input.right.double_click() {
            self.blobcat = right;
        }
        if self.input.left.long_press() && self.input.right.long_press() {
            self.blobcat = blank;
        } else if self.input.left.long_press() || self.input.right.long_press() {
            self.blobcat = happy;
        }

        // Draw
        let result = match self.blobcat {
            blank => {
                display.clear(Rgb565::BLACK)
            }
            base => {Self::draw_graphic(display, include_bytes!("../../assets/blobcat1.bmp"))}
            left => {Self::draw_graphic(display, include_bytes!("../../assets/blobcat2.bmp"))}
            right => {Self::draw_graphic(display, include_bytes!("../../assets/blobcat3.bmp"))}
            happy => {Self::draw_graphic(display, include_bytes!("../../assets/blobcat4.bmp"))}
        };

        // Unwrap draw result
        match result {
            Ok(_) => {}
            Err(_) => {}
        }

        // Return the output state
        OutputState {

        }
    }
    
    fn draw_graphic<T: DrawTarget<Color = Rgb565>>(display: &mut T, bmp_data: &[u8]) -> Result<(), T::Error> {
        let bmp = Bmp::from_slice(bmp_data).unwrap();
        Image::new(&bmp, Point::new(0, 0)).draw(display)
    }
}
