use crate::actions::Action;
use crate::entities::Interactable;
use crate::player::Player;
use crate::world::WorldManager;

// objet ramassable générique (corde, chapeau, pièce...), surtout un jeton d'inventaire
pub struct Objet {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub aura_ramassage: f64,
}

impl Interactable for Objet {
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
            Action::Observer => println!("{}", self.description),
            Action::Ramasser => {
                world.remove_interactable_from_zone(player.zone, self.id);
                player.inventory.push(self.id);
                if self.aura_ramassage != 0.0 {
                    player.aura += self.aura_ramassage;
                }
                println!("Vous ramassez {}. Ajouté à l'inventaire.", self.name);
            }
            _ => println!("Vous ne pouvez pas faire ça avec {}.", self.name),
        }
    }
}
