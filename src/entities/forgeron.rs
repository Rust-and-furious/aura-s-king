use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;

pub struct Forgeron {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub epee_id: usize,
    pub reforgee_id: usize,
    pub medaille_id: usize,
    pub bouclier_id: usize,
}

impl Saveable for Forgeron {}

impl Interactable for Forgeron {
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
                    MenuOption::new("Demander une armure"),
                    MenuOption::new("Offrir un objet"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que dire au forgeron ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 3;
                        println!("« J'forge. Tu veux quoi ? »");
                    }
                    1 => {
                        world.current_tick += 3;
                        println!("« T'as de quoi payer ? Non ? Alors dégage... Ou ramène-moi du bon métal. »");
                    }
                    2 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.epee_id) {
                            player.inventory.retain(|&x| x != self.epee_id);
                            player.inventory.push(self.reforgee_id);
                            player.aura += 150000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Oh ! Du bon acier sous la rouille ! Tiens, je te l'ai reforgée. »", colore!(Vert, "[+150 000 Aura]"));
                        } else if player.inventory.contains(&self.medaille_id) {
                            player.inventory.retain(|&x| x != self.medaille_id);
                            player.inventory.push(self.bouclier_id);
                            player.aura += 100000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Un bon métal ancien. Tiens, je t'ai fait un bouclier en échange. »", colore!(Vert, "[+100 000 Aura]"));
                        } else {
                            println!("« C'est de la camelote, ça. Reviens avec du vrai métal. »");
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("Le forgeron tape sur son enclume sans vous écouter."),
        }
    }
}
