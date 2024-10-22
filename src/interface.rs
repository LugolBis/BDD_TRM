// User interface 
use eframe::egui;

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("BDD_TRM", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           //ui.heading("Welcome in BDD_TRM !");
           ui.label(egui::RichText::new("Welcome in BDD_TRM !").font(egui::FontId::proportional(40.0)));
           ui.add_space(15.0);
           ui.label(egui::RichText::new("Texte [...]").font(egui::FontId::proportional(30.0)));
       });
   }
}