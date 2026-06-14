use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Chene {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub oeuf_id: usize,
    pub deja_grimpe: bool,
    pub deja_enlace: bool,
    pub deja_grave: bool,
}

impl Saveable for Chene {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("deja_grimpe".to_string(), serde_json::json!(self.deja_grimpe));
        state.insert("deja_enlace".to_string(), serde_json::json!(self.deja_enlace));
        state.insert("deja_grave".to_string(), serde_json::json!(self.deja_grave));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("deja_grimpe") {
            if let Some(b) = val.as_bool() {
                self.deja_grimpe = b;
            }
        }
        if let Some(val) = state.get("deja_enlace") {
            if let Some(b) = val.as_bool() {
                self.deja_enlace = b;
            }
        }
        if let Some(val) = state.get("deja_grave") {
            if let Some(b) = val.as_bool() {
                self.deja_grave = b;
            }
        }
    }
}

impl Useable for Chene {
    // trois façons d'aborder le chêne → on demande laquelle via un sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Grimper"),
            MenuOption::new("Enlacer l'arbre"),
            MenuOption::new("Graver son nom"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "Le vieux chêne") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 30;
                if jet_reussite(world.current_tick, 60) {
                    // le nid (et son œuf doré) ne se trouve qu'une fois
                    if self.deja_grimpe {
                        println!("Vous remontez. La vue est toujours aussi belle, mais le nid est vide désormais.");
                    } else {
                        self.deja_grimpe = true;
                        player.inventory.push(self.oeuf_id);
                        player.aura += 150000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!("{} La vue sur le royaume est magnifique, un moment digne des chansons de geste ! Et vous trouvez un œuf doré dans un nid !", colore!(Vert, "[+150 000 Aura]"));
                    }
                } else {
                    player.aura -= 25000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Vous tombez dans un buisson de ronces. Un écureuil vient casser sa noisette sur votre front.", colore!(Rouge, "[-25 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 5;
                // le réconfort de la première étreinte ne se reproduit pas (pas une rente d'aura)
                if self.deja_enlace {
                    println!("Vous enlacez de nouveau l'arbre. C'est toujours agréable, mais l'effet de surprise est passé.");
                } else {
                    self.deja_enlace = true;
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} C'est étrange, mais étonnamment réconfortant.", colore!(Vert, "[+5 000 Aura]"));
                }
            }
            2 => {
                world.current_tick += 10;
                if self.deja_grave {
                    println!("Votre nom est déjà gravé là. L'écorce n'est pas un cahier de brouillon.");
                } else {
                    self.deja_grave = true;
                    player.aura += 15000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Votre légende commence à s'inscrire dans l'écorce.", colore!(Vert, "[+15 000 Aura]"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Chene {
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
        vec![Action::Observer, Action::Utiliser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Le tronc fait dix fois votre tour de taille. Ce qui n'est pas un exploit, vu ce que vous mangez.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur le chêne."),
        }
    }
}
