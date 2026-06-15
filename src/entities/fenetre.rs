use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::traits::Openable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Fenetre {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub est_ouverte: bool,
    pub est_cassee: bool,
    pub target_zone: usize,
}

impl Saveable for Fenetre {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("est_ouverte".to_string(), serde_json::json!(self.est_ouverte));
        state.insert("est_cassee".to_string(), serde_json::json!(self.est_cassee));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("est_ouverte") {
            if let Some(b) = val.as_bool() {
                self.est_ouverte = b;
            }
        }
        if let Some(val) = state.get("est_cassee") {
            if let Some(b) = val.as_bool() {
                self.est_cassee = b;
            }
        }
    }
}

impl Openable for Fenetre {
    fn ouvrir(&mut self) -> Result<(), &'static str> {
        if self.est_cassee { return Err("Impossible, la fenêtre est cassée !"); }
        if self.est_ouverte { return Err("C'est déjà ouvert."); }
        self.est_ouverte = true;
        Ok(())
    }

    fn fermer(&mut self) -> Result<(), &'static str> {
        if !self.est_ouverte { return Err("C'est déjà fermé."); }
        self.est_ouverte = false;
        Ok(())
    }
}

impl Interactable for Fenetre {
    fn id(&self) -> usize { self.id }
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }

    fn get_actions(&self, _player: &Player, _world: &WorldManager) -> Vec<Action> {
        let mut actions = vec![
            Action::Observer,
            Action::Deplacer { target_zone: self.target_zone },
        ];
        if !self.est_cassee {
            if self.est_ouverte {
                actions.push(Action::Fermer);
            } else {
                actions.push(Action::Ouvrir);
            }
        }
        actions
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                println!("Vous observez la fenêtre. {}", self.description);
                if self.est_ouverte {
                    println!("Elle est ouverte, vous pouvez voir la plaine verdoyante au loin.");
                } else {
                    println!("Elle est fermée. La vitre est un peu sale.");
                }
            }
            Action::Ouvrir => match self.ouvrir() {
                Ok(_) => {
                    world.current_tick += 1;
                    println!("Vous ouvrez la fenêtre.");
                }
                Err(e) => println!("{}", e),
            },
            Action::Fermer => match self.fermer() {
                Ok(_) => {
                    world.current_tick += 1;
                    println!("Vous fermez la fenêtre.");
                }
                Err(e) => println!("{}", e),
            },
            Action::Deplacer { target_zone } => {
                world.current_tick += 10;
                player.zone = *target_zone;

                if self.est_ouverte {
                    player.aura += 15000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("\n{} Sortie audacieuse ! Vous enjambez le rebord et atterrissez gracieusement dans l'herbe.", colore!(Vert, "[+15 000 Aura]"));
                } else {
                    self.est_cassee = true;
                    self.est_ouverte = true;
                    player.aura -= 40000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("\n{} BAM ! Vous traversez la vitre fermée tête la première.", colore!(Rouge, "[-40 000 Aura]"));
                    println!("Votre dignité ne s'en remet pas, votre peau non plus. Des éclats de verre s'enfoncent à chacun de vos pas.");
                }
            }
            _ => println!("Action impossible sur la fenêtre."),
        }
    }
}
