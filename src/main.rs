pub mod actions;
pub mod audio;
#[macro_use]
pub mod couleur;
pub mod entities;
pub mod loader;
pub mod menu;
pub mod player;
pub mod save;
pub mod traits;
pub mod world;

use actions::Action;
use loader::load_from_json;
use menu::{select_from_menu, MenuOption, MenuResult};
use player::Player;
use std::io::{self, Write};
use world::WorldManager;

// séparateur des en-têtes de menu
const SEP_RAW: &str = "--------------------------------------------------";
/// Renvoie le séparateur coloré en magenta.
fn sep() -> String {
    colore!(Magenta, "{}", SEP_RAW)
}

// entité bidon pour le tour de mem::replace (voir interact_with_entity)
struct DummyEntity;
impl entities::Saveable for DummyEntity {}
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

// Sauvegarde l'état du jeu dans data/save.json
fn handle_save(world: &WorldManager) {
    clear_screen();
    match save::save_game(world, "data/save.json") {
        Ok(_) => println!("{}", colore!(Vert, "✓ Jeu sauvegardé avec succès dans 'data/save.json'.")),
        Err(e) => println!("{}", colore!(RougeGras, "✗ Erreur lors de la sauvegarde : {}", e)),
    }
    wait_for_enter();
}

fn confirm_quit(world: &WorldManager) {
    clear_screen();
    print!("{}", colore!(Cyan, "Voulez-vous sauvegarder avant de quitter? (o/n) : "));
    io::stdout().flush().unwrap();
    let mut response = String::new();
    let _ = io::stdin().read_line(&mut response);
    if response.trim().to_lowercase() == "o" {
        match save::save_game(world, "data/save.json") {
            Ok(_) => println!("{}", colore!(Vert, "✓ Jeu sauvegardé avec succès.")),
            Err(e) => println!("{}", colore!(RougeGras, "✗ Erreur lors de la sauvegarde : {}", e)),
        }
    }
    println!("{}", colore!(Jaune, "Au revoir !"));
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
    action_header.push_str(&sep());
    action_header.push('\n');
    action_header.push_str(&format!(
        "{} | {}\n",
        colore!(Cyan, "[Heure : {}]", world.format_time()),
        colore!(Jaune, "[Aura : {:.1}]", world.player.aura),
    ));
    action_header.push_str(&format!("Interaction avec : {}\n", colore!(CyanGras, "{}", name)));
    action_header.push_str(&sep());

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
            handle_save(world);
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
        header.push_str(&sep());
        header.push('\n');
        header.push_str(&format!(
            "{} | {}\n",
            colore!(Cyan, "[Heure : {}]", world.format_time()),
            colore!(Jaune, "[Aura : {:.1}]", world.player.aura),
        ));
        header.push_str(&format!(
            "Lieu : {}\n",
            colore!(CyanGras, "{}", world.zones[zone_idx].interest_points[ip_idx].description),
        ));
        header.push_str(&sep());

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
            MenuResult::Save => handle_save(world),
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
    println!("{}", colore!(JauneGras, "=== Aura Farming Simulator ==="));

    let save_exists = std::path::Path::new("data/save.json").exists();
    let mut use_save = false;

    if save_exists {
        let menu_opts = vec![
            MenuOption::new("Continuer la partie sauvegardée"),
            MenuOption::new("Nouvelle partie (efface la sauvegarde précédente)"),
        ];
        if let Ok(MenuResult::Selected(0)) = select_from_menu(menu_opts, "Une sauvegarde existante a été trouvée. Que voulez-vous faire ?") {
            use_save = true;
        }
    }

    let loaded = match load_from_json("data/world.json") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{}", colore!(RougeGras, "Erreur : impossible de charger le monde : {}", e));
            return;
        }
    };

    let mut world = loaded.world;

    if use_save {
        match save::load_game(&mut world, "data/save.json") {
            Ok(_) => {
                // Chargé avec succès
            }
            Err(e) => {
                println!("{}", colore!(RougeGras, "✗ Erreur lors du chargement de la sauvegarde : {}", e));
                println!("Démarrage d'une nouvelle partie.");
                wait_for_enter();
                use_save = false;
            }
        }
    }

    if !use_save {
        clear_screen();
        println!("{}", colore!(JauneGras, "=== Aura Farming Simulator ==="));
        print!("{}", colore!(Cyan, "Entrez votre nom (par défaut: Jean-Michel) : "));
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
        println!(
            "{}",
            colore!(MagentaGras, "================================================================================"),
        );
        println!("{}", loaded.intro_text.replace("{player_name}", player_name));
        println!(
            "{}\n",
            colore!(MagentaGras, "================================================================================"),
        );
        wait_for_enter();
    }

    loop {
        if world.current_tick >= world.max_ticks {
            clear_screen();
            println!("\n{}", colore!(RougeGras, "20h00 - L'HEURE DE LA DÉFAITE !"));
            println!("Les portes du château se ferment. Le bal commence sans vous.");
            println!(
                "Vous entendez les trompettes au loin alors que vous êtes encore dans la boue."
            );
            println!("Vous passerez le reste de votre vie à sarcler des navets sous la pluie.");
            println!("\n{}", colore!(RougeGras, "=== GAME OVER ==="));
            break;
        }

        // sortir de la maison ne termine plus la partie : on peut explorer la Plaine
        // (la vraie victoire viendra avec la salle du trône)

        // en-tête de la zone
        let mut header = String::new();
        header.push_str(&sep());
        header.push('\n');
        header.push_str(&format!(
            "{} | {}\n",
            colore!(Cyan, "[Heure : {}]", world.format_time()),
            colore!(Jaune, "[Aura : {:.1}]", world.player.aura),
        ));
        header.push_str(&format!(
            "Lieu : {}\n",
            colore!(CyanGras, "{}", world.zones[world.player.zone].description),
        ));

        if !world.player.inventory.is_empty() {
            let inv_names: Vec<String> = world
                .player
                .inventory
                .iter()
                .map(|&id| world.entities[id].name().to_string())
                .collect();
            header.push_str(&format!("Inventaire : {}\n", colore!(Magenta, "{}", inv_names.join(", "))));
        } else {
            header.push_str("Inventaire : Vide\n");
        }
        header.push_str(&sep());

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
            MenuResult::Save => handle_save(&world),
            MenuResult::Quit => {
                confirm_quit(&world);
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
                        confirm_quit(&world);
                        return;
                    }
                }
                ZoneEntry::InterestPoint(ip_idx) => {
                    if let Flow::Quit = enter_interest_point(&mut world, zone_idx, ip_idx) {
                        confirm_quit(&world);
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

    #[test]
    fn test_save_and_load_game() {
        let mut world = load_test_world();

        // Modifie l'état général
        world.current_tick = 120;
        world.player.aura = 15000.0;
        world.player.zone = 1;
        world.player.inventory = vec![2, 7];

        // Modifie les interactables d'une zone
        world.zones[0].interactables = vec![0, 1, 3];

        // Modifie l'état interne d'une entité (Fenetre en index 4) en essayant de traverser la fenêtre fermée
        let mut temp_player = Player {
            aura: 0.0,
            zone: 0,
            inventory: vec![],
        };
        let mut fenetre = std::mem::replace(&mut world.entities[4], Box::new(DummyEntity));
        fenetre.execute_action(&Action::Deplacer { target_zone: 1 }, &mut temp_player, &mut world);
        let _ = std::mem::replace(&mut world.entities[4], fenetre);

        let test_save_path = "data/test_save.json";

        // Sauvegarde
        save::save_game(&world, test_save_path).expect("Sauvegarde échouée");

        // Crée un monde propre
        let mut new_world = load_test_world();

        // Charge l'état
        save::load_game(&mut new_world, test_save_path).expect("Chargement échoué");

        // Nettoie le fichier de test
        let _ = std::fs::remove_file(test_save_path);

        // Vérifications
        assert_eq!(new_world.current_tick, 130);
        assert_eq!(new_world.player.aura, 15000.0);
        assert_eq!(new_world.player.zone, 1);
        assert_eq!(new_world.player.inventory, vec![2, 7]);
        assert_eq!(new_world.zones[0].interactables, vec![0, 1, 3]);

        // Vérifie que l'état interne de l'entité Fenetre a bien été restauré
        let saved_state = new_world.entities[4].save_state();
        assert_eq!(saved_state.get("est_cassee").unwrap().as_bool(), Some(true));
        assert_eq!(saved_state.get("est_ouverte").unwrap().as_bool(), Some(true));
    }
}
