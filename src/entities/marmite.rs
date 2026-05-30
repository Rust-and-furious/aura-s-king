use crate::actions::Action;
use crate::entities::{Interactable, find_entity_id};
use crate::player::Player;
use crate::world::WorldManager;

pub struct Marmite {
    pub name: String,
    pub description: String,
    pub has_key: bool,
    pub key_revealed: bool,
    pub key_entity_id: usize,
}

impl Interactable for Marmite {
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
                println!("Vous observez la marmite. {}", self.description);
                if self.has_key && !self.key_revealed {
                    self.key_revealed = true;
                    let current_zone = player.zone;
                    world.zones[current_zone]
                        .interactables
                        .push(self.key_entity_id);
                    println!(
                        "\nAu fond de la soupe tiède, quelque chose brille... C'est la clé de votre propre porte !"
                    );
                    println!("La 'Clé de la maison' est maintenant visible dans la pièce.");
                } else if self.has_key {
                    println!("\nLa soupe est toujours tiède, mais la clé a déjà été retirée.");
                }
            }
            Action::Ramasser => {
                if let Some(id) = find_entity_id(self, world) {
                    let current_zone = player.zone;
                    world.zones[current_zone].interactables.retain(|&x| x != id);
                    player.inventory.push(id);
                    println!("Vous ramassez la marmite. Elle est ajoutée à votre inventaire.");
                }
            }
            _ => println!("Action impossible sur la marmite."),
        }
    }
}
