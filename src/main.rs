pub mod actions;
pub mod entities;
pub mod player;
pub mod traits;
pub mod world;

use actions::Action;
use player::Player;
use std::io::{self, Write};
use world::{WorldManager, load_first_zone};

struct DummyEntity;
impl entities::Interactable for DummyEntity {
    fn name(&self) -> &str {
        ""
    }
    fn description(&self) -> &str {
        ""
    }
    fn get_actions(&self, _player: &Player, _world: &WorldManager) -> Vec<Action> {
        vec![]
    }
    fn execute_action(
        &mut self,
        _action: &Action,
        _player: &mut Player,
        _world: &mut WorldManager,
    ) {
    }
}

fn main() {
    println!("=== Aura Farming Simulator ===");
    print!("Entrez votre nom (par défaut: Jean-Michel) : ");
    io::stdout().flush().unwrap();

    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).unwrap();
    let player_name = player_name.trim();
    let player_name = if player_name.is_empty() {
        "Jean-Michel"
    } else {
        player_name
    };

    // TODO: Ce texte d'introduction et de contexte narratif devrait être extrait et chargé
    // depuis un fichier de données JSON lors de l'implémentation de la séralisation.
    println!("\n================================================================================");
    println!(
        "Vous êtes {}, un paysan dont la lignée est connue depuis sept générations pour",
        player_name
    );
    println!("une seule chose : la culture intensive de légumes oubliés. Votre famille a fourni");
    println!("au royaume assez de navets pour nourrir une armée, mais n'a jamais reçu en retour");
    println!("qu'une dette fiscale et des ampoules aux mains.");
    println!("\nLe Royaume est dirigé par le Roi Anthony, un souverain dont la bonté n'a d'égale");
    println!("que son besoin viscéral d'être impressionné. Pour sortir de votre condition et");
    println!(
        "devenir enfin 'Chevalier', il ne suffit pas d'être courageux. Il faut être légendaire."
    );
    println!("\nVotre unique monnaie d'échange est l'Aura. Attention, le Grand Bal d'adoubement");
    println!("aura lieu ce soir à 20h00 précises. Ne soyez pas en retard !");
    println!("================================================================================\n");

    let mut world = load_first_zone();

    loop {
        if world.current_tick >= world.max_ticks {
            println!("\n⏱️ 20h00 - L'HEURE DE LA DÉFAITE !");
            println!("Les portes du château se ferment. Le bal commence sans vous.");
            println!(
                "Vous entendez les trompettes au loin alors que vous êtes encore dans la boue."
            );
            println!("Vous passerez le reste de votre vie à sarcler des navets sous la pluie.");
            println!("\n=== GAME OVER ===");
            break;
        }

        if world.player.zone != 0 {
            println!("\n==================================================");
            println!("{}", world.zones[world.player.zone].description);
            println!("Votre Aura finale : {:.1}", world.player.aura);
            println!("Heure de fin : {}", world.format_time());
            println!("Félicitations, vous avez réussi à sortir de chez vous !");
            println!("(Fin du prototype de la première zone en mémoire)");
            println!("==================================================");
            break;
        }

        println!("\n--------------------------------------------------");
        println!(
            "[Heure : {}] | [Aura : {:.1}]",
            world.format_time(),
            world.player.aura
        );
        println!("Lieu : {}", world.zones[world.player.zone].description);

        if !world.player.inventory.is_empty() {
            let inv_names: Vec<String> = world
                .player
                .inventory
                .iter()
                .map(|&id| world.entities[id].name().to_string())
                .collect();
            println!("Inventaire : {}", inv_names.join(", "));
        } else {
            println!("Inventaire : Vide");
        }
        println!("--------------------------------------------------");

        let zone_interactables = &world.zones[world.player.zone].interactables;
        if zone_interactables.is_empty() {
            println!("Il n'y a rien d'intéressant ici.");
            break;
        }

        println!("Que voulez-vous observer ou manipuler ?");
        for (i, &entity_idx) in zone_interactables.iter().enumerate() {
            println!("  [{}] {}", i + 1, world.entities[entity_idx].name());
        }
        println!("  [0] Attendre (consomme 15 minutes)");

        print!("Votre choix : ");
        io::stdout().flush().unwrap();

        let mut choix_input = String::new();
        io::stdin().read_line(&mut choix_input).unwrap();
        let choix_idx = match choix_input.trim().parse::<usize>() {
            Ok(val) => val,
            Err(_) => {
                println!("Choix invalide.");
                continue;
            }
        };

        if choix_idx == 0 {
            world.current_tick += 15;
            println!("Vous attendez en regardant le plafond. 15 minutes s'écoulent...");
            continue;
        }

        if choix_idx > zone_interactables.len() {
            println!("Choix invalide.");
            continue;
        }

        let chosen_entity_idx = zone_interactables[choix_idx - 1];
        let actions = world.entities[chosen_entity_idx].get_actions(&world.player, &world);

        println!(
            "\nActions pour '{}' :",
            world.entities[chosen_entity_idx].name()
        );
        for (i, action) in actions.iter().enumerate() {
            let label = match action {
                Action::Observer => "Observer".to_string(),
                Action::Utiliser => "Utiliser (Dormir / Bricoler / etc.)".to_string(),
                Action::Ramasser => "Ramasser (Prendre)".to_string(),
                Action::Ouvrir => "Ouvrir".to_string(),
                Action::Fermer => "Fermer".to_string(),
                Action::Attaquer { degats } => {
                    format!("Attaquer (Enfoncer / Casser, dégâts: {})", degats)
                }
                Action::Deplacer { target_zone: _ } => "Passer / Traverser / Sauter".to_string(),
                _ => format!("{:?}", action),
            };
            println!("  [{}] {}", i + 1, label);
        }
        println!("  [0] Retour");

        print!("Votre action : ");
        io::stdout().flush().unwrap();

        let mut action_input = String::new();
        io::stdin().read_line(&mut action_input).unwrap();
        let action_idx = match action_input.trim().parse::<usize>() {
            Ok(val) => val,
            Err(_) => {
                println!("Choix d'action invalide.");
                continue;
            }
        };

        if action_idx == 0 {
            continue;
        }

        if action_idx > actions.len() {
            println!("Choix d'action invalide.");
            continue;
        }

        let chosen_action = &actions[action_idx - 1];

        // Résolution du borrow checker : swap temporaire de l'entité et du joueur
        let mut entity = std::mem::replace(
            &mut world.entities[chosen_entity_idx],
            Box::new(DummyEntity),
        );
        let mut temp_player = std::mem::replace(
            &mut world.player,
            Player {
                aura: 0.0,
                zone: 0,
                inventory: vec![],
            },
        );
        entity.execute_action(chosen_action, &mut temp_player, &mut world);
        let _ = std::mem::replace(&mut world.player, temp_player);
        let _ = std::mem::replace(&mut world.entities[chosen_entity_idx], entity);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Tests unitaires basiques pour les fonctions critiques du WorldManager et des entités.
/// Permet de vérifier que le formatage de l'heure fonctionne correctement et que les actions sur les entités modifient bien l'état du monde et du joueur.
/// Note : Ces tests sont très basiques et servent surtout de point de départ pour une suite de tests plus complète à l'avenir.
////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        let mut world = load_first_zone();
        assert_eq!(world.format_time(), "08h00");
        world.current_tick = 90;
        assert_eq!(world.format_time(), "09h30");
        world.current_tick = 720;
        assert_eq!(world.format_time(), "20h00");
    }

    #[test]
    fn test_lit_sleep_advances_time_and_changes_aura() {
        let mut world = load_first_zone();
        let mut temp_player = std::mem::replace(
            &mut world.player,
            Player {
                aura: 0.0,
                zone: 0,
                inventory: vec![],
            },
        );
        let mut lit = std::mem::replace(&mut world.entities[0], Box::new(DummyEntity));

        lit.execute_action(&Action::Utiliser, &mut temp_player, &mut world);

        assert!(world.current_tick >= 60 && world.current_tick <= 120);
        assert!(temp_player.aura == 10000.0 || temp_player.aura == -30000.0);
    }
}
