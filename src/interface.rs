// User interface 

use eframe::egui;
use BDD_TRM::*;

pub fn run() -> eframe::Result {

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
    let mut show_entities = false;  // groupe 1

    let mut entity_name = String::new(); // groupe 2
    let mut entity_fields = String::new(); // groupe 2
    let mut show_entity = false; // groupe 2 

    let mut new_name = String::new(); // groupe 3
    let mut new_fields = String::new(); // groupe 3
    let mut add_entity = false;  // groupe 3

    let mut delete_name = String::new(); // groupe 4
    let mut delete_output = String::new(); // groupe 4
    let mut delete_entity = false; // groupe 4

    let mut show_relationships = false;  // groupe 1

    let mut relationship_name = String::new(); // groupe 2
    let mut relationship_fields = String::new(); // groupe 2
    let mut show_relationship = false; // groupe 2 

    let mut new_name_r = String::new(); // groupe 3
    let mut new_fields_r = String::new(); // groupe 3
    let mut cardinality_1 = String::new();
    let mut cardinality_2 = String::new();
    let mut entity_1 = String::new();
    let mut entity_2 = String::new();
    let mut add_relationship = false;  // groupe 3

    let mut delete_name_r = String::new(); // groupe 4
    let mut delete_output_r = String::new(); // groupe 4
    let mut delete_relationship = false; // groupe 4

    eframe::run_simple_native("BDD_TRM", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome in BDD_TRM !");
            ui.vertical(|ui| {
                // Entities :
                ui.label("Entities");

                ui.group(|ui| {
                    ui.checkbox(&mut show_entities, "Show entities");
                    if show_entities {
                        for entity in entities.values.keys() {
                            ui.label(format!("-> {}",entity).as_str());
                        }
                        ui.set_invisible(); 
                    }
                });
                ui.group(|ui| {
                    ui.checkbox(&mut show_entity, "Show fields of entity");
                    if show_entity {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut entity_name);
                        ui.label(&entity_fields);
                        if ui.button("Search").clicked() {
                            &entity_fields.truncate(0);
                            if let Some(vector) = entities.values.get(&entity_name) {
                                &entity_fields.push_str(&format!("{:#?}",vector));
                            }
                            else {
                                &entity_fields.push_str(&format!("Entity '{:?}' doen't exist.",&entity_name));
                            }
                        }
                        ui.set_invisible();
                    }
                });
                ui.group(|ui| {
                    ui.checkbox(&mut add_entity, "Add / Alter");
                    if add_entity {
                        ui.label("Name :");
                        ui.text_edit_singleline(&mut new_name);
                        ui.label("Fields (split each field with a ',' and the primary keys need to strat with '#') :");
                        ui.text_edit_singleline(&mut new_fields);
                        if ui.button("Commit").clicked() {
                            entities.values.insert(new_name.clone(),new_fields.split(',').map(|val| val.trim().to_string()).collect());
                        }
                        ui.set_invisible();
                    }
                    
                });
                ui.group(|ui| {
                    ui.checkbox(&mut delete_entity, "Delete");
                    if delete_entity {
                        ui.label("Entity name :");
                        ui.text_edit_singleline(&mut delete_name);
                        ui.label(&delete_output);
                        if ui.button("Delete").clicked() {
                            &delete_output.truncate(0);
                            if let Some(value) = entities.values.remove(&delete_name) {
                                &delete_output.push_str("Successfuly deleted.");
                            }
                            else {
                                &delete_output.push_str(&format!("Entity '{:?}' doen't exist.",&entity_name));
                            }
                        }
                        ui.set_invisible();
                    }   
                });

                // Relationships :
                ui.label("Relationships");

                ui.group(|ui| {
                    ui.checkbox(&mut show_relationships, "Show relationships");
                    if show_relationships {
                        for relationship in relationships.values.keys() {
                            ui.label(format!("-> {}",relationship).as_str());
                        }
                        ui.set_invisible(); 
                    }
                });
                ui.group(|ui| {
                    ui.checkbox(&mut show_relationship, "Show fields/link of relationship");
                    if show_relationship {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut relationship_name);
                        ui.label(&relationship_fields);
                        if ui.button("Search").clicked() {
                            &relationship_fields.truncate(0);
                            if let Some(tuple) = relationships.values.get(&relationship_name) {
                                &relationship_fields.push_str(&format!("Fields : {:#?}\n{:#?}",tuple.0,tuple.1));
                            }
                            else {
                                &relationship_fields.push_str(&format!("Relationship '{:?}' doen't exist.",&relationship_name));
                            }
                        }
                        ui.set_invisible();
                    }
                });
                ui.group(|ui| {
                    ui.checkbox(&mut add_relationship, "Add / Alter");
                    if add_relationship {
                        ui.label("Name :");
                        ui.text_edit_singleline(&mut new_name_r);
                        ui.label("Fields (split each field with a ',' and the primary keys need to strat with '#') :");
                        ui.text_edit_singleline(&mut new_fields_r);
                        ui.label("Link of the relationship");
                        ui.label("Entity 1");
                        ui.text_edit_singleline(&mut entity_1);
                        ui.label("Cardinality 1 (between Entity 1 and the current relationship)");
                        ui.text_edit_singleline(&mut cardinality_1);
                        ui.label("Entity 2");
                        ui.text_edit_singleline(&mut entity_2);
                        ui.label("Cardinality 2 (between Entity 2 and the current relationship)");
                        ui.text_edit_singleline(&mut cardinality_2);
                        if ui.button("Commit").clicked() {
                            relationships.values.insert(new_name_r.clone(),
                            (new_fields_r.split(',').map(|val| val.trim().to_string()).collect(),
                            Link::from((&cardinality_1,&cardinality_2,&entity_1,&entity_2)) ));
                        }
                        ui.set_invisible();
                    }
                    
                });
                ui.group(|ui| {
                    ui.checkbox(&mut delete_relationship, "Delete");
                    if delete_relationship {
                        ui.label("Relationship name :");
                        ui.text_edit_singleline(&mut delete_name_r);
                        ui.label(&delete_output_r);
                        if ui.button("Delete").clicked() {
                            &delete_output_r.truncate(0);
                            if let Some(value) = relationships.values.remove(&delete_name_r) {
                                &delete_output_r.push_str("Successfuly deleted.");
                            }
                            else {
                                &delete_output_r.push_str(&format!("Entity '{:?}' doen't exist.",&delete_name_r));
                            }
                        }
                        ui.set_invisible();
                    }   
                });
            });
        });
    })
}