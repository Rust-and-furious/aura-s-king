use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;

pub struct Meule {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl Saveable for Meule {}

impl Useable for Meule {
    // trois façons de l'utiliser → on demande laquelle via un sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Mettre la main dedans"),
            MenuOption::new("Essayer de la soulever"),
            MenuOption::new("Essayer de croquer dedans"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "La meule de pierre") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 60;
                player.aura -= 500000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!("{} Mauvaise idée. Très mauvaise idée. Le meunier doit appeler le guérisseur pour recoudre votre dignité.", colore!(Rouge, "[-500 000 Aura]"));
            }
            1 => {
                world.current_tick += 15;
                // 2% seulement : vu le jackpot (+750 000), au-delà l'espérance redevient positive et farmable
                if jet_reussite(world.current_tick, 2) {
                    player.aura += 750000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Le meunier est bouche bée. EXPLOIT LÉGENDAIRE : vous soulevez la meule !", colore!(Vert, "[+750 000 Aura]"));
                } else {
                    player.aura -= 20000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Votre dos fait un bruit de branche sèche et s'en souviendra longtemps.", colore!(Rouge, "[-20 000 Aura]"));
                }
            }
            2 => {
                world.current_tick += 5;
                player.aura -= 80000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!("{} C'est une meule de pierre, pas de fromage, idiot. Vous avez encore plus l'air d'un paysan sans dents.", colore!(Rouge, "[-80 000 Aura]"));
            }
            _ => {}
        }
        Ok(())
    }
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
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur la meule."),
        }
    }
}
