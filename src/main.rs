pub mod actions;
pub mod audio;
pub mod entities;
pub mod loader;
pub mod menu;
pub mod player;
pub mod traits;
pub mod world;

use actions::Action;
use loader::load_from_json;
use menu::{select_from_menu, MenuOption, MenuResult};
use player::Player;
use std::io::{self, Write};
use world::WorldManager;

// séparateur des en-têtes de menu
const SEP: &str = "\x1B[35m--------------------------------------------------\x1B[0m";

// entité bidon pour le tour de mem::replace (voir interact_with_entity)
struct DummyEntity;
impl entities::Interactable for DummyEntity {
    fn id(&self) -> usize {
        usize::MAX
    }
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

// faut-il continuer la boucle de jeu, ou quitter ?
enum Flow {
    Continue,
    Quit,
}

// une ligne du menu d'une zone : un objet, un point d'intérêt, ou "Attendre"
#[derive(Clone, Copy)]
enum ZoneEntry {
    Interactable(usize),  // index dans world.entities
    InterestPoint(usize), // index dans world.zones[zone].interest_points
    Wait,
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

// la sauvegarde n'est pas encore implémentée, on affiche juste un message
fn handle_save() {
    clear_screen();
    println!("\x1B[32m✓ Jeu en cours de sauvegarde...\x1B[0m");
    wait_for_enter();
}

fn confirm_quit() {
    clear_screen();
    print!("\x1B[36mVoulez-vous sauvegarder avant de quitter? (o/n) : \x1B[0m");
    io::stdout().flush().unwrap();
    let mut response = String::new();
    let _ = io::stdin().read_line(&mut response);
    if response.trim().to_lowercase() == "o" {
        println!("\x1B[32m✓ Jeu sauvegardé.\x1B[0m");
    }
    println!("\x1B[33mAu revoir !\x1B[0m");
}

fn action_label(action: &Action) -> String {
    match action {
        Action::Observer => "Observer".to_string(),
        Action::Utiliser => "Utiliser (Dormir / Bricoler / etc.)".to_string(),
        Action::Ramasser => "Ramasser (Prendre)".to_string(),
        Action::Ouvrir => "Ouvrir".to_string(),
        Action::Fermer => "Fermer".to_string(),
        Action::Attaquer { degats } => format!("Attaquer (Enfoncer / Casser, dégâts: {})", degats),
        Action::Deplacer { target_zone: _ } => "Passer / Traverser / Sauter".to_string(),
        _ => format!("{:?}", action),
    }
}

// affiche les actions d'une entité et exécute le choix (appelée depuis une zone ou un point d'intérêt)
fn interact_with_entity(world: &mut WorldManager, entity_idx: usize) -> Flow {
    let actions = world.entities[entity_idx].get_actions(&world.player, world);
    let name = world.entities[entity_idx].name().to_string();

    let mut action_header = String::new();
    action_header.push_str(SEP);
    action_header.push('\n');
    action_header.push_str(&format!(
        "\x1B[36m[Heure : {}]\x1B[0m | \x1B[33m[Aura : {:.1}]\x1B[0m\n",
        world.format_time(),
        world.player.aura
    ));
    action_header.push_str(&format!("Interaction avec : \x1B[1;36m{}\x1B[0m\n", name));
    action_header.push_str(SEP);

    let mut menu_options = Vec::new();
    for action in actions.iter() {
        menu_options.push(MenuOption::new(action_label(action)));
    }
    menu_options.push(MenuOption::special("Retour"));

    let action_result = match select_from_menu(menu_options, &action_header) {
        Ok(res) => res,
        Err(_) => return Flow::Continue,
    };

    match action_result {
        MenuResult::Cancelled => Flow::Continue,
        MenuResult::Save => {
            handle_save();
            Flow::Continue
        }
        MenuResult::Quit => Flow::Quit,
        MenuResult::Selected(action_idx) => {
            if action_idx == actions.len() {
                return Flow::Continue; // « Retour »
            }
            let chosen_action = &actions[action_idx];

            // on sort l'entité (et le joueur) du world pour pouvoir prêter world entier
            // à execute_action, puis on les remet à leur place juste après
            let mut entity =
                std::mem::replace(&mut world.entities[entity_idx], Box::new(DummyEntity));
            let mut temp_player = std::mem::replace(
                &mut world.player,
                Player {
                    aura: 0.0,
                    zone: 0,
                    inventory: vec![],
                },
            );
            entity.execute_action(chosen_action, &mut temp_player, world);
            let _ = std::mem::replace(&mut world.player, temp_player);
            let _ = std::mem::replace(&mut world.entities[entity_idx], entity);

            wait_for_enter();
            Flow::Continue
        }
    }
}

// boucle dans un point d'intérêt (liste ses objets) jusqu'au choix "Retour"
fn enter_interest_point(world: &mut WorldManager, zone_idx: usize, ip_idx: usize) -> Flow {
    loop {
        let mut header = String::new();
        header.push_str(SEP);
        header.push('\n');
        header.push_str(&format!(
            "\x1B[36m[Heure : {}]\x1B[0m | \x1B[33m[Aura : {:.1}]\x1B[0m\n",
            world.format_time(),
            world.player.aura
        ));
        header.push_str(&format!(
            "Lieu : \x1B[1;36m{}\x1B[0m\n",
            world.zones[zone_idx].interest_points[ip_idx].description
        ));
        header.push_str(SEP);

        // on clone la liste (évite un emprunt de world pendant le menu, et reflète les objets ramassés)
        let entity_ids: Vec<usize> = world.zones[zone_idx].interest_points[ip_idx]
            .interactables
            .clone();

        if entity_ids.is_empty() {
            clear_screen();
            println!("{}", header);
            println!("\nIl n'y a plus rien d'intéressant ici.");
            wait_for_enter();
            return Flow::Continue;
        }

        let mut menu_options = Vec::new();
        for &eidx in &entity_ids {
            menu_options.push(MenuOption::new(world.entities[eidx].name()));
        }
        menu_options.push(MenuOption::special("Retour"));

        let result = match select_from_menu(menu_options, &header) {
            Ok(res) => res,
            Err(_) => return Flow::Continue,
        };

        match result {
            MenuResult::Cancelled => return Flow::Continue,
            MenuResult::Save => handle_save(),
            MenuResult::Quit => return Flow::Quit,
            MenuResult::Selected(sel) => {
                if sel == entity_ids.len() {
                    return Flow::Continue; // « Retour »
                }
                // On reste dans le point d'intérêt après l'interaction.
                if let Flow::Quit = interact_with_entity(world, entity_ids[sel]) {
                    return Flow::Quit;
                }
            }
        }
    }
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

    let loaded = match load_from_json("data/world.json") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("\x1B[1;31mErreur : impossible de charger le monde : {e}\x1B[0m");
            return;
        }
    };

