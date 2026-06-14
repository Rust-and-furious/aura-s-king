use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::world::WorldManager;

// panneau d'interdiction de la forêt : on peut le lire, ou l'arracher pour récupérer une planche
pub struct Panneau {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub planche_id: usize,
}

impl Saveable for Panneau {}

impl Interactable for Panneau {
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
        vec![Action::Observer, Action::Ramasser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("« Bienvenue en Forêt de Brâme. Interdiction de crier, chanter, ou de devenir chevalier sans permis. »");
            }
            Action::Ramasser => {
                world.current_tick += 5;
                // arraché : le panneau quitte la zone et devient une planche dans l'inventaire
                world.remove_interactable_from_zone(player.zone, self.id);
                player.inventory.push(self.planche_id);
                player.aura += 15000.0;
                crate::audio::play_sound("assets/victory.wav");
                println!("{} Vous arrachez le panneau d'un coup sec. Rebelle dans l'âme ! Il vous reste une belle planche de bois.", colore!(Vert, "[+15 000 Aura]"));
            }
            _ => println!("Action impossible sur le panneau."),
        }
    }
}
