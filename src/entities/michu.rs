use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Michu {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub biscuit_id: usize,
    pub broche_id: usize,
    pub chapeau_id: usize,
    pub biscuit_donne: bool,
}

impl Saveable for Michu {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("biscuit_donne".to_string(), serde_json::json!(self.biscuit_donne));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("biscuit_donne") {
            if let Some(b) = val.as_bool() {
                self.biscuit_donne = b;
            }
        }
    }
}

impl Interactable for Michu {
    fn id(&self) -> usize {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }

    fn get_actions(&self, _player: &Player, _world: &WorldManager) -> Vec<Action> {
        vec![Action::Dialoguer]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            // plusieurs sujets de conversation → on ouvre un sous-menu
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Parler"),
                    MenuOption::new("Demander conseil"),
                    MenuOption::new("Demander un biscuit"),
                    MenuOption::new("Offrir un objet"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que dire à Michu ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 5;
                        println!("« Le roi ? Ah oui, il adoube les mardis et jeudis. Faut prendre rendez-vous. Et surtout, faut pas sentir le chou. »");
                    }
                    1 => {
                        world.current_tick += 5;
                        println!("« La forêt au nord cache un vieux chêne et un ermite bavard. Et le lac à l'est... méfie-toi de la barque. » (indices notés)");
                    }
                    2 => {
                        world.current_tick += 3;
                        if self.biscuit_donne {
                            println!("« C'est pas un buffet ici ! »");
                        } else {
                            self.biscuit_donne = true;
                            player.inventory.push(self.biscuit_id);
                            player.aura += 10000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Tiens, mon grand. » Les biscuits de Michu donnent du courage.", colore!(Vert, "[+10 000 Aura]"));
                        }
                    }
                    3 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.chapeau_id) {
                            player.inventory.retain(|&x| x != self.chapeau_id);
                            player.inventory.push(self.broche_id);
                            player.aura += 50000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Oh ! Je cherchais ce chapeau pour mes poules depuis des années ! Tiens, prends cette broche. »", colore!(Vert, "[+50 000 Aura]"));
                        } else {
                            println!("« C'est gentil mais non merci. Je ne suis pas Emmaüs. »");
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("Michu vous regarde par-dessus ses lunettes."),
        }
    }
}
