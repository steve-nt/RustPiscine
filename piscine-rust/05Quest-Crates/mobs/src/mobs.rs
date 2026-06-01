pub mod boss;
pub mod member;

use std::collections::{HashMap, HashSet};

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_string(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_power: u64 = self.members.values().map(|m| m.role.power()).sum();
        let other_power: u64 = other.members.values().map(|m| m.role.power()).sum();

        if self_power > other_power {
            remove_youngest(&mut other.members);
            if other.members.is_empty() {
                self.cities.extend(other.cities.drain());
                self.wealth += other.wealth;
                other.wealth = 0;
            }
        } else {
            // self loses on draw or when self_power <= other_power
            remove_youngest(&mut self.members);
            if self.members.is_empty() {
                other.cities.extend(self.cities.drain());
                other.wealth += self.wealth;
                self.wealth = 0;
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let stolen = amount.min(target.wealth);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    pub fn conquer_city(&mut self, others: &[&Mob], city: String) {
        if !others.iter().any(|mob| mob.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }
}

fn remove_youngest(members: &mut HashMap<String, Member>) {
    if let Some(min_age) = members.values().map(|m| m.age).min() {
        members.retain(|_, m| m.age != min_age);
    }
}