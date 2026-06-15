use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Tonneau {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub deja_cache: bool,
}

impl Saveable for Tonneau {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("deja_cache".to_string(), serde_json::json!(self.deja_cache));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("deja_cache") {
            if let Some(b) = val.as_bool() {
                self.deja_cache = b;
            }
        }
    }
}

impl Useable for Tonneau {
    // boire (pari 50/50, espérance négative) ou se cacher (gain unique) → sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Boire dedans"),
            MenuOption::new("Se cacher dedans"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "Le tonneau de la taverne") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 10;
                // 50/50, mais le mauvais résultat coûte cher et du temps : espérance négative, pas farmable
                if jet_reussite(world.current_tick, 50) {
                    player.aura += 30000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Un cidre vigoureux ! Vous vous sentez pousser des ailes (de poulet).", colore!(Vert, "[+30 000 Aura]"));
                } else {
                    world.current_tick += 60;
                    player.aura -= 40000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Du vinaigre. De l'ancien vinaigre très acide. Vous vous évanouissez une heure.", colore!(Rouge, "[-40 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 15;
                if self.deja_cache {
                    println!("Vous vous recachez dans le tonneau. Toujours personne ne vous cherche. C'est presque vexant.");
                } else {
                    self.deja_cache = true;
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Vous vous y cachez. Personne ne vous cherchait de toute façon.", colore!(Vert, "[+5 000 Aura]"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Tonneau {
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
                println!("Un tonneau entrouvert. Ça sent fort le fermenté.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur le tonneau."),
        }
    }
}
