use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;

pub struct Seau {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub seau_vide_id: usize,
}

impl Saveable for Seau {}

impl Useable for Seau {
    // se mettre le seau sur la tête : toujours une mauvaise idée
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        world.current_tick += 5;
        player.aura -= 50000.0;
        println!("{} Vous ne voyez plus rien, trébuchez et tombez dans le lac. Bravo.", colore!(Rouge, "[-50 000 Aura]"));
        Ok(())
    }
}

impl Interactable for Seau {
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
        vec![Action::Observer, Action::Ramasser, Action::Utiliser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        crate::audio::play_sound("assets/sceau.wav");
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Un seau vide qui traîne sur le ponton. Il a connu des jours meilleurs.");
            }
            Action::Ramasser => {
                world.current_tick += 2;
                // on embarque le seau : il quitte le ponton et devient un objet d'inventaire
                world.remove_interactable_from_zone(player.zone, self.id);
                player.inventory.push(self.seau_vide_id);
                println!("Vous embarquez le seau. On ne sait jamais, ça peut servir (ou pas).");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible sur le seau."),
        }
    }
}
