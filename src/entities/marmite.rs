use crate::actions::Action;
use crate::entities::Interactable;
use crate::player::Player;
use crate::world::WorldManager;

pub struct Marmite {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub has_key: bool,
    pub key_revealed: bool,
    pub key_entity_id: usize,
}

impl Interactable for Marmite {
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
                println!("Vous observez la marmite. {}", self.description);
                if self.has_key && !self.key_revealed {
                    self.key_revealed = true;
                    player.inventory.push(self.key_entity_id);
                    println!("\nAu fond de la soupe tiède, quelque chose brille... C'est la clé de votre propre porte !");
                    println!("Vous l'empochez : la \x1B[36m'Clé de la maison'\x1B[0m est maintenant dans votre inventaire.");
                } else if self.has_key {
                    println!("\nLa soupe est toujours tiède, mais la clé a déjà été récupérée.");
                }
            }
            Action::Ramasser => {
                world.remove_interactable_from_zone(player.zone, self.id);
                player.inventory.push(self.id);
                println!("Vous ramassez la marmite. Elle est ajoutée à votre inventaire.");
            }
            _ => println!("Action impossible sur la marmite."),
        }
    }
}
