use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Esprit {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub manuel_id: usize,
    pub duel_gagne: bool,
}

impl Saveable for Esprit {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("duel_gagne".to_string(), serde_json::json!(self.duel_gagne));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("duel_gagne") {
            if let Some(b) = val.as_bool() {
                self.duel_gagne = b;
            }
        }
    }
}

impl Interactable for Esprit {
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
                println!("Une silhouette spectrale et translucide flotte au-dessus de son propre tombeau, l'air profondément contrarié.");
            }
            // plusieurs sujets → sous-menu
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Parler"),
                    MenuOption::new("Demander sa mort"),
                    MenuOption::new("Provoquer en duel d'Aura"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que dire à l'esprit ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 5;
                        println!("« QUI OSE TROUBLER MON REPOS ? Oh, un bouseux. Pars, avant que je ne te maudisse d'une haleine d'ail éternelle. »");
                    }
                    1 => {
                        world.current_tick += 5;
                        println!("« J'ai glissé sur une poule pendant mon adoubement. Mon crâne a rencontré le trône. Un complot, j'en suis sûr. »");
                    }
                    2 => {
                        world.current_tick += 10;
                        // le duel ne se gagne qu'une fois : ensuite l'esprit s'est incliné
                        if self.duel_gagne {
                            println!("Le fantôme vous a déjà cédé. Il fait mine de ne pas vous voir, ce qui est ironique pour un fantôme.");
                        } else if player.aura >= 150000.0 {
                            self.duel_gagne = true;
                            player.inventory.push(self.manuel_id);
                            player.aura += 300000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} Le fantôme bégaie, impressionné par votre prestance, et s'évapore en vous laissant son Manuel du Parfait Petit Chevalier !", colore!(Vert, "[+300 000 Aura]"));
                        } else {
                            player.aura -= 100000.0;
                            crate::audio::play_sound("assets/defeat.wav");
                            println!("{} Il se moque de vous avec un rire d'outre-tombe. Votre ego est pulvérisé.", colore!(Rouge, "[-100 000 Aura]"));
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("L'esprit vous traverse en frissonnant. Désagréable."),
        }
    }
}
