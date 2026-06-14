use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Fontaine {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub piece_id: usize,
    pub deja_bu: bool,
    pub deja_lave: bool,
}

impl Saveable for Fontaine {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("deja_bu".to_string(), serde_json::json!(self.deja_bu));
        state.insert("deja_lave".to_string(), serde_json::json!(self.deja_lave));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("deja_bu") {
            if let Some(b) = val.as_bool() {
                self.deja_bu = b;
            }
        }
        if let Some(val) = state.get("deja_lave") {
            if let Some(b) = val.as_bool() {
                self.deja_lave = b;
            }
        }
    }
}

impl Useable for Fontaine {
    // trois usages de la fontaine → sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Boire"),
            MenuOption::new("Jeter une pièce"),
            MenuOption::new("Se laver"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "La fontaine du village") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 2;
                if self.deja_bu {
                    println!("Vous n'avez plus soif. L'eau tiède au goût de calcaire perd de son charme.");
                } else {
                    self.deja_bu = true;
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} L'eau est tiède et a un goût de calcaire. C'est la meilleure eau que vous ayez bue.", colore!(Vert, "[+5 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 2;
                if player.inventory.contains(&self.piece_id) {
                    player.inventory.retain(|&x| x != self.piece_id);
                    player.aura += 40000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Vous jetez la pièce et faites le vœu d'avoir de l'aura. Méta.", colore!(Vert, "[+40 000 Aura]"));
                } else {
                    println!("Vous n'avez pas de pièce à jeter. Faire un vœu gratuit ne marche pas, ici.");
                }
            }
            2 => {
                world.current_tick += 15;
                if self.deja_lave {
                    println!("Vous êtes déjà propre. Vous re-laver ne ferait que vexer les villageois qui attendent leur tour.");
                } else {
                    self.deja_lave = true;
                    player.aura += 15000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Vous sentez moins le chou. Les villageois vous regardent avec un dégoût modéré.", colore!(Vert, "[+15 000 Aura]"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Fontaine {
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
                println!("Une fontaine de pierre au centre du village. L'eau y coule paresseusement.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur la fontaine."),
        }
    }
}
