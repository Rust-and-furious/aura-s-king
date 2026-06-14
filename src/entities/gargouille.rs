use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::player::Player;
use crate::traits::Fightable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Gargouille {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub rubis_id: usize,
    pub bidule_id: usize,
    pub balai_id: usize,
    pub deja_insulte: bool,
    pub gargouille_brisee: bool,
}

impl Saveable for Gargouille {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("deja_insulte".to_string(), serde_json::json!(self.deja_insulte));
        state.insert("gargouille_brisee".to_string(), serde_json::json!(self.gargouille_brisee));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("deja_insulte") {
            if let Some(b) = val.as_bool() {
                self.deja_insulte = b;
            }
        }
        if let Some(val) = state.get("gargouille_brisee") {
            if let Some(b) = val.as_bool() {
                self.gargouille_brisee = b;
            }
        }
    }
}

impl Fightable for Gargouille {
    // une statue de pierre n'a pas de points de vie : on la brise d'un coup ou pas du tout
    fn recevoir_degats(&mut self, _degats: i32) {}

    fn est_vivant(&self) -> bool {
        !self.gargouille_brisee
    }
}

impl Interactable for Gargouille {
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
            Action::Dialoguer,
            Action::Attaquer { degats: 10 },
        ]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Elle est très laide. Elle vous rappelle vaguement votre oncle Maurice.");
            }
            // insulter la gargouille : défoulement gratifiant, mais une seule fois
            Action::Dialoguer => {
                world.current_tick += 3;
                if self.deja_insulte {
                    println!("Vous l'insultez de nouveau. Elle encaisse, stoïque. Comme votre oncle Maurice.");
                } else {
                    self.deja_insulte = true;
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Ça fait du bien de se défouler. Et puis elle ne va pas répondre... n'est-ce pas ?", colore!(Vert, "[+5 000 Aura]"));
                }
            }
            Action::Attaquer { degats } => {
                self.recevoir_degats(*degats);
                world.current_tick += 10;
                if self.gargouille_brisee {
                    println!("La gargouille n'est plus qu'un tas de gravats. Inutile d'insister.");
                } else if player.inventory.contains(&self.bidule_id)
                    || player.inventory.contains(&self.balai_id)
                {
                    if jet_reussite(world.current_tick, 30) {
                        self.gargouille_brisee = true;
                        player.inventory.push(self.rubis_id);
                        player.aura += 100000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!("{} La gargouille se brise dans un fracas de pierre, révélant un rubis rutilant niché en son cœur !", colore!(Vert, "[+100 000 Aura]"));
                    } else {
                        player.aura -= 25000.0;
                        // l'outil utilisé se brise sur la pierre (bidule en priorité, sinon balai)
                        if player.inventory.contains(&self.bidule_id) {
                            player.inventory.retain(|&x| x != self.bidule_id);
                        } else {
                            player.inventory.retain(|&x| x != self.balai_id);
                        }
                        crate::audio::play_sound("assets/defeat.wav");
                        println!("{} Votre outil rebondit sur la pierre et se brise net. La gargouille, elle, n'a pas bougé d'un pouce.", colore!(Rouge, "[-25 000 Aura]"));
                    }
                } else {
                    player.aura -= 10000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Frapper de la pierre à mains nues... Vos poignets pleurent.", colore!(Rouge, "[-10 000 Aura]"));
                }
            }
            _ => println!("Action impossible sur la gargouille."),
        }
    }
}
