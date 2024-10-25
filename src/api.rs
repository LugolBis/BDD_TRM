// This crate convert E/R model to Relationnal model

use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Link(bool,bool,String,String);

#[derive(Debug,Clone)]
pub struct Entities {
    pub values: HashMap<String, Vec<String>>
}

#[derive(Debug,Clone)]
pub struct Relationships {
    pub values: HashMap<String, (Vec<String>, Link)>
}  

#[derive(Debug,Clone)]
pub struct Plan {
    pub tables: HashMap<String, Vec<String>>
}

impl Link {
    pub fn from(input:(&str,&str,&str,&str)) -> Self {
        let mut cardinality_1 = true;
        let mut cardinality_2 = true;
        if input.0.ends_with(":1") { cardinality_1 = false; }
        if input.1.ends_with(":1") { cardinality_2 = false; }
        Self(cardinality_1,cardinality_2,input.2.to_string(),input.3.to_string())
    }

    pub fn from_string(input:(String,String,String,String)) -> Self {
        let mut cardinality_1 = true;
        let mut cardinality_2 = true;
        if input.0.ends_with(":1") { cardinality_1 = false; }
        if input.1.ends_with(":1") { cardinality_2 = false; }
        Self(cardinality_1,cardinality_2,input.2.to_string(),input.3.to_string())
    }
}

impl Entities {
    pub fn new() -> Self {
        Self{values:HashMap::new()}
    }

    pub fn from(couples:Vec<(&str,Vec<&str>)>) -> Self {
        let mut values: HashMap<String, Vec<String>> = HashMap::new();
        for couple in couples {
            values.insert(couple.0.to_string(), 
            couple.1.into_iter().map(|elem| elem.to_string()).collect());
        }
        Self {values:values}
    }
}

impl Relationships {
    pub fn new() -> Self {
        Self{values:HashMap::new()}
    }
    
    pub fn from(couples:Vec<(&str,(Vec<&str>, (bool,bool,&str,&str)))>) -> Self {
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
    pub fn new() -> Self {
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
                self.tables.insert(relationship.0,vec![format!("ERROR : {} doesn't exist.",entity_1)]);
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
                None => attribute.push(format!("ERROR : {} doesn't exist.",entity_1))
            }
            attribute.extend_from_slice(&relationship.1.0);     // We add all the fields from the relationship
            if let Some(vector_2) = self.tables.get_mut(&entity_2) {
                vector_2.extend_from_slice(&attribute);
            }
            else {
                attribute.push(format!("ERROR : {} doesn't exist.",entity_2));
                self.tables.insert(relationship.0,attribute);
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
                None=> attribute.push(format!("ERROR : {} doesn't exist.",entity_2))
            }
            attribute.extend_from_slice(&relationship.1.0);
            if let Some(vector_1) = self.tables.get_mut(&entity_1) {
                vector_1.extend_from_slice(&attribute);
            } else {
                attribute.push(format!("ERROR : {} doesn't exist.",entity_1));
                self.tables.insert(relationship.0,attribute);
            }
        } else {
            match self.tables.get(&entity_1) {
                Some(vector_1) => {
                    attribute = vector_1.into_iter()
                    .filter_map(|value: &String| {
                        if value.starts_with("#") && (value.find("##")==None) { Some(format!("##{}_{}", entity_1, &value[1..]).to_string()) }
                        else { None } }).collect();
                },
                None=> attribute.push(format!("ERROR : {} doesn't exist.",entity_1))
            }
            attribute.extend_from_slice(&relationship.1.0);
            if let Some(vector_2) = self.tables.get_mut(&entity_2) {
                vector_2.extend_from_slice(&attribute);
            } else {
                attribute.push(format!("ERROR : {} doesn't exist.",entity_2));
                self.tables.insert(relationship.0,attribute);
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
                None=> attribute.push(format!("ERROR : {} doesn't exist.",entity_1))
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
                None=> attribute.push(format!("ERROR : {} doesn't exist.",entity_1))
            }
            match self.tables.get(&entity_2) {
                Some(vector_2) => {
                    let other_values: Vec<String> = vector_2.into_iter()
                        .filter_map(|value: &String| {
                            if value.starts_with("#") && (value.find("##")==None) { Some(format!("##{}_{}", entity_2, &value[1..]).to_string()) }
                            else { None } }).collect();
                    attribute.extend_from_slice(&other_values);
                },
                None=> attribute.push(format!("ERROR : {} doesn't exist.",entity_2))
            }
        }
        attribute.extend_from_slice(&relationship.1.0);
        self.tables.insert(relationship.0, attribute);
    }

    pub fn translate(&mut self, entities: Entities, relationships: Relationships) {
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