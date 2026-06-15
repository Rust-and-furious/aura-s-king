use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::player::Player;
use crate::traits::{Fightable, Openable};
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Coffre {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub cape_id: usize,
    pub epee_id: usize,
    pub marmite_id: usize,
    pub est_ouverte: bool,
}

impl Saveable for Coffre {
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

impl Openable for Coffre {
    fn ouvrir(&mut self) -> Result<(), &'static str> {
        if self.est_ouverte {
            return Err("Le coffre est déjà ouvert.");
        }
        self.est_ouverte = true;
        Ok(())
    }

    fn fermer(&mut self) -> Result<(), &'static str> {
        if !self.est_ouverte {
            return Err("Le coffre est déjà fermé.");
        }
        self.est_ouverte = false;
        Ok(())
    }
}

impl Fightable for Coffre {
    // un coffre n'a pas de PV : on force le cadenas, on ne le "tue" pas
    fn recevoir_degats(&mut self, _degats: i32) {}

    fn est_vivant(&self) -> bool {
        !self.est_ouverte
    }
}

impl Coffre {
    fn donner_cape(&mut self, player: &mut Player) {
        let _ = self.ouvrir();
        player.inventory.push(self.cape_id);
        crate::audio::play_sound("assets/cape.wav");
    }
}

impl Interactable for Coffre {
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
                world.current_tick += 1;
                if self.est_ouverte {
                    println!("Le coffre est ouvert et vide. Vous avez déjà raflé son trésor.");
                } else {
                    println!("Un coffre en bois solide. Le cadenas est rouillé.");
                }
            }
            Action::Ouvrir => {
                world.current_tick += 10;
                // forcer le cadenas à mains nues : 40 % de réussite
                if jet_reussite(world.current_tick, 40) {
                    self.donner_cape(player);
                    player.aura += 150000.0;
                    println!("{} Le cadenas cède ! À l'intérieur, une magnifique cape brodée.", colore!(Vert, "[+150 000 Aura]"));
                } else {
                    player.aura -= 40000.0;
                    println!("{} Vos doigts saignent sur la rouille. Le cadenas tient bon.", colore!(Rouge, "[-40 000 Aura]"));
                }
            }
            Action::Attaquer { degats } => {
                self.recevoir_degats(*degats);
                world.current_tick += 5;
                if player.inventory.contains(&self.epee_id) {
                    player.inventory.retain(|&x| x != self.epee_id);
                    self.donner_cape(player);
                    player.aura += 150000.0;
                    println!("{} L'épée se brise mais le cadenas aussi. Marché conclu. Une cape brodée vous attend à l'intérieur.", colore!(Vert, "[+150 000 Aura]"));
                } else if player.inventory.contains(&self.marmite_id) {
                    player.inventory.retain(|&x| x != self.marmite_id);
                    self.donner_cape(player);
                    player.aura += 100000.0;
                    println!("{} BONG ! Le bruit résonne sur tout le lac. Le coffre cède. Une cape brodée vous attend.", colore!(Vert, "[+100 000 Aura]"));
                } else {
                    println!("Ça fait « toc ». Le coffre s'en fiche.");
                }
            }
            _ => println!("Action impossible sur le coffre."),
        }
    }
}