    clear_screen();
    println!(
        "\x1B[1;35m================================================================================\x1B[0m"
    );
    println!("{}", loaded.intro_text.replace("{player_name}", player_name));
    println!(
        "\x1B[1;35m================================================================================\x1B[0m\n"
    );
    wait_for_enter();

    let mut world = loaded.world;

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

        // sortir de la maison ne termine plus la partie : on peut explorer la Plaine
        // (la vraie victoire viendra avec la salle du trône)

        // en-tête de la zone
        let mut header = String::new();
        header.push_str(SEP);
        header.push('\n');
        header.push_str(&format!(
            "\x1B[36m[Heure : {}]\x1B[0m | \x1B[33m[Aura : {:.1}]\x1B[0m\n",
            world.format_time(),
            world.player.aura
        ));
        header.push_str(&format!(
            "Lieu : \x1B[1;36m{}\x1B[0m\n",
            world.zones[world.player.zone].description
        ));

        if !world.player.inventory.is_empty() {
            let inv_names: Vec<String> = world
                .player
                .inventory
                .iter()
                .map(|&id| world.entities[id].name().to_string())
                .collect();
            header.push_str(&format!("Inventaire : \x1B[35m{}\x1B[0m\n", inv_names.join(", ")));
        } else {
            header.push_str("Inventaire : Vide\n");
        }
        header.push_str(SEP);

        // menu de la zone : objets directs + points d'intérêt + "Attendre"
        let zone_idx = world.player.zone;
        let interactable_ids: Vec<usize> = world.zones[zone_idx].interactables.clone();
        let ip_count = world.zones[zone_idx].interest_points.len();

        if interactable_ids.is_empty() && ip_count == 0 {
            println!("Il n'y a rien d'intéressant ici.");
            break;
        }

        let mut entries: Vec<ZoneEntry> = Vec::new();
        let mut menu_options: Vec<MenuOption> = Vec::new();

        for &eidx in &interactable_ids {
            menu_options.push(MenuOption::new(world.entities[eidx].name()));
            entries.push(ZoneEntry::Interactable(eidx));
        }
        for ip_idx in 0..ip_count {
            let label = format!(
                "» {}",
                world.zones[zone_idx].interest_points[ip_idx].description
            );
            menu_options.push(MenuOption::new(label));
            entries.push(ZoneEntry::InterestPoint(ip_idx));
        }
        menu_options.push(MenuOption::special("Attendre (consomme 15 minutes)"));
        entries.push(ZoneEntry::Wait);

        let result = match select_from_menu(menu_options, &header) {
            Ok(res) => res,
            Err(_) => continue,
        };

        match result {
            MenuResult::Cancelled => continue,
            MenuResult::Save => handle_save(),
            MenuResult::Quit => {
                confirm_quit();
                return;
            }
            MenuResult::Selected(sel) => match entries[sel] {
                ZoneEntry::Wait => {
                    world.current_tick += 15;
                    println!("Vous attendez en regardant le plafond. 15 minutes s'écoulent...");
                    wait_for_enter();
                }
                ZoneEntry::Interactable(eidx) => {
                    if let Flow::Quit = interact_with_entity(&mut world, eidx) {
                        confirm_quit();
                        return;
                    }
                }
                ZoneEntry::InterestPoint(ip_idx) => {
                    if let Flow::Quit = enter_interest_point(&mut world, zone_idx, ip_idx) {
                        confirm_quit();
                        return;
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_world() -> WorldManager {
        load_from_json("data/world.json")
            .expect("Le fichier data/world.json doit être présent et valide pour les tests")
            .world
    }

    #[test]
    fn test_format_time() {
        let mut world = load_test_world();
        assert_eq!(world.format_time(), "08h00");
        world.current_tick = 90;
        assert_eq!(world.format_time(), "09h30");
        world.current_tick = 720;
        assert_eq!(world.format_time(), "20h00");
    }

    #[test]
    fn test_lit_sleep_advances_time_and_changes_aura() {
        let mut world = load_test_world();
        let mut temp_player = std::mem::replace(
            &mut world.player,
            Player {
                aura: 0.0,
                zone: 0,
                inventory: vec![],
            },
        );
        // entities[0] est le Lit (lit_joueur, premier du tableau JSON)
        let mut lit = std::mem::replace(&mut world.entities[0], Box::new(DummyEntity));

        lit.execute_action(&Action::Utiliser, &mut temp_player, &mut world);

        assert!(world.current_tick >= 60 && world.current_tick <= 120);
        assert!(temp_player.aura == 10000.0 || temp_player.aura == -30000.0);
    }
}
