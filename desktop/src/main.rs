use eframe::egui;
use egui::{Key, Sense};
use tiny_bean_boi_lib::{Game, InputState};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tiny Bean Boi",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<AppWrapper>::default()
        }),
    )
}

struct AppWrapper {
    game: Game,
    input_state: InputState,
}


impl Default for AppWrapper {
    fn default() -> Self {
        Self {
            game: Game::default(),
            input_state: InputState::default(),
        }
    }
}

impl eframe::App for AppWrapper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Just a little guy!");

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
        });
        let output_state = self.game.update(self.input_state);
    }
}
