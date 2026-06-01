use ::mobs::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut corleones = Mob {
        name: "Corleones".to_string(),
        boss: Boss::new("Vito", 60),
        members: HashMap::new(),
        cities: HashSet::new(),
        wealth: 1000,
    };

    let mut barzinis = Mob {
        name: "Barzinis".to_string(),
        boss: Boss::new("Emilio", 55),
        members: HashMap::new(),
        cities: HashSet::new(),
        wealth: 500,
    };

    corleones.recruit(("Sonny", 30));
    corleones.recruit(("Fredo", 25));
    corleones.recruit(("Tom", 28));

    barzinis.recruit(("Pete", 22));
    barzinis.recruit(("Carlo", 35));

    println!("Before attack:");
    println!("Corleones members: {}", corleones.members.len());
    println!("Barzinis members: {}", barzinis.members.len());

    corleones.attack(&mut barzinis);

    println!("\nAfter attack (Corleones attack Barzinis):");
    println!("Corleones members: {}", corleones.members.len());
    println!("Barzinis members: {}", barzinis.members.len());

    corleones.steal(&mut barzinis, 300);
    println!("\nAfter steal:");
    println!("Corleones wealth: {}", corleones.wealth);
    println!("Barzinis wealth: {}", barzinis.wealth);

    let others = vec![&barzinis];
    corleones.conquer_city(&others, "New York".to_string());
    println!("\nCorleones cities: {:?}", corleones.cities);

    let mut member = Member { role: Role::Associate, age: 25 };
    member.get_promotion();
    println!("\nPromoted member role: {:?}", member.role);
}