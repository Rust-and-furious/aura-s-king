use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Marmite {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub has_key: bool,
    pub key_revealed: bool,
    pub key_entity_id: usize,
}

impl Saveable for Marmite {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("key_revealed".to_string(), serde_json::json!(self.key_revealed));
        state.insert("has_key".to_string(), serde_json::json!(self.has_key));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("key_revealed") {
            if let Some(b) = val.as_bool() {
                self.key_revealed = b;
            }
        }
        if let Some(val) = state.get("has_key") {
            if let Some(b) = val.as_bool() {
                self.has_key = b;
            }
        }
    }
}

impl Interactable for Marmite {
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
        vec![Action::Observer, Action::Ramasser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        crate::audio::play_sound("assets/bong.wav");
        match action {
            Action::Observer => {
                println!("Vous observez la marmite. {}", self.description);
                if self.has_key && !self.key_revealed {
                    self.key_revealed = true;
                    player.inventory.push(self.key_entity_id);
                    println!("\nAu fond de la soupe tiède, quelque chose brille... C'est la clé de votre propre porte !");
                    println!("Vous l'empochez : la {} est maintenant dans votre inventaire.", colore!(Cyan, "'Clé de la maison'"));
                } else if self.has_key {
                    println!("\nLa soupe est toujours tiède, mais la clé a déjà été récupérée.");
                }
            }
            Action::Ramasser => {
                world.remove_interactable_from_zone(player.zone, self.id);
                player.inventory.push(self.id);
                println!("Vous ramassez la marmite. Elle est ajoutée à votre inventaire.");
            }
            _ => println!("Action impossible sur la marmite."),
        }
    }
}
