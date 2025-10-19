use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Calculator", 
        options, 
        Box::new(|cc| Ok(Box::new(Calculator::new(cc))))
    )
}

#[derive(Default)]
struct Calculator {

}

impl Calculator {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self{
        Calculator::default()
    }
}

impl eframe::App for Calculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello!");
        });
    }
}