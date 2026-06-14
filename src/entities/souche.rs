use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Souche {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub epee_id: usize,
    pub epee_prise: bool,
    pub deja_assis: bool,
}

impl Saveable for Souche {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("epee_prise".to_string(), serde_json::json!(self.epee_prise));
        state.insert("deja_assis".to_string(), serde_json::json!(self.deja_assis));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("epee_prise") {
            if let Some(b) = val.as_bool() {
                self.epee_prise = b;
            }
        }
        if let Some(val) = state.get("deja_assis") {
            if let Some(b) = val.as_bool() {
                self.deja_assis = b;
            }
        }
    }
}

impl Useable for Souche {
    // deux façons d'utiliser la souche → on demande laquelle via un sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Tirer l'épée"),
            MenuOption::new("S'asseoir sur la souche"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "La souche mystérieuse") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 15;
                if jet_reussite(world.current_tick, 20) {
                    // l'épée ne se tire qu'une fois : ensuite la souche est vide
                    if self.epee_prise {
                        println!("La souche est vide : vous avez déjà arraché son épée. Vous tirez sur du bois.");
                    } else {
                        self.epee_prise = true;
                        player.inventory.push(self.epee_id);
                        player.aura += 120000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!("{} VOUS AVEZ TIRÉ L'ÉPÉE DE LA SOUCHE ! Bon, ce n'est pas Excalibur, et prévoyez peut-être un vaccin contre le tétanos.", colore!(Vert, "[+120 000 Aura]"));
                    }
                } else {
                    player.aura -= 20000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} L'épée ne bouge pas. Vous vous êtes fait un tour de rein. Aïe.", colore!(Rouge, "[-20 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 10;
                if self.deja_assis {
                    println!("Vous vous rasseyez. Le papillon, lui, a clairement mieux à faire cette fois.");
                } else {
                    self.deja_assis = true;
                    player.aura += 8000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Vous méditez un instant. Un papillon se pose sur votre nez. Très poétique.", colore!(Vert, "[+8 000 Aura]"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Souche {
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
                println!("Sur la souche, une épée est plantée. Elle est rouillée, tordue et franchement pas terrible. Mais c'est une ÉPÉE.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur la souche."),
        }
    }
}
