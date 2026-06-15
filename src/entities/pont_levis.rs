use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::world::WorldManager;

pub struct PontLevis {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub target_zone: usize,
    pub laissez_passer_id: usize,
}

impl Saveable for PontLevis {}

impl Interactable for PontLevis {
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
        vec![
            Action::Observer,
            Action::Deplacer {
                target_zone: self.target_zone,
            },
        ]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Un pont-levis qui grince. De l'autre côté vous attend la salle du trône.");
            }
            Action::Deplacer { target_zone } => {
                // on ne traverse que si les gardes ont accordé l'accès (laissez-passer en inventaire)
                if player.inventory.contains(&self.laissez_passer_id) {
                    world.current_tick += 5;
                    player.zone = *target_zone;
                    println!("Vous traversez le pont-levis et pénétrez dans la salle du trône. Votre cœur bat la chamade.");
                } else {
                    println!("Les gardes vous barrent le passage. Obtenez d'abord leur accord.");
                }
            }
            _ => println!("Action impossible sur le pont-levis."),
        }
    }
}
