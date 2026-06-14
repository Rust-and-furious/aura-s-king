use crate::actions::Action;
use crate::entities::{pseudo_rand, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Canne {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub corde_id: usize,
    pub poisson_id: usize,
    pub botte_id: usize,
    pub reparee: bool,
    pub poisson_pris: bool,
    pub botte_prise: bool,
}

impl Saveable for Canne {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("reparee".to_string(), serde_json::json!(self.reparee));
        state.insert("poisson_pris".to_string(), serde_json::json!(self.poisson_pris));
        state.insert("botte_prise".to_string(), serde_json::json!(self.botte_prise));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("reparee") {
            if let Some(b) = val.as_bool() {
                self.reparee = b;
            }
        }
        if let Some(val) = state.get("poisson_pris") {
            if let Some(b) = val.as_bool() {
                self.poisson_pris = b;
            }
        }
        if let Some(val) = state.get("botte_prise") {
            if let Some(b) = val.as_bool() {
                self.botte_prise = b;
            }
        }
    }
}

impl Useable for Canne {
    // réparer ou pêcher → sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Réparer la canne"),
            MenuOption::new("Pêcher"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "La canne à pêche") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 10;
                if self.reparee {
                    println!("La canne est déjà réparée. Prête à pêcher.");
                } else if player.inventory.contains(&self.corde_id) {
                    // on ne consomme pas la corde : elle sert aussi à réparer la barque
                    self.reparee = true;
                    player.aura += 20000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Avec la corde, vous rafistolez la canne. Comme vos rêves, elle est réparée.", colore!(Vert, "[+20 000 Aura]"));
                } else {
                    println!("Il vous faut une corde solide pour réparer la canne.");
                }
            }
            1 => {
                if !self.reparee {
                    println!("La canne est cassée en deux. Réparez-la d'abord.");
                    return Ok(());
                }
                world.current_tick += 30;
                // tirage : 40 % poisson / 30 % botte / 30 % rien.
                // chaque prise « utile » est unique (sinon la pêche serait une mine d'aura infinie).
                let tirage = pseudo_rand(world.current_tick) % 100;
                if tirage < 40 {
                    if self.poisson_pris {
                        println!("Un autre poisson mord, mais minuscule. Vous le relâchez par fierté.");
                    } else {
                        self.poisson_pris = true;
                        player.inventory.push(self.poisson_id);
                        player.aura += 30000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!("{} Ça mord enfin ! Vous remontez un beau poisson frais.", colore!(Vert, "[+30 000 Aura]"));
                    }
                } else if tirage < 70 {
                    if self.botte_prise {
                        println!("Encore une vieille botte. Vous en avez déjà une, merci bien.");
                    } else {
                        self.botte_prise = true;
                        player.inventory.push(self.botte_id);
                        player.aura += 2000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!("{} Vous repêchez une vieille botte taille 47. Inutile mais amusant.", colore!(Vert, "[+2 000 Aura]"));
                    }
                } else {
                    println!("Les poissons sont au courant de votre condition sociale et ignorent l'appât.");
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Canne {
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
                println!("Cassée en deux. Comme vos rêves. Mais les rêves, ça se répare.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur la canne."),
        }
    }
}
