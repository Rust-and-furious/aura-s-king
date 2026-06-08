use crate::actions::Action;
use crate::player::Player;
use crate::world::WorldManager;

// ============================================================
// Ce module centralise toutes les entités interactives du jeu, avec une interface commune.
// Chaque entité implémente le trait Interactable, qui définit les actions possibles et leur exécution.
// Cela permet au moteur de jeu de traiter toutes les entités de manière uniforme, sans connaître leurs détails spécifiques.
// ============================================================

pub trait Interactable {
    fn id(&self) -> usize;
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    /// L'entité déclare dynamiquement ce que le joueur peut faire avec elle.
    fn get_actions(&self, player: &Player, world: &WorldManager) -> Vec<Action>;

    /// Le moteur de jeu déclenche l'action choisie par le joueur.
    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager);
}

// Helper simple pour le random
pub fn pseudo_rand(seed: usize) -> usize {
    seed.wrapping_mul(1103515245).wrapping_add(12345)
}

/// Trouve l'index physique (ID) d'une entité dans la liste globale du WorldManager.
pub fn find_entity_id(entity: &dyn Interactable, world: &WorldManager) -> Option<usize> {
    let self_ptr = entity as *const dyn Interactable as *const ();
    for (i, ent) in world.entities.iter().enumerate() {
        let ent_ptr = &**ent as *const dyn Interactable as *const ();
        // reference égale ?
        if std::ptr::eq(self_ptr, ent_ptr) {
            return Some(i);
        }
    }
    None
}

pub mod balai;
pub mod cle;
pub mod fenetre;
pub mod lit;
pub mod marmite;
pub mod porte;

pub use balai::Balai;
pub use cle::CleMaison;
pub use fenetre::Fenetre;
pub use lit::Lit;
pub use marmite::Marmite;
pub use porte::Porte;
