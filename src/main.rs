pub mod actions;
pub mod audio;
pub mod entities;
pub mod menu;
pub mod player;
pub mod traits;
pub mod world;

use actions::Action;
use menu::{select_from_menu, MenuOption, MenuResult};
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

fn clear_screen() {
    if cfg!(target_os = "windows") {
        if std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .is_err()
        {
            print!("\x1B[2J\x1B[1;1H");
        }
    } else if std::process::Command::new("clear").status().is_err() {
        print!("\x1B[2J\x1B[1;1H");
    }
    let _ = io::stdout().flush();
}

fn wait_for_enter() {
    println!("\nAppuyez sur Entrée pour continuer...");
    let mut dummy = String::new();
    let _ = io::stdin().read_line(&mut dummy);
}

fn main() {
    clear_screen();
    audio::play_music_loop("assets/music.wav");
    println!("\x1B[1;33m=== Aura Farming Simulator ===\x1B[0m");
    print!("\x1B[36mEntrez votre nom (par défaut: Jean-Michel) : \x1B[0m");
    io::stdout().flush().unwrap();

    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).unwrap();
    let player_name = player_name.trim();
    let player_name = if player_name.is_empty() {
        "Jean-Michel"
    } else {
        player_name
    };

    clear_screen();
    // TODO: Ce texte d'introduction et de contexte narratif devrait être extrait et chargé
    // depuis un fichier de données JSON lors de l'implémentation de la séralisation par l'équipe.
    println!(
        "\x1B[1;35m================================================================================\x1B[0m"
    );
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
    println!(
        "\x1B[1;35m================================================================================\x1B[0m\n"
    );
    wait_for_enter();

    let mut world = load_first_zone();

    loop {
        if world.current_tick >= world.max_ticks {
            clear_screen();
            println!("\n\x1B[1;31m20h00 - L'HEURE DE LA DÉFAITE !\x1B[0m");
            println!("Les portes du château se ferment. Le bal commence sans vous.");
            println!(
                "Vous entendez les trompettes au loin alors que vous êtes encore dans la boue."
            );
            println!("Vous passerez le reste de votre vie à sarcler des navets sous la pluie.");
            println!("\n\x1B[1;31m=== GAME OVER ===\x1B[0m");
            break;
        }

        // pour le moment, fin du jeu = sortie zone 0, mais aprés, la fin sera la présence dans la salle du roi pour l'adoubemment
        if world.player.zone != 0 {
            clear_screen();
            println!("\n\x1B[1;32m==================================================\x1B[0m");
            println!("{}", world.zones[world.player.zone].description);
            println!(
                "Votre Aura finale : \x1B[33m{:.1}\x1B[0m",
                world.player.aura
            );
            println!("Heure de fin : \x1B[36m{}\x1B[0m", world.format_time());
            println!("Félicitations, vous avez réussi à sortir de chez vous !");
            println!("(Fin du prototype de la première zone en mémoire)");
            println!("\x1B[1;32m==================================================\x1B[0m");
            break;
        }

        clear_screen();
        println!("\x1B[35m--------------------------------------------------\x1B[0m");
        println!(
            "\x1B[36m[Heure : {}]\x1B[0m | \x1B[33m[Aura : {:.1}]\x1B[0m",
            world.format_time(),
            world.player.aura
        );
        println!(
            "Lieu : \x1B[1;36m{}\x1B[0m",
            world.zones[world.player.zone].description
        );

        if !world.player.inventory.is_empty() {
            let inv_names: Vec<String> = world
                .player
                .inventory
                .iter()
                .map(|&id| world.entities[id].name().to_string())
                .collect();
            println!("Inventaire : \x1B[35m{}\x1B[0m", inv_names.join(", "));
        } else {
            println!("Inventaire : Vide");
        }
        println!("\x1B[35m--------------------------------------------------\x1B[0m");

        let zone_interactables = &world.zones[world.player.zone].interactables;
        if zone_interactables.is_empty() {
            println!("Il n'y a rien d'intéressant ici.");
            break;
        }

        let mut menu_options = Vec::new();
        for &entity_idx in zone_interactables.iter() {
            menu_options.push(MenuOption::new(world.entities[entity_idx].name()));
        }
        menu_options.push(MenuOption::special("Attendre (consomme 15 minutes)"));

        let result = match select_from_menu(menu_options) {
            Ok(res) => res,
            Err(_) => continue,
        };

        match result {
            MenuResult::Save => {
                clear_screen();
                println!("\x1B[32m✓ Jeu en cours de sauvegarde...\x1B[0m");
                wait_for_enter();
                continue;
            }
            MenuResult::Quit => {
                clear_screen();
                print!("\x1B[36mVoulez-vous sauvegarder avant de quitter? (o/n) : \x1B[0m");
                io::stdout().flush().unwrap();
                let mut response = String::new();
                let _ = io::stdin().read_line(&mut response);
                if response.trim().to_lowercase() == "o" {
                    println!("\x1B[32m✓ Jeu sauvegardé.\x1B[0m");
                }
                println!("\x1B[33mAu revoir !\x1B[0m");
                return;
            }
            MenuResult::Selected(choix_idx) => {
                if choix_idx == zone_interactables.len() {
                    world.current_tick += 15;
                    println!("Vous attendez en regardant le plafond. 15 minutes s'écoulent...");
                    wait_for_enter();
                    continue;
                }

                let chosen_entity_idx = zone_interactables[choix_idx];
        let actions = world.entities[chosen_entity_idx].get_actions(&world.player, &world);

        clear_screen();
        println!("\x1B[35m--------------------------------------------------\x1B[0m");
        println!(
            "\x1B[36m[Heure : {}]\x1B[0m | \x1B[33m[Aura : {:.1}]\x1B[0m",
            world.format_time(),
            world.player.aura
        );
        println!(
            "Interaction avec : \x1B[1;36m{}\x1B[0m",
            world.entities[chosen_entity_idx].name()
        );
        println!("\x1B[35m--------------------------------------------------\x1B[0m");

        let mut menu_options = Vec::new();
        for action in actions.iter() {
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
            menu_options.push(MenuOption::new(label));
        }
        menu_options.push(MenuOption::special("Retour"));

        let action_result = match select_from_menu(menu_options) {
            Ok(res) => res,
            Err(_) => continue,
        };

        match action_result {
            MenuResult::Save => {
                clear_screen();
                println!("\x1B[32m✓ Jeu en cours de sauvegarde...\x1B[0m");
                wait_for_enter();
                continue;
            }
            MenuResult::Quit => {
                clear_screen();
                print!("\x1B[36mVoulez-vous sauvegarder avant de quitter? (o/n) : \x1B[0m");
                io::stdout().flush().unwrap();
                let mut response = String::new();
                let _ = io::stdin().read_line(&mut response);
                if response.trim().to_lowercase() == "o" {
                    println!("\x1B[32m✓ Jeu sauvegardé.\x1B[0m");
                }
                println!("\x1B[33mAu revoir !\x1B[0m");
                return;
            }
            MenuResult::Selected(action_idx) => {
                if action_idx == actions.len() {
                    continue;
                }

                let chosen_action = &actions[action_idx];

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

                wait_for_enter();
            }
        }
            }
        }
    }
}

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
