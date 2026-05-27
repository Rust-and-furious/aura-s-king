// main.rs — Point d'entrée. Démontre le Command Pattern en action.

pub mod actions;
pub mod entities;
pub mod player;
pub mod traits;
pub mod world;

use entities::{Fenetre, Garde, Pomme};
use player::Player;
use world::{InterestPoint, WorldManager, Zone};

fn main() {
    println!("=== Aura Farming Simulator ===");
    println!("Essaie d'obtenir assez d'aura pour vaincre le roi Antony !\n");

    // --- Création du monde ---
    let mut world = WorldManager {
        current_tick: 0,
        max_ticks: 100,
        player: Player {
            aura: 50.0,
            zone: 0,
            inventory: vec![],
        },
        zones: vec![
            Zone {
                id: 0,
                description: "La salle du trône, froide et imposante.".to_string(),
                interest_points: vec![
                    InterestPoint {
                        id: 0,
                        description: "Une fenêtre donnant sur les jardins.".to_string(),
                        interactables: vec![0], // ID de la Fenetre dans entities
                    },
                ],
                interactables: vec![1, 2], // IDs du Garde et de la Pomme
                connected_zones: vec![],
            },
        ],
        entities: vec![
            Box::new(Fenetre {
                name: "Fenêtre de la salle du trône".to_string(),
                description: "Une grande fenêtre en verre soufflé.".to_string(),
                est_ouverte: false,
                est_cassee: false,
            }),
            Box::new(Garde {
                name: "Garde royal".to_string(),
                description: "Un garde en armure dorée.".to_string(),
                hp: 100,
                is_hostile: true,
            }),
            Box::new(Pomme {
                name: "Pomme d'aura".to_string(),
                description: "Une pomme qui irradie d'énergie mystique.".to_string(),
                aura_rendue: 25.0,
            }),
        ],
    };

    // --- Démo du Command Pattern ---
    // 1. On regarde ce qu'on peut faire avec l'entité à l'index 0 (la Fenêtre)
    println!("--- Interaction avec : {} ---", world.entities[0].name());
    let actions = world.entities[0].get_actions(&world.player, &world);
    println!("Actions disponibles :");
    for (i, action) in actions.iter().enumerate() {
        println!("  [{}] {:?}", i, action);
    }

    // 2. Le joueur choisit l'action "Ouvrir" (index 1)
    let choix = &actions[1];
    println!("\nVous choisissez : {:?}", choix);
    // On doit re-emprunter mutablement pour exécuter (borrow checker)
    let (player, entities) = (&mut world.player, &mut world.entities);
    entities[0].execute_action(choix, player, &mut WorldManager {
        current_tick: 0,
        max_ticks: 100,
        player: Player { aura: 0.0, zone: 0, inventory: vec![] },
        zones: vec![],
        entities: vec![],
    });

    // 3. On mange la pomme (index 2) pour gagner de l'aura
    println!("\n--- Interaction avec : {} ---", world.entities[2].name());
    let actions_pomme = world.entities[2].get_actions(&world.player, &world);
    println!("Actions disponibles :");
    for (i, action) in actions_pomme.iter().enumerate() {
        println!("  [{}] {:?}", i, action);
    }
    let choix_pomme = &actions_pomme[2]; // Action::Utiliser
    println!("\nVous choisissez : {:?}", choix_pomme);
    println!("Aura avant : {}", world.player.aura);
    let (player2, entities2) = (&mut world.player, &mut world.entities);
    entities2[2].execute_action(choix_pomme, player2, &mut WorldManager {
        current_tick: 0,
        max_ticks: 100,
        player: Player { aura: 0.0, zone: 0, inventory: vec![] },
        zones: vec![],
        entities: vec![],
    });
    println!("Aura après  : {}", world.player.aura);
}
