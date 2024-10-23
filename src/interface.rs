// User interface 

use std::fmt::Debug;
use eframe::egui;
use BDD_TRM::*;

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
           ui.label(egui::RichText::new("How to use [...]").font(egui::FontId::proportional(30.0)));
       });
   }
}

pub fn run2() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 850.0]),
        ..Default::default()
    };

    // Our application state:
    let mut entities = Entities::new();
    entities.values.insert("Voiture".to_string(), vec!["#NVEH".to_string(),"couleur".to_string()]);
    entities.values.insert("Garage".to_string(), vec!["#adresse".to_string(),"nom".to_string()]);
    let mut relationships = Relationships::new();
    let mut plan = Plan::new();

    // Entities :
    let mut entities_name = String::new();  // groupe 1
    let mut show_entities = false;  // groupe 1

    let mut new_entity_name = String::new(); // groupe 2
    let mut new_entity_fields = String::new(); // groupe 2
    let mut add_entities = false;  // groupe 2

    eframe::run_simple_native("BDD_TRM", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome in BDD_TRM !");
            ui.horizontal(|ui| {
                ui.label("Entities");

                //ui.add_space(20.0);
                //ui.spacing();
                ui.label("\n");
                ui.group(|ui| {
                    ui.checkbox(&mut show_entities, "Show");
                    if !show_entities {
                        for entity in entities.values.keys() {
                            if !entities_name.contains(entity) {
                            entities_name.push_str(format!("\n-> {}",entity).as_str()); 
                            }
                        }
                        ui.set_invisible();
                    }
                    ui.label(&entities_name);  
                });
                ui.group(|ui| {
                    ui.checkbox(&mut add_entities, "Add");
                    if !add_entities {
                        ui.set_invisible();
                    }
                    let new_name = ui.label("Name");
                    ui.text_edit_singleline(&mut new_entity_name)
                                .labelled_by(new_name.id);
                    let new_fields = ui.label("Fields (split each field with a ',' and the primary keys need to strat with '#')");
                    ui.text_edit_singleline(&mut new_entity_fields)
                                .labelled_by(new_fields.id);
                    if ui.button("Submit").clicked() {
                        entities.values.insert(new_entity_name.clone(),new_entity_fields.split(',').map(|elem| elem.to_string()).collect());
                    }
                });
                ui.group(|ui| {
                    ui.button("Alter");
                });
                ui.group(|ui| {
                    ui.button("Delete");
                });
            });

            /*ui.text_edit_singleline(&mut name)
                                .labelled_by(name_label.id);*/

            /*ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}")); */
        });
    })
}

/* 
Notes -- la structure Layout peut permettre de centrer le texte

*/