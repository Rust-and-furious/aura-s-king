use crate::actions::Action;
use crate::entities::{Interactable, find_entity_id};
use crate::player::Player;
use crate::world::WorldManager;

pub struct CleMaison {
    pub name: String,
    pub description: String,
}

impl Interactable for CleMaison {
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
                println!("Vous observez la clé. {}", self.description);
            }
            Action::Ramasser => {
                if let Some(id) = find_entity_id(self, world) {
                    let current_zone = player.zone;
                    world.zones[current_zone].interactables.retain(|&x| x != id);
                    player.inventory.push(id);
                    println!(
                        "Vous mettez la clé dans votre poche. Elle est ajoutée à votre inventaire."
                    );
                }
            }
            _ => println!("Action impossible sur la clé."),
        }
    }
}
