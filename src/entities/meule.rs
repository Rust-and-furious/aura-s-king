use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;

pub struct Meule {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl Interactable for Meule {
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
                println!("Une grande meule. Elle tourne. C'est son truc, c'est une meule.");
            }
            // 3 façons de l'utiliser → on demande laquelle
            Action::Utiliser => {
                let options = vec![
                    MenuOption::new("Mettre la main dedans"),
                    MenuOption::new("Essayer de la soulever"),
                    MenuOption::new("Essayer de croquer dedans"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(options, "La meule de pierre") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 60;
                        player.aura -= 500000.0;
                        println!("\x1B[31m[-500 000 Aura]\x1B[0m Mauvaise idée. Très mauvaise idée. Le meunier doit appeler le guérisseur pour recoudre votre dignité.");
                    }
                    1 => {
                        world.current_tick += 15;
                        if jet_reussite(world.current_tick, 10) {
                            player.aura += 750000.0;
                            println!("\x1B[32m[+750 000 Aura]\x1B[0m Le meunier est bouche bée. EXPLOIT LÉGENDAIRE : vous soulevez la meule !");
                        } else {
                            player.aura -= 20000.0;
                            println!("\x1B[31m[-20 000 Aura]\x1B[0m Votre dos fait un bruit de branche sèche et s'en souviendra longtemps.");
                        }
                    }
                    2 => {
                        world.current_tick += 5;
                        player.aura -= 80000.0;
                        println!("\x1B[31m[-80 000 Aura]\x1B[0m C'est une meule de pierre, pas de fromage, idiot. Vous avez encore plus l'air d'un paysan sans dents.");
                    }
                    _ => {}
                }
            }
            _ => println!("Action impossible sur la meule."),
        }
    }
}
