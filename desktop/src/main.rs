use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Beans",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<BeanGameApp>::default()
        }),
    )
}

struct BeanGameApp {
    name: String,
}

impl Default for BeanGameApp {
    fn default() -> Self {
        Self {
            name: "New Game".to_string(),
        }
    }
}

impl eframe::App for BeanGameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("We got an egg!");
            ui.horizontal(|ui| {
                let name_label = ui.label("Name the little guy: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
        });
    }
}
