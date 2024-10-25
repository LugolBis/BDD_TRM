// This crate contains the tests integration of the project

#[cfg(test)]
use std::collections::HashMap;
use crate::api::*;

#[test]
fn test_one_to_one_same() {
    let entities_var = Entities::from(vec![
        ("Voiture",vec!["#NVEH","Model","Année"]),
        ("Garage", vec!["km","etat","#adresse"]) ]);
    let relationships_var = Relationships::from(vec![
        ("Stocké",(vec!["date"],(false,false,"Voiture","Voiture")))
    ]);
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var,relationships_var);
    println!("{:?}",plan_var.tables);
    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    result.insert("Voiture".to_string(),vec!["#NVEH".to_string(), "Model".to_string(), "Année".to_string(), "date".to_string()]);
    result.insert("Garage".to_string(),vec!["km".to_string(), "etat".to_string(), "#adresse".to_string()]);
    assert!(plan_var.tables==result);
}

#[test]
fn test_one_to_one_other() {
    let entities_var = Entities::from(vec![
        ("Voiture",vec!["#NVEH","Model","Année"]),
        ("Garage", vec!["km","etat","#adresse"]) ]);
    let relationships_var = Relationships::from(vec![
        ("Stocké",(vec!["date"],(false,false,"Garage","Voiture")))
    ]);
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var,relationships_var);
    println!("{:?}",plan_var.tables);
    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    result.insert("Voiture".to_string(),vec!["#NVEH".to_string(), "Model".to_string(), "Année".to_string(), "##Garage_adresse".to_string(), "date".to_string()]);
    result.insert("Garage".to_string(),vec!["km".to_string(), "etat".to_string(), "#adresse".to_string()]);
    assert!(plan_var.tables==result);
}

#[test]
fn test_one_to_many() {
    let entities_var = Entities::from(vec![
        ("Voiture",vec!["#NVEH","Model","Année"]),
        ("Garage", vec!["km","etat","#adresse"]) ]);
    let relationships_var = Relationships::from(vec![
        ("Stocké",(vec!["date"],(false,true,"Garage","Voiture")))
    ]);
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var,relationships_var);
    println!("{:?}",plan_var.tables);
    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    result.insert("Voiture".to_string(),vec!["#NVEH".to_string(), "Model".to_string(), "Année".to_string()]);
    result.insert("Garage".to_string(),vec!["km".to_string(), "etat".to_string(), "#adresse".to_string(), "##Voiture_NVEH".to_string(), "date".to_string()]);
    assert!(plan_var.tables==result);
}

#[test]
fn test_many_to_one() {
    let entities_var = Entities::from(vec![
        ("Voiture",vec!["#NVEH","Model","Année"]),
        ("Garage", vec!["km","etat","#adresse"]) ]);
    let relationships_var = Relationships::from(vec![
        ("Stocké",(vec!["date"],(true,false,"Garage","Voiture")))
    ]);
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var,relationships_var);
    println!("{:?}",plan_var.tables);
    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    result.insert("Voiture".to_string(),vec!["#NVEH".to_string(), "Model".to_string(), "Année".to_string(), "##Garage_adresse".to_string(), "date".to_string()]);
    result.insert("Garage".to_string(),vec!["km".to_string(), "etat".to_string(), "#adresse".to_string()]);
    assert!(plan_var.tables==result);
}

#[test]
fn test_one_to_many_same() {
    let entities_var = Entities::from(vec![
        ("Voiture",vec!["#NVEH","Model","Année"]),
        ("Garage", vec!["km","etat","#adresse"]) ]);
    let relationships_var = Relationships::from(vec![
        ("Stocké",(vec!["date"],(false,true,"Voiture","Voiture")))
    ]);
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var,relationships_var);
    println!("{:?}",plan_var.tables);
    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    result.insert("Voiture".to_string(),vec!["#NVEH".to_string(), "Model".to_string(), "Année".to_string(), "##Voiture_NVEH".to_string(), "date".to_string()]);
    result.insert("Garage".to_string(),vec!["km".to_string(), "etat".to_string(), "#adresse".to_string()]);
    assert!(plan_var.tables==result);
}

#[test]
fn test_many_to_many() {
    let entities_var = Entities::from(vec![
        ("Voiture",vec!["#NVEH","Model","Année"]),
        ("Garage", vec!["km","etat","#adresse"]) ]);
    let relationships_var = Relationships::from(vec![
        ("Stocké",(vec!["date"],(true,true,"Garage","Voiture")))
    ]);
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var,relationships_var);
    println!("{:?}",plan_var.tables);
    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    result.insert("Voiture".to_string(),vec!["#NVEH".to_string(), "Model".to_string(), "Année".to_string()]);
    result.insert("Garage".to_string(),vec!["km".to_string(), "etat".to_string(), "#adresse".to_string()]);
    result.insert("Stocké".to_string(),vec!["##Garage_adresse".to_string(), "##Voiture_NVEH".to_string(), "date".to_string()]);
    assert!(plan_var.tables==result);
}

#[test]
fn test_many_to_many_same() {
    let entities_var = Entities::from(vec![
        ("Voiture",vec!["#NVEH","Model","Année"]),
        ("Garage", vec!["km","etat","#adresse"]) ]);
    let relationships_var = Relationships::from(vec![
        ("Stocké",(vec!["date"],(true,true,"Voiture","Voiture")))
    ]);
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var,relationships_var);
    println!("{:?}",plan_var.tables);
    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    result.insert("Voiture".to_string(),vec!["#NVEH".to_string(), "Model".to_string(), "Année".to_string()]);
    result.insert("Garage".to_string(),vec!["km".to_string(), "etat".to_string(), "#adresse".to_string()]);
    result.insert("Stocké".to_string(),vec!["##Voiture_NVEH1".to_string(), "##Voiture_NVEH2".to_string(), "date".to_string()]);
    assert!(plan_var.tables==result);
}