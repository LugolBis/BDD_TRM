// This crate contains the tests integration of the project

use std::collections::HashMap;
extern crate BDD_TRM;
use BDD_TRM::*;

#[test]
fn test_new_plan() {
    let mut entities_var = Entities::new() ;
    let mut relationships_var = Relationships::new() ;
    let mut plan_var = Plan::new();
    plan_var.translate(entities_var, relationships_var);
    assert_eq!(plan_var.tables,HashMap::new());
}