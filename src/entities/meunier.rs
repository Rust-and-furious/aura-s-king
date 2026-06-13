use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Meunier {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub marmite_id: usize,
    pub farine_id: usize,
    pub travail_donne: bool,
}

impl Saveable for Meunier {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("travail_donne".to_string(), serde_json::json!(self.travail_donne));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("travail_donne") {
            if let Some(b) = val.as_bool() {
                self.travail_donne = b;
            }
        }
    }
}

impl Interactable for Meunier {
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
        vec![Action::Observer, Action::Dialoguer]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => println!("{}", self.description),
            // plusieurs sujets de conversation → on ouvre un sous-menu
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Parler"),
                    MenuOption::new("Demander du travail"),
                    MenuOption::new("Offrir un objet"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que dire au meunier ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 3;
                        println!("« Encore un va-nu-pieds qui veut devenir chevalier ? Reviens quand t'auras de quoi payer. »");
                    }
                    1 => {
                        world.current_tick += 5;
                        // une seule livraison à confier : pas une rente d'aura
                        if self.travail_donne {
                            println!("« J'ai plus rien à te faire livrer pour aujourd'hui, gamin. »");
                        } else {
                            self.travail_donne = true;
                            player.aura += 5000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} Le meunier vous confie une livraison de sacs de farine pour le village. Quel dévouement.", colore!(Vert, "[+5 000 Aura]"));
                        }
                    }
                    2 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.marmite_id) {
                            player.inventory.retain(|&x| x != self.marmite_id);
                            player.inventory.push(self.farine_id);
                            player.aura += 30000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Ah, une marmite ! Parfaite pour ma soupe. Tiens, prends cette farine enchantée ! »", colore!(Vert, "[+30 000 Aura]"));
                        } else {
                            println!("« Qu'est-ce que tu veux que je fasse de ça ? »");
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("Le meunier hausse les épaules."),
        }
    }
}
