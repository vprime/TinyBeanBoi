
use std::convert::Infallible;

use eframe::egui;
use egui::{ColorImage, Key, Sense, TextureHandle};

use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    pixelcolor::Rgb888,
    primitives::Rectangle,
};

use tiny_bean_boi_lib::{Game, InputState};


fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tiny Bean Boi",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<AppWrapper>::new(AppWrapper::new())
        }),
    )
}

const DISPLAY_WIDTH: u32 = 135;
const DISPLAY_HEIGHT: u32 = 240;
const DISPLAY_WIDTH_INCLUSIVE: u32 = DISPLAY_WIDTH - 1;
const DISPLAY_HEIGHT_INCLUSIVE: u32 = DISPLAY_HEIGHT - 1;

struct GameDisplay {
    image: [u8; (DISPLAY_WIDTH * DISPLAY_HEIGHT * 3) as usize],
}

impl GameDisplay {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let texture: TextureHandle = ui.ctx().load_texture(
            "game-display",
            ColorImage::from_rgb([DISPLAY_WIDTH as usize, DISPLAY_HEIGHT as usize], &self.image),
            Default::default()
        );

        // Show the image:
        ui.image((texture.id(), texture.size_vec2()));
    }
}

impl Dimensions for GameDisplay {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT))
    }
}

impl DrawTarget for GameDisplay {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item=Pixel<Self::Color>>
    {
        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok((x @ 0..=DISPLAY_WIDTH_INCLUSIVE, y @ 0..=DISPLAY_HEIGHT_INCLUSIVE))
                = coord.try_into() {
                let index: u32 = (x + (y * DISPLAY_WIDTH)) * 3;
                let full_color: Rgb888 = color.into();
                self.image[index as usize] = full_color.r();
                self.image[(index + 1) as usize] = full_color.g();
                self.image[(index + 2) as usize] = full_color.b();
            }
        }
        Ok(())
    }
}


struct AppWrapper {
    display: GameDisplay,
    game: Game,
    input_state: InputState,
}


impl AppWrapper {
    fn new() -> Self {
        Self {
            display: GameDisplay {
                image: [0; (DISPLAY_WIDTH * DISPLAY_HEIGHT * 3) as usize],
            },
            game: Game::default(),
            input_state: InputState::default(),
        }
    }
}

impl eframe::App for AppWrapper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.display.ui(ui);

            ui.horizontal(|ui| {
                let left_button = ui.add(egui::Button::new("Left").sense(Sense::drag()));
                let right_button = ui.add(egui::Button::new("Right").sense(Sense::drag()));

                let mut new_state = InputState {
                    left: left_button.dragged(),
                    right: right_button.dragged(),
                };

                if !new_state.left && !new_state.right {
                    new_state.left = ui.input(|i| i.key_down(Key::ArrowLeft));
                    new_state.right = ui.input(|i| i.key_down(Key::ArrowRight));
                }
                self.input_state = new_state;
            });
            ui.heading("Debug");
            ctx.texture_ui(ui);
        });
        let output_state = self.game.update(&mut self.display, self.input_state);

    }
}
