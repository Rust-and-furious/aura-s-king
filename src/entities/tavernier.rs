use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;

pub struct Tavernier {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub poisson_id: usize,
    pub noix_id: usize,
}

impl Saveable for Tavernier {}

impl Interactable for Tavernier {
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
                    MenuOption::new("Parler"),
                    MenuOption::new("Offrir un objet"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que dire au tavernier ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 3;
                        println!("« Bienvenue au Cochon Pendu ! On sert de la bière, des rumeurs et des mauvais conseils. Paraît que le roi cherche un remplaçant pour son bouffon démissionnaire. »");
                    }
                    1 => {
                        world.current_tick += 3;
                        if player.inventory.contains(&self.poisson_id) {
                            player.inventory.retain(|&x| x != self.poisson_id);
                            player.aura += 50000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Un poisson frais ! Ça change du ragoût éternel. Tiens, bois un coup à ma santé. »", colore!(Vert, "[+50 000 Aura]"));
                        } else if player.inventory.contains(&self.noix_id) {
                            player.inventory.retain(|&x| x != self.noix_id);
                            player.aura += 30000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « C'est quoi ce truc ? ...On va en faire un cocktail. »", colore!(Vert, "[+30 000 Aura]"));
                        } else {
                            println!("« Garde ça pour toi, j'ai déjà bien assez de bric-à-brac. »");
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("Le tavernier essuie un verre qui n'a jamais été propre de sa vie."),
        }
    }
}
