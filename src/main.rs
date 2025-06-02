mod app;
mod astrology;
mod ui;
mod utils;

use app::DialApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Uranian Astrology Dial"),
        ..Default::default()
    };

    eframe::run_native(
        "Uranian Astrology",
        options,
        Box::new(|_cc| Box::<DialApp>::default()),
    )
}