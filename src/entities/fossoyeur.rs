use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;

pub struct Fossoyeur {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl Saveable for Fossoyeur {}

impl Interactable for Fossoyeur {
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

    fn execute_action(&mut self, action: &Action, _player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("{}", self.description);
            }
            // deux répliques possibles → on ouvre un sous-menu
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Parler"),
                    MenuOption::new("Demander une pelle"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que dire au fossoyeur ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 3;
                        println!("« Encore un futur client... Prends un ticket, j'suis débordé. »");
                    }
                    1 => {
                        world.current_tick += 3;
                        println!("« Une pelle, ça se mérite. Ou ça s'achète. T'as l'air d'avoir ni l'un ni l'autre. »");
                    }
                    _ => {}
                }
            }
            _ => println!("Le fossoyeur continue de creuser sans vous accorder un regard."),
        }
    }
}
