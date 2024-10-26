// User interface 

use eframe::egui;
use crate::api::*;
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;

const ICON_DATA_BYTES: &[u8] = include_bytes!("../icon.txt"); // Extract the bytes from the file

#[derive(Debug)]
pub struct UiEntities {
    pub show_all: bool,        // groupe 1
    
    pub show_one: bool,        // groupe 2 
    pub name: String,          // groupe 2
    pub fields: String,        // groupe 2
    
    pub add_one: bool,         // groupe 3
    pub new_name: String,      // groupe 3
    pub new_fields: String,    // groupe 3
    
    pub delete_one: bool,      // groupe 4 
    pub delete_name: String,   // groupe 4
    pub delete_output: String, // groupe 4
}

#[derive(Debug)]
pub struct UiRelationships {
    pub show_all: bool,        // groupe 1
    
    pub show_one: bool,        // groupe 2 
    pub name: String,          // groupe 2
    pub fields: String,        // groupe 2
    
    pub add_one: bool,         // groupe 3
    pub new_name: String,      // groupe 3
    pub new_fields: String,    // groupe 3
    pub new_link: (String,String,String,String), // groupe 3
    
    pub delete_one: bool,      // groupe 4 
    pub delete_name: String,   // groupe 4
    pub delete_output: String, // groupe 4
}

impl UiEntities {
    pub fn new() -> Self {
        Self{show_all: false, show_one: false, name: String::new(), fields: String::new(), add_one: false, new_name: String::new(),
            new_fields: String::new(), delete_one: false, delete_name: String::new(), delete_output: String::new()}
    }
}

impl UiRelationships {
    pub fn new() -> Self {
        Self{show_all: false, show_one: false, name: String::new(), fields: String::new(), add_one: false, new_name: String::new(),
            new_fields: String::new(), new_link: (String::new(),String::new(),String::new(),String::new()),
            delete_one: false, delete_name: String::new(), delete_output: String::new()}
    }
}

