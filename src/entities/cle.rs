use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::player::Player;
use crate::world::WorldManager;

pub struct CleMaison {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl Saveable for CleMaison {}

impl Interactable for CleMaison {
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
                println!("Vous observez la clé. {}", self.description);
            }
            Action::Ramasser => {
                world.remove_interactable_from_zone(player.zone, self.id);
                player.inventory.push(self.id);
                println!(
                    "Vous mettez la clé dans votre poche. Elle est ajoutée à votre inventaire."
                );
            }
            _ => println!("Action impossible sur la clé."),
        }
    }
}
