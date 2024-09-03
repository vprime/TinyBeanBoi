mod input;
mod ui;
mod sprite;

use std::collections::HashMap;
use std::io::Read;
use std::time::{Instant, Duration};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics::image::{Image, ImageRaw, ImageRawBE, ImageRawLE};
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable};
use embedded_graphics::text::Text;
use tinybmp::{Bmp, Pixels};
use crate::input::Input;
use crate::sprite::Assets;
use crate::ui::GameUi;

pub struct Game<'a> {
    time: Instant,
    frame: u64,
    pub input: Input,
    ui: GameUi,
    assets: Assets<'a>,
}

#[derive(Copy, Clone, Default)]
pub struct InputState {
    pub left: bool,
    pub right: bool,
}

const DISPLAY_WIDTH: u32 = 135;
const DISPLAY_HEIGHT: u32 = 240;
const DISPLAY_WIDTH_INCLUSIVE: u32 = DISPLAY_WIDTH - 1;
const DISPLAY_HEIGHT_INCLUSIVE: u32 = DISPLAY_HEIGHT - 1;

impl<'a> Default for Game<'a> {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            frame: 0,
            input: Input::default(),
            ui: GameUi::new(),
            assets: Assets::default(),
        }
    }
}

impl<'a> Game<'a> {
    pub fn update<T: DrawTarget<Color = Rgb565>>(&mut self, display: &mut T, input_state: InputState)  {
        let clock = Instant::now();
        // Update the game timer
        self.frame += 1;
        let now = self.time.elapsed().as_millis();

        // Update input from device
        self.input.update(input_state, now, self.frame);

        // Test sprite rendering
        self.assets.sprites.draw_sprite(display, Point::new(32, 50), 104);
        self.assets.sprites.draw_sprite(display, Point::new(48, 50), 105);
        self.assets.sprites.draw_sprite(display, Point::new(32, 66), 120);
        self.assets.sprites.draw_sprite(display, Point::new(48, 66), 121);

        // UI
        self.ui.update(display, &self.input);

        // Print framerate
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        let recstyle = PrimitiveStyleBuilder::new().stroke_color(Rgb565::RED).stroke_width(1).fill_color(Rgb565::BLACK).build();
        let text = format!("R:{:?} Frame:{:?}", clock.elapsed(), self.frame);
        Rectangle::new(Point::new(0, 0), Size::new(DISPLAY_WIDTH, 20)).draw_styled(&recstyle, display);
        Text::new(&*text, Point::new(3, 10), style).draw(display);
    }
    
    fn draw_graphic<T: DrawTarget<Color = Rgb565>>(&self, display: &mut T, pixels: &[u8]) {
        let _ = Image::new(&Bmp::from_slice(pixels).unwrap(), Point::new(0, 0)).draw(display);
    }
}
