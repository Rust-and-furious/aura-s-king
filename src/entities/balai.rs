use crate::actions::Action;
use crate::entities::Interactable;
use crate::player::Player;
use crate::world::WorldManager;

pub struct Balai {
    pub id: usize,
    pub name: String,
    pub description: String,
}

impl Interactable for Balai {
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
                println!("Vous observez le balai. {}", self.description);
                println!(
                    "*Note* : Un outil formidable pour lutter contre la poussière. Moins contre les dragons."
                );
            }
            Action::Ramasser => {
                let current_zone = player.zone;
                world.zones[current_zone].interactables.retain(|&x| x != self.id);
                player.inventory.push(self.id);
                println!("Vous ramassez le balai. Il est ajouté à votre inventaire.");
            }
            _ => println!("Action impossible sur le balai."),
        }
    }
}
