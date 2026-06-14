use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct ArbreTordu {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub noix_id: usize,
    pub deja_grimpe: bool,
    pub deja_secoue: bool,
}

impl Saveable for ArbreTordu {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("deja_grimpe".to_string(), serde_json::json!(self.deja_grimpe));
        state.insert("deja_secoue".to_string(), serde_json::json!(self.deja_secoue));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("deja_grimpe") {
            if let Some(b) = val.as_bool() {
                self.deja_grimpe = b;
            }
        }
        if let Some(val) = state.get("deja_secoue") {
            if let Some(b) = val.as_bool() {
                self.deja_secoue = b;
            }
        }
    }
}

impl Useable for ArbreTordu {
    // deux façons d'utiliser l'arbre → sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Grimper"),
            MenuOption::new("Secouer"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "L'arbre tordu") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 15;
                if self.deja_grimpe {
                    println!("Vous regrimpez. La vue est toujours aussi belle, mais vous n'êtes plus aussi impressionné.");
                } else {
                    self.deja_grimpe = true;
                    player.aura += 20000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} La vue sur le château au loin est superbe. Vous vous y croyez déjà.", colore!(Vert, "[+20 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 3;
                if self.deja_secoue {
                    println!("Vous secouez encore l'arbre. Plus rien ne tombe : il est à sec de noix.");
                } else {
                    self.deja_secoue = true;
                    player.inventory.push(self.noix_id);
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Une noix de coco tombe. Vous n'êtes même pas sous les tropiques. Ne cherchez pas.", colore!(Vert, "[+5 000 Aura]"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for ArbreTordu {
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
                println!("Un arbre unique et tordu, le seul de l'île. Quelque chose pend à ses branches.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur l'arbre."),
        }
    }
}
