use eframe::{egui, epi};

pub struct CalligraphyNotepad {
    text: String,
}

impl Default for CalligraphyNotepad {
    fn default() -> Self {
        Self {
            text: String::from("🖋️ Write your elegant notes here..."),
        }
    }
}

impl epi::App for CalligraphyNotepad {
    fn name(&self) -> &str {
        "Calligraphy Notepad"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        configure_calligraphy_font(ctx);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🖋️ Calligraphy Notepad");
            ui.add_space(10.0);
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .font(egui::TextStyle::Monospace)
                    .desired_rows(20)
                    .desired_width(f32::INFINITY)
                    .text_color(egui::Color32::from_rgb(80, 40, 120)),
            );
        });
    }
}

fn configure_calligraphy_font(ctx: &egui::Context) {
    use egui::{FontData, FontDefinitions, FontFamily::Proportional};

    let mut fonts = FontDefinitions::default();

    // Load custom font
    fonts.font_data.insert(
        "CalligraphyFont".to_owned(),
        FontData::from_static(include_bytes!("GreatVibes-Regular.ttf")),
    );

    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "CalligraphyFont".to_owned());

    ctx.set_fonts(fonts);
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calligraphy Notepad",
        native_options,
        Box::new(|_cc| Box::new(CalligraphyNotepad::default())),
    )
}
