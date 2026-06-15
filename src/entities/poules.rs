use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Poules {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub target_zone: usize,
    pub deja_caresse: bool,
}

impl Saveable for Poules {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("deja_caresse".to_string(), serde_json::json!(self.deja_caresse));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("deja_caresse") {
            if let Some(b) = val.as_bool() {
                self.deja_caresse = b;
            }
        }
    }
}

impl Useable for Poules {
    // caresser une poule : mignon, mais une seule fois (sinon aura farmable)
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        world.current_tick += 2;
        if self.deja_caresse {
            println!("Les poules en ont assez de vos câlins. Elles vous toisent.");
        } else {
            self.deja_caresse = true;
            player.aura += 5000.0;
            println!("{} La poule accepte. C'est doux. Vous repensez brièvement à vos choix de vie.", colore!(Vert, "[+5 000 Aura]"));
        }
        Ok(())
    }
}

impl Interactable for Poules {
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
            Action::Utiliser,
            Action::Deplacer {
                target_zone: self.target_zone,
            },
            Action::Dialoguer,
        ]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Des poules se promènent dans le village avec bien plus d'assurance que vous.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            Action::Deplacer { target_zone } => {
                world.current_tick += 15;
                player.aura -= 30000.0;
                player.zone = *target_zone;
                println!("{} Vous courez après les poules. Tout le village vous regarde. Elles sont bien plus rapides. Vous avez l'air ridicule.", colore!(Rouge, "[-30 000 Aura]"));
            }
            Action::Dialoguer => {
                world.current_tick += 2;
                println!("« Cot. Cot cot. Cot. » ... Vous n'êtes pas plus avancé.");
            }
            _ => println!("Action impossible avec les poules."),
        }
    }
}
