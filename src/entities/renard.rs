use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Renard {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub baie_id: usize,
    pub target_zone: usize,
    pub baie_donnee: bool,
}

impl Saveable for Renard {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("baie_donnee".to_string(), serde_json::json!(self.baie_donnee));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("baie_donnee") {
            if let Some(b) = val.as_bool() {
                self.baie_donnee = b;
            }
        }
    }
}

impl Useable for Renard {
    // s'approcher doucement : le renard offre sa baie une seule fois
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        world.current_tick += 5;
        if self.baie_donnee {
            println!("Le renard vous a déjà offert sa baie. Il vous fixe désormais avec un ennui poli.");
        } else {
            self.baie_donnee = true;
            player.inventory.push(self.baie_id);
            player.aura += 15000.0;
            crate::audio::play_sound("assets/victory.wav");
            println!("{} Le renard s'approche, renifle votre main et dépose une baie à vos pieds.", colore!(Vert, "[+15 000 Aura]"));
        }
        Ok(())
    }
}

impl Interactable for Renard {
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
                println!("Un renard roux vous fixe avec une intelligence dérangeante.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            Action::Deplacer { target_zone } => {
                world.current_tick += 20;
                player.aura -= 10000.0;
                player.zone = *target_zone;
                crate::audio::play_sound("assets/defeat.wav");
                println!("{} Vous quittez la clairière pour lui courir après dans les bois. Il court bien plus vite que vous et se retourne pour vous regarder avec mépris.", colore!(Rouge, "[-10 000 Aura]"));
            }
            Action::Dialoguer => {
                world.current_tick += 3;
                println!("Le renard penche la tête. Vous croyez qu'il comprend. Il ne comprend pas.");
            }
            _ => println!("Action impossible avec le renard."),
        }
    }
}
