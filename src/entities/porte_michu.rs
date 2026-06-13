use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::traits::{Fightable, Openable};
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct PorteMichu {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub est_ouverte: bool,
    pub michu_id: usize,
    pub chat_id: usize,
}

impl Saveable for PorteMichu {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("est_ouverte".to_string(), serde_json::json!(self.est_ouverte));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("est_ouverte") {
            if let Some(b) = val.as_bool() {
                self.est_ouverte = b;
            }
        }
    }
}

impl Openable for PorteMichu {
    fn ouvrir(&mut self) -> Result<(), &'static str> {
        if self.est_ouverte {
            return Err("La porte est déjà ouverte.");
        }
        self.est_ouverte = true;
        Ok(())
    }

    fn fermer(&mut self) -> Result<(), &'static str> {
        if !self.est_ouverte {
            return Err("La porte est déjà fermée.");
        }
        self.est_ouverte = false;
        Ok(())
    }
}

impl Fightable for PorteMichu {
    // la porte n'a pas de PV : la frapper ne l'ouvre pas, ça réveille juste Michu
    fn recevoir_degats(&mut self, _degats: i32) {}

    // "vivante" = encore fermée, donc toujours un obstacle
    fn est_vivant(&self) -> bool {
        !self.est_ouverte
    }
}

impl PorteMichu {
    // porte ouverte → Michu et le chat apparaissent dans le lieu (comme la marmite révèle la clé)
    fn reveler_occupants(&self, world: &mut WorldManager, zone_id: usize) {
        let zone = &mut world.zones[zone_id];
        for ip in &mut zone.interest_points {
            if ip.interactables.contains(&self.id) && !ip.interactables.contains(&self.michu_id) {
                ip.interactables.push(self.michu_id);
                ip.interactables.push(self.chat_id);
            }
        }
    }
}

impl Interactable for PorteMichu {
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
        let mut actions = vec![Action::Observer];
        if !self.est_ouverte {
            actions.push(Action::Ouvrir);
            actions.push(Action::Attaquer { degats: 10 });
        }
        actions
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                if self.est_ouverte {
                    println!("La porte est grande ouverte. Michu vous attend à l'intérieur.");
                } else {
                    println!("{}", self.description);
                }
            }
            Action::Ouvrir => match self.ouvrir() {
                Ok(_) => {
                    world.current_tick += 2;
                    self.reveler_occupants(world, player.zone);
                    crate::audio::play_sound("assets/victory.wav");
                    println!("« Oh, c'est toi gamin ! Entre donc ! » Michu vous ouvre la porte.");
                }
                Err(e) => println!("{}", e),
            },
            Action::Attaquer { degats } => {
                self.recevoir_degats(*degats);
                world.current_tick += 120;
                player.aura -= 150000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!("{} Michu vous assomme d'un coup de poêle. Sacrés réflexes pour 847 ans. Vous vous réveillez deux heures plus tard.", colore!(Rouge, "[-150 000 Aura]"));
            }
            _ => println!("Action impossible sur la porte de Michu."),
        }
    }
}
