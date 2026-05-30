// traits.rs — Les traits de capacité (Composition > Héritage)
// Chaque trait représente un comportement précis qu'une entité peut posséder.

use crate::player::Player;
use crate::world::WorldManager;

/// Une entité que l'on peut ouvrir et fermer (porte, coffre, fenêtre...).
pub trait Openable {
    fn ouvrir(&mut self) -> Result<(), &'static str>;
    fn fermer(&mut self) -> Result<(), &'static str>;
}

/// Une entité que l'on peut combattre.
pub trait Fightable {
    fn recevoir_degats(&mut self, degats: i32);
    fn est_vivant(&self) -> bool;
}

/// Une entité consommable / utilisable (pomme, potion...).
pub trait Useable {
    fn utiliser(&mut self, player: &mut Player, world: &mut WorldManager) -> Result<(), &'static str>;
}
