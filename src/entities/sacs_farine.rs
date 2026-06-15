use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::player::Player;
use crate::traits::Fightable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct SacsFarine {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub piece_id: usize,
    pub piece_trouvee: bool,
}

impl Saveable for SacsFarine {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("piece_trouvee".to_string(), serde_json::json!(self.piece_trouvee));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("piece_trouvee") {
            if let Some(b) = val.as_bool() {
                self.piece_trouvee = b;
            }
        }
    }
}

impl Fightable for SacsFarine {
    // de simples sacs : pas de points de vie, on tape juste dedans
    fn recevoir_degats(&mut self, _degats: i32) {}

    fn est_vivant(&self) -> bool {
        false
    }
}

impl Interactable for SacsFarine {
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
        vec![
            Action::Observer,
            Action::Fouiller,
            Action::Attaquer { degats: 1 },
        ]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Des dizaines de sacs empilés. Ça sent le pain et le labeur.");
            }
            Action::Fouiller => {
                world.current_tick += 10;
                // 40% de se faire surprendre par le meunier.
                if jet_reussite(world.current_tick, 40) {
                    player.aura -= 30000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Le meunier vous attrape la main dans le sac. Littéralement. Aucun butin.", colore!(Rouge, "[-30 000 Aura]"));
                } else if self.piece_trouvee {
                    println!("Vous fouillez encore, mais les sacs ne cachent plus rien d'intéressant.");
                } else {
                    self.piece_trouvee = true;
                    player.inventory.push(self.piece_id);
                    crate::audio::play_sound("assets/victory.wav");
                    println!("Discret comme une ombre, vous trouvez une pièce de monnaie et l'empochez.");
                }
            }
            Action::Attaquer { degats } => {
                self.recevoir_degats(*degats);
                world.current_tick += 5;
                player.aura -= 50000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!("{} Le meunier hurle, vous êtes couvert de farine. Pas très chevaleresque.", colore!(Rouge, "[-50 000 Aura]"));
            }
            _ => println!("Action impossible sur les sacs de farine."),
        }
    }
}
