use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics::image::Image;
use tinybmp::Bmp;

#[derive(Default)]
pub struct Game {
    prev_input: InputState,
}

#[derive(Copy, Clone, Default)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
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

        if input_state.left && !input_state.right {
            Self::draw_graphic(display, include_bytes!("../../assets/blobcat2.bmp"));
        } else if input_state.right && !input_state.left {
            Self::draw_graphic(display, include_bytes!("../../assets/blobcat3.bmp"));
        } else if input_state.right && input_state.left {
            Self::draw_graphic(display, include_bytes!("../../assets/blobcat4.bmp"));
        } else {
            Self::draw_graphic(display, include_bytes!("../../assets/blobcat1.bmp"));
        }
        
        self.prev_input = input_state;
        OutputState {

        }
    }
    
    fn draw_graphic<T: DrawTarget<Color = Rgb565>>(display: &mut T, bmp_data: &[u8]) {
        let bmp = Bmp::from_slice(bmp_data).unwrap();
        match Image::new(&bmp, Point::new(0, 0)).draw(display) {
            Ok(x) => x,
            Err(_) => todo!(),
        };
    }
}
