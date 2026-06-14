use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Barque {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub planche_id: usize,
    pub corde_id: usize,
    pub target_zone: usize,
    pub reparee: bool,
}

impl Saveable for Barque {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("reparee".to_string(), serde_json::json!(self.reparee));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("reparee") {
            if let Some(b) = val.as_bool() {
                self.reparee = b;
            }
        }
    }
}

impl Useable for Barque {
    // deux façons d'utiliser la barque → sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Monter dedans sans la réparer"),
            MenuOption::new("Réparer la barque"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "La vieille barque") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 10;
                player.aura -= 40000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!("{} Vous coulez lentement en essayant de garder votre dignité. L'eau est froide, votre fierté aussi.", colore!(Rouge, "[-40 000 Aura]"));
            }
            1 => {
                world.current_tick += 30;
                if self.reparee {
                    println!("La barque est déjà colmatée. Inutile d'en rajouter.");
                } else if player.inventory.contains(&self.planche_id)
                    && player.inventory.contains(&self.corde_id)
                {
                    // on ne consomme ni la planche ni la corde : la corde sert aussi à la canne du ponton
                    self.reparee = true;
                    player.aura += 30000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Avec la planche et la corde, vous colmatez la coque. C'est pas joli, mais ça flotte ! L'île est désormais accessible.", colore!(Vert, "[+30 000 Aura]"));
                } else {
                    println!("Il vous manque de quoi réparer : il faut une planche de bois et une corde solide.");
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Barque {
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
        ]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                if self.reparee {
                    println!("La barque est désormais colmatée et prête à naviguer vers l'île.");
                } else {
                    println!("Une barque avec un trou béant dans la coque. Classique.");
                }
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            Action::Deplacer { target_zone } => {
                if self.reparee {
                    world.current_tick += 10;
                    player.zone = *target_zone;
                    println!("Vous ramez vaillamment jusqu'à la petite île au milieu du lac.");
                } else {
                    println!("La barque prend l'eau dès que vous posez le pied dedans. Réparez-la d'abord !");
                }
            }
            _ => println!("Action impossible sur la barque."),
        }
    }
}