pub fn run() -> eframe::Result {
    let icon_vec  = ICON_DATA_BYTES.to_vec();
    let icon = Arc::new(egui::viewport::IconData{rgba: icon_vec, width: 256u32, height: 256u32});
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder{inner_size:Some([1080.0, 850.0].into()),resizable:Some(true), icon:icon.into(),..Default::default()},
        centered:false,..Default::default()
    };

    const FONT_TITLE: egui::FontId = egui::FontId::proportional(25.0);
    const FONT_USUAL: egui::FontId = egui::FontId::proportional(15.0);
    
    let mut entities = Entities::new();
    let mut relationships = Relationships::new();

    let mut show_information = false;
    let mut ui_e = UiEntities::new();
    let mut ui_r = UiRelationships::new();
    let mut tables = String::new();

    eframe::run_simple_native("BDD_TRM", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let visual = egui::style::Visuals{
                dark_mode:true,
                panel_fill: egui::Color32::from_rgb(50u8, 82u8, 105u8),..Default::default() //
            };
            ctx.set_pixels_per_point(2.0);
            ctx.set_visuals(visual);
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Welcome in BDD_TRM !").font(FONT_TITLE));   
                ui.add_space(15.0);
                if ui.button(egui::RichText::new("How to use ?").font(FONT_USUAL)).clicked() {
                    show_information = !show_information;
                }
                ui.group(|ui| {
                    if show_information {
                        ui.label(egui::RichText::new("- The primary keys need to start with an '#'\n
                        - The foreign keys start with an '##'\n
                        - The Entities/Relationships needs to be split with a ','").font(FONT_USUAL));
                        if ui.button(egui::RichText::new("Hide").font(FONT_USUAL)).clicked() {
                            show_information = false;
                        }
                        ui.set_invisible(); 
                    }
                }); 
                ui.add_space(15.0);
                // Entities
                ui.label(egui::RichText::new("Entities").font(FONT_TITLE));
                ui.add_space(15.0);
                ui.checkbox(&mut ui_e.show_all, egui::RichText::new("Show entities").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_e.show_all {
                        for entity in entities.values.keys() {
                            ui.label(egui::RichText::new(format!("-> {}",entity).as_str()).font(FONT_USUAL));
                        }
                        ui.set_invisible(); 
                    }
                }); 
                ui.add_space(5.0);
                ui.checkbox(&mut ui_e.show_one, egui::RichText::new("Show fields of entity").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_e.show_one {
                        ui.add_space(1.0);
                        ui.label(egui::RichText::new("Name").font(FONT_USUAL)); ui.add_space(1.0);
                        ui.text_edit_singleline(&mut ui_e.name);
                        ui.label(egui::RichText::new(&ui_e.fields).font(FONT_USUAL));
                        if ui.button(egui::RichText::new("Search").font(FONT_USUAL)).clicked() {
                            &ui_e.fields.truncate(0);
                            if let Some(vector) = entities.values.get(ui_e.name.trim()) {
                                &ui_r.fields.push_str("Fields :\n");
                                for entity in vector {
                                    &ui_e.fields.push_str(&format!("{}, ",entity));
                                }
                            }
                            else {
                                &ui_e.fields.push_str(&format!("Entity '{:?}' doen't exist.",&ui_e.name));
                            }
                        }
                        ui.set_invisible();
                    }
                });
                ui.add_space(5.0);
                ui.checkbox(&mut ui_e.add_one, egui::RichText::new("Add / Alter").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_e.add_one {
                        ui.label(egui::RichText::new("Name :").font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_e.new_name);
                        ui.label(egui::RichText::new("Fields :")
                            .font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_e.new_fields);
                        if ui.button(egui::RichText::new("Commit").font(FONT_USUAL)).clicked() {
                            entities.values.insert(ui_e.new_name.trim().to_string(),ui_e.new_fields.split(',')
                            .map(|val| val.trim().to_string()).collect());
                        }
                        ui.set_invisible();
                    }
                    
                });
                ui.add_space(5.0);
                ui.checkbox(&mut ui_e.delete_one, egui::RichText::new("Delete").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_e.delete_one {
                        ui.label(egui::RichText::new("Entity name :").font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_e.delete_name);
                        ui.label(egui::RichText::new(&ui_e.delete_output).font(FONT_USUAL));
                        if ui.button(egui::RichText::new("Delete").font(FONT_USUAL)).clicked() {
                            &ui_e.delete_output.truncate(0);
                            if let Some(value) = entities.values.remove(ui_e.delete_name.trim()) {
                                &ui_e.delete_output.push_str("Successfuly deleted.");
                            }
                            else {
                                &ui_e.delete_output.push_str(&format!("Entity '{:?}' doen't exist.",&ui_e.delete_name));
                            }
                        }
                        ui.set_invisible();
                    }   
                });
                ui.add_space(15.0);
                // Relationships :
                ui.label(egui::RichText::new("Relationships").font(FONT_TITLE));
                ui.add_space(5.0);
                ui.checkbox(&mut ui_r.show_all, egui::RichText::new("Show relationships").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_r.show_all {
                        for relationship in relationships.values.keys() {
                            ui.label(egui::RichText::new(format!("-> {}",relationship).as_str()).font(FONT_USUAL));
                        }
                        ui.set_invisible(); 
                    }
                });
                ui.add_space(5.0);
                ui.checkbox(&mut ui_r.show_one, egui::RichText::new("Show fields/link of relationship").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_r.show_one {
                        ui.label(egui::RichText::new("Name").font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.name);
                        ui.label(egui::RichText::new(&ui_r.fields).font(FONT_USUAL));
                        if ui.button(egui::RichText::new("Search").font(FONT_USUAL)).clicked() {
                            &ui_r.fields.truncate(0);
                            if let Some(tuple) = relationships.values.get(ui_r.name.trim()) {
                                &ui_r.fields.push_str("Fields :\n");
                                for field in &tuple.0 {
                                    &ui_r.fields.push_str(&format!("{}, ",field));
                                }
                                &ui_r.fields.push_str(&format!("\n{:?}",tuple.1));
                            }
                            else {
                                &ui_r.fields.push_str(&format!("Relationship '{:?}' doen't exist.",&ui_r.name));
                            }
                        }
                        ui.set_invisible();
                    }
                });
                ui.add_space(5.0);
                ui.checkbox(&mut ui_r.add_one, egui::RichText::new("Add / Alter").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_r.add_one {
                        ui.label(egui::RichText::new("Name").font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.new_name);
                        ui.label(egui::RichText::new("Fields :")
                            .font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.new_fields);
                        ui.label(egui::RichText::new("Link of the relationship").font(FONT_USUAL));
                        ui.label(egui::RichText::new("Entity 1 :").font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.new_link.2);
                        ui.label( egui::RichText::new("Cardinality 1 :")
                            .font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.new_link.0);
                        ui.label(egui::RichText::new("Entity 2 :").font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.new_link.3);
                        ui.label( egui::RichText::new("Cardinality 2 :")
                            .font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.new_link.1);
                        if ui.button(egui::RichText::new("Commit").font(FONT_USUAL)).clicked() {
                            relationships.values.insert(ui_r.new_name.trim().to_string(),
                            (ui_r.new_fields.split(',').map(|val| val.trim().to_string()).collect(),
                            Link::from_string(ui_r.new_link.clone())));
                        }
                        ui.set_invisible();
                    }
                });
                ui.add_space(5.0);
                ui.checkbox(&mut ui_r.delete_one, egui::RichText::new("Delete").font(FONT_USUAL));
                ui.group(|ui| {
                    if ui_r.delete_one {
                        ui.label(egui::RichText::new("Relationship name :").font(FONT_USUAL));
                        ui.text_edit_singleline(&mut ui_r.delete_name);
                        ui.label(egui::RichText::new(&ui_r.delete_output).font(FONT_USUAL));
                        if ui.button(egui::RichText::new("Delete").font(FONT_USUAL)).clicked() {
                            &ui_r.delete_output.truncate(0);
                            if let Some(value) = relationships.values.remove(ui_r.delete_name.trim()) {
                                &ui_r.delete_output.push_str("Successfuly deleted.");
                            }
                            else {
                                &ui_r.delete_output.push_str(&format!("Entity '{:?}' doen't exist.",&ui_r.delete_name));
                            }
                        }
                        ui.set_invisible();
                    }   
                });
                ui.add_space(15.0);
                // Plan - Translate to the relational model
                ui.label(egui::RichText::new("Tables").font(FONT_TITLE));
                ui.add_space(5.0);
                if ui.button(egui::RichText::new("Translate to relationnal model").font(FONT_USUAL)).clicked() {
                    tables.truncate(0);
                    let mut plan = Plan::new();
                    plan.translate(entities.clone(), relationships.clone());
                    for table in plan.tables {
                        tables.push_str(&format!("{} : (",table.0));
                        for field in table.1 {
                            tables.push_str(&format!("{},",field));
                        }
                        tables.pop(); tables.push_str(")\n");
                    }
                }
                ui.add_space(4.0);
                ui.label( egui::RichText::new(&tables).font(FONT_USUAL));
            });
        });
        });
    })
}