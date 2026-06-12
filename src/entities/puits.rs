use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;

pub struct Puits {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub corde_id: usize,
    pub deja_crie: bool,
}

impl Useable for Puits {
    // deux façons d'utiliser le puits → on demande laquelle via un sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Descendre dans le puits"),
            MenuOption::new("Jeter un objet dedans"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "Le puits") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 30;
                // 30% de réussite : l'espérance est négative pour qu'on ne puisse pas farmer l'aura ici
                if jet_reussite(world.current_tick, 30) {
                    player.aura += 20000.0;
                    if !player.inventory.contains(&self.corde_id) {
                        player.inventory.push(self.corde_id);
                    }
                    crate::audio::play_sound("assets/victory.wav");
                    println!("\x1B[32m[+20 000 Aura]\x1B[0m Au fond, vous trouvez une corde solide et la récupérez !");
                } else {
                    player.aura -= 15000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("\x1B[31m[-15 000 Aura]\x1B[0m Vous glissez et remontez trempé. Un crapaud vous juge en coassant.");
                }
            }
            1 => {
                if player.inventory.is_empty() {
                    println!("Vous n'avez rien à jeter.");
                    return Ok(());
                }
                // on liste l'inventaire pour choisir l'objet à jeter
                let mut inv_opts: Vec<MenuOption> = player
                    .inventory
                    .iter()
                    .map(|&id| MenuOption::new(world.entities[id].name()))
                    .collect();
                inv_opts.push(MenuOption::special("Annuler"));
                let sel = match select_from_menu(inv_opts, "Jeter quel objet ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return Ok(()),
                };
                if sel >= player.inventory.len() {
                    return Ok(());
                }
                world.current_tick += 3;
                let jete = player.inventory.remove(sel);
                player.aura -= 8000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!(
                    "\x1B[31m[-8 000 Aura]\x1B[0m Vous jetez {} dans le puits. Pourquoi ?! Le peuple taupe n'a pas besoin de ça !",
                    world.entities[jete].name()
                );
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Puits {
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
        vec![Action::Observer, Action::Dialoguer, Action::Utiliser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("C'est profond et sombre. Comme votre avenir si vous ne bougez pas.");
            }
            Action::Dialoguer => {
                world.current_tick += 2;
                // le cri ne galvanise qu'une fois : sinon c'est de l'aura gratuite à l'infini
                if self.deja_crie {
                    println!("Vous criez encore. L'écho, lassé, ne répond même plus.");
                } else {
                    self.deja_crie = true;
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("\x1B[32m[+5 000 Aura]\x1B[0m Vous criez dans le puits. L'écho répond « CHEVALIEEEER ». Vous êtes galvanisé.");
                }
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur le puits."),
        }
    }
}
