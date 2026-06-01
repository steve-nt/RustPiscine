pub mod mall;
pub use mall::*;

use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    let mut biggest = None;
    let mut max_sqm = 0u64;
    
    for (store_name, store) in mall.floors.values().flat_map(|f| f.stores.iter()) {
        if store.square_meters > max_sqm {
            max_sqm = store.square_meters;
            biggest = Some((store_name, store));
        }
    }
    
    biggest.unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut all_employees: Vec<(&String, &Employee)> = Vec::new();
    
    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, emp) in &store.employees {
                all_employees.push((name, emp));
            }
        }
    }
    
    if all_employees.is_empty() {
        return Vec::new();
    }
    
    let max_salary = all_employees.iter()
        .map(|(_, e)| e.salary)
        .fold(f64::NEG_INFINITY, f64::max);
    
    let mut result = Vec::new();
    for (name, emp) in all_employees {
        if (emp.salary - max_salary).abs() < f64::EPSILON {
            result.push((name, emp));
        }
    }
    
    result.sort_by(|a, b| a.0.cmp(b.0));
    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = mall.guards.len();
    
    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            count += store.employees.len();
        }
    }
    
    count
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total_sqm = 0u64;
    for floor in mall.floors.values() {
        total_sqm += floor.size_limit;
    }
    
    let needed = (total_sqm / 200) as usize;
    let current = mall.guards.len();
    
    if current < needed {
        let to_add = needed - current;
        let mut added = 0;
        
        for (name, guard) in guards {
            if added >= to_add {
                break;
            }
            if !mall.guards.contains_key(&name) {
                mall.guards.insert(name, guard);
                added += 1;
            }
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for emp in store.employees.values_mut() {
                let hours = emp.working_hours.1 - emp.working_hours.0;
                let adjustment = emp.salary * 0.10;
                
                if hours >= 10 {
                    emp.raise(adjustment);
                } else {
                    emp.cut(adjustment);
                }
            }
        }
    }
}