use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;

pub struct Chat {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl Saveable for Chat {}

impl Useable for Chat {
    // caresser ou soulever → on demande lequel via un sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Caresser"),
            MenuOption::new("Soulever"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "Pataud, le chat de Michu") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 5;
                // gain volontairement faible : même à 70% de réussite, l'espérance reste négative
                if jet_reussite(world.current_tick, 70) {
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Pataud ronronne bruyamment. Vous vous sentez validé.", colore!(Vert, "[+5 000 Aura]"));
                } else {
                    player.aura -= 15000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Pataud n'est pas d'humeur et vous griffe méchamment le nez.", colore!(Rouge, "[-15 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 5;
                player.aura -= 40000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!("{} Le chat se transforme en tornade de griffes. Michu vous gronde. Double peine.", colore!(Rouge, "[-40 000 Aura]"));
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Chat {
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
        vec![Action::Utiliser, Action::Dialoguer]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            Action::Dialoguer => {
                world.current_tick += 2;
                println!("« Miaou. » ... C'est tout. C'était un chat, après tout.");
            }
            _ => println!("Pataud vous ignore royalement."),
        }
    }
}
