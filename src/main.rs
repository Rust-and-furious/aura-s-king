pub mod entities;
pub mod player;
pub mod world;

use entities::{Furniture, Interactable, Npc, Objet};
use player::Player;
use world::WorldManager;

fn main() {
    println!("Essaie d'obtenir assez d'aura pour vaincre le roi Antony ! :D");

    let mut world = WorldManager {
        zones: vec![],
        entities: vec![],
    };
    let mut player = Player {
        aura: 100.0,
        zone: 0,
        inventory: vec![],
    };

    let mut king = Npc {
        name: "Antony".to_string(),
        description: "Le roi en personne.".to_string(),
        is_hostile: true,
    };

    let mut chest = Furniture {
        name: "Coffre".to_string(),
        description: "Un vieux coffre en bois couvert de poussière.".to_string(),
        durability: 10,
    };

    let mut sword = Objet {
        name: "Épée en fer".to_string(),
        description: "Une simple épée en fer.".to_string(),
        weight: 5,
        durability: 100,
    };

    println!("\n--- Méthodes spécifiques ---");
    king.dialoguer(&mut player, &mut world);
    println!("Observation: {}", chest.observer());
    chest.fouiller(&mut player, &mut world);
    sword.ramasser(&mut player, &mut world);

    println!("\n--- Appel polymorphe via Interactable ---");
    let mut interactables: Vec<Box<dyn Interactable>> = vec![
        Box::new(Npc {
            name: "Garde".to_string(),
            description: "Un garde au regard sévère.".to_string(),
            is_hostile: false,
        }),
        Box::new(Furniture {
            name: "Table".to_string(),
            description: "Une table de banquet.".to_string(),
            durability: 20,
        }),
        Box::new(Objet {
            name: "Clé".to_string(),
            description: "Une clé rouillée.".to_string(),
            weight: 1,
            durability: 50,
        }),
    ];

    for it in &mut interactables {
        println!("- {} : {}", it.name(), it.description());
        it.interagir(&mut player, &mut world);
    }
}
