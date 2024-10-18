// This crate convert E/R model to Relationnal model

use std::collections::HashMap;

#[derive(Debug)]
pub struct Link(bool,bool,String,String);

#[derive(Debug)]
pub struct Entities {
    values: HashMap<String, Vec<String>>
}

#[derive(Debug)]
pub struct Relationships {
    values: HashMap<String, (Vec<String>, Link)>
}  

#[derive(Debug)]
pub struct Plan {
    tables: HashMap<String, Vec<String>>
}  

impl Entities {
    fn from(couples:Vec<(&str,Vec<&str>)>) -> Self {
        let mut values: HashMap<String, Vec<String>> = HashMap::new();
        for couple in couples {
            values.insert(couple.0.to_string(), 
            couple.1.into_iter().map(|elem| elem.to_string()).collect());
        }
        Self {values:values}
    }
}

impl Relationships {
    fn from(couples:Vec<(&str,(Vec<&str>, (bool,bool,&str,&str)))>) -> Self {
        let mut values: HashMap<String, (Vec<String>, Link)> = HashMap::new();
        for couple in couples {
            values.insert(couple.0.to_string(),
            (couple.1.0.into_iter().map(|elem| elem.to_string()).collect(),
            Link(couple.1.1.0,couple.1.1.1,couple.1.1.2.to_string(),couple.1.1.3.to_string())) );    // Good luck to understand !
        }
        Self {values:values}
    }
}

impl Plan {
    fn new() -> Self {
        Self {tables: HashMap::new()}
    }

    fn one_to_one(&mut self,relationship:(String, (Vec<String>,Link))) {
        let entity_1 = relationship.1.1.2;
        let entity_2 = relationship.1.1.3;
        if entity_1==entity_2 {
            if let Some(vector) = self.tables.get_mut(&entity_1) {
                vector.extend_from_slice(&relationship.1.0);
            }
            else {
                panic!("The entity : {} isn't in the 'Entities' arg from the method '.translate()'",entity_1)
            }
        } else {
            let mut attribute: Vec<String> = Vec::new();
            match self.tables.get(&entity_1) {
                Some(vector_1) => {
                    attribute = vector_1.into_iter()
                    .filter_map(|value: &String| {
                        if value.starts_with("#") && (value.find("##")==None) { Some(format!("##{}_{}", entity_1, &value[1..]).to_string()) }
                        else { None } }).collect();
                },
                None => panic!("The entity : {} isn't in the 'Entities' arg from the method '.translate()'",entity_1)
            }
            attribute.extend_from_slice(&relationship.1.0);     // We add all the fields from the relationship
            if let Some(vector_2) = self.tables.get_mut(&entity_2) {
                vector_2.extend_from_slice(&attribute);
            }
            else {
                panic!("The entity : {} isn't in the 'Entities' arg from the method '.translate()'",entity_2)
            }
        }
    }

    fn one_to_many(&mut self,relationship:(String, (Vec<String>,Link))) {
        let mut attribute: Vec<String> = Vec::new();
        let entity_1 = relationship.1.1.2;
        let entity_2 = relationship.1.1.3;
        if relationship.1.1.1==true {
            match self.tables.get(&entity_2) {
                Some(vector_2) => {
                    attribute = vector_2.into_iter()
                    .filter_map(|value: &String| {
                        if value.starts_with("#") && (value.find("##")==None) { Some(format!("##{}_{}", entity_2, &value[1..]).to_string()) }
                        else { None } }).collect();
                },
                None=> panic!("The entity : {} doesn't exist in 'Plan.entities'.",entity_2)
            }
            attribute.extend_from_slice(&relationship.1.0);
            if let Some(vector_1) = self.tables.get_mut(&entity_1) {
                vector_1.extend_from_slice(&attribute);
            } else {
                panic!("The entity : {} doesn't exist in 'Plan.entities'.",entity_1)
            }
        } else {
            match self.tables.get(&entity_1) {
                Some(vector_1) => {
                    attribute = vector_1.into_iter()
                    .filter_map(|value: &String| {
                        if value.starts_with("#") && (value.find("##")==None) { Some(format!("##{}_{}", entity_1, &value[1..]).to_string()) }
                        else { None } }).collect();
                },
                None=> panic!("The entity : {} doesn't exist in 'Plan.entities'.",entity_1)
            }
            attribute.extend_from_slice(&relationship.1.0);
            if let Some(vector_2) = self.tables.get_mut(&entity_2) {
                vector_2.extend_from_slice(&attribute);
            } else {
                panic!("The entity : {} doesn't exist in 'Plan.entities'.",entity_2)
            }
        }
    }

    fn many_to_many(&mut self,relationship:(String, (Vec<String>,Link))) {
        let mut attribute: Vec<String> = Vec::new();
        let entity_1 = relationship.1.1.2;
        let entity_2 = relationship.1.1.3;
        if entity_1==entity_2 {
            match self.tables.get(&entity_1) {
                Some(vector_1) => {
                    for value in vector_1 {
                        if value.starts_with("#") && (value.find("##")==None) {
                            attribute.push(format!("##{}_{}1", entity_1, &value[1..]).to_string());
                            attribute.push(format!("##{}_{}2", entity_1, &value[1..]).to_string());
                        }
                    }
                },
                None=> panic!("The entity : {} doesn't exist in 'Plan.entities'.",entity_1)
            }
        }
        else {
            match self.tables.get(&entity_1) {
                Some(vector_1) => {
                    attribute = vector_1.into_iter()
                        .filter_map(|value: &String| {
                            if value.starts_with("#") && (value.find("##")==None) { Some(format!("##{}_{}", entity_1, &value[1..]).to_string()) }
                            else { None } }).collect();
                },
                None=> panic!("The entity : {} doesn't exist in 'Plan.entities'.",entity_1)
            }
            match self.tables.get(&entity_2) {
                Some(vector_2) => {
                    let other_values: Vec<String> = vector_2.into_iter()
                        .filter_map(|value: &String| {
                            if value.starts_with("#") && (value.find("##")==None) { Some(format!("##{}_{}", entity_2, &value[1..]).to_string()) }
                            else { None } }).collect();
                    attribute.extend_from_slice(&other_values);
                },
                None=> panic!("The entity : {} doesn't exist in 'Plan.entities'.",entity_2)
            }
        }
        attribute.extend_from_slice(&relationship.1.0);
        self.tables.insert(relationship.0, attribute);
    }

    fn translate(&mut self, entities: Entities, relationships: Relationships) {
        for entity in entities.values {
            self.tables.insert(entity.0,entity.1);
        }
        for relationship in relationships.values {
            let cardinality = (relationship.1.1.0,relationship.1.1.1);
            match cardinality {
                (true,true) => self.many_to_many(relationship),
                (true,false) => self.one_to_many(relationship),
                (false,true) => self.one_to_many(relationship),
                (false,false) => self.one_to_one(relationship),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

// README
// This crate permite to convert an Entity/Relationship (E/R) to a Relationnal model.
// DISCLAIMER : It only support E/R model where there is only binary relationship.