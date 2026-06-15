use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Barde {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub oeuf_id: usize,
    pub botte_id: usize,
    pub chanson_faite: bool,
}

impl Saveable for Barde {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("chanson_faite".to_string(), serde_json::json!(self.chanson_faite));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("chanson_faite") {
            if let Some(b) = val.as_bool() {
                self.chanson_faite = b;
            }
        }
    }
}

impl Interactable for Barde {
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
        vec![Action::Observer, Action::Dialoguer]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("{}", self.description);
            }
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Écouter chanter"),
                    MenuOption::new("Demander une chanson sur vous"),
                    MenuOption::new("Offrir un objet"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que faire avec le barde ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 10;
                        player.aura -= 15000.0;
                        crate::audio::play_sound("assets/defeat.wav");
                        println!("{} C'est vraiment très mauvais. Vos oreilles saignent.", colore!(Rouge, "[-15 000 Aura]"));
                    }
                    1 => {
                        world.current_tick += 15;
                        // une chanson légendaire ne se compose qu'une fois ; les échecs, eux, peuvent se répéter
                        if self.chanson_faite {
                            println!("Le barde a déjà fait votre chanson. Il refuse de se répéter, « par respect pour l'art ».");
                        } else if jet_reussite(world.current_tick, 40) {
                            self.chanson_faite = true;
                            player.aura += 200000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} La chanson est atroce mais entraînante : les gens scandent votre nom !", colore!(Vert, "[+200 000 Aura]"));
                        } else {
                            player.aura -= 80000.0;
                            crate::audio::play_sound("assets/defeat.wav");
                            println!("{} Le barde improvise sur « Le paysan qui pue le chou ». Humiliation publique.", colore!(Rouge, "[-80 000 Aura]"));
                        }
                    }
                    2 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.oeuf_id) {
                            player.inventory.retain(|&x| x != self.oeuf_id);
                            player.aura += 400000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « PAR LES DIEUX ! Un œuf de phénix ! Je compose un OPÉRA entier en votre honneur ! »", colore!(Vert, "[+400 000 Aura]"));
                        } else if player.inventory.contains(&self.botte_id) {
                            player.inventory.retain(|&x| x != self.botte_id);
                            player.aura += 15000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Je peux en faire un instrument percussif bizarre. Merci ! »", colore!(Vert, "[+15 000 Aura]"));
                        } else {
                            println!("« Je ne mendie pas... enfin si, mais pas ça. »");
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("Le barde gratte sa lyre faussement."),
        }
    }
}
