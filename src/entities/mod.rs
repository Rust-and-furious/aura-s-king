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

// renvoie true avec une proba de `pourcentage`% (petit jet de chance)
pub fn jet_reussite(seed: usize, pourcentage: usize) -> bool {
    pseudo_rand(seed) % 100 < pourcentage
}

pub mod balai;
pub mod chat;
pub mod cle;
pub mod epouvantail;
pub mod fenetre;
pub mod lit;
pub mod marmite;
pub mod meule;
pub mod meunier;
pub mod michu;
pub mod objet;
pub mod porte;
pub mod porte_michu;
pub mod puits;
pub mod sacs_farine;

pub use balai::Balai;
pub use chat::Chat;
pub use cle::CleMaison;
pub use epouvantail::Epouvantail;
pub use fenetre::Fenetre;
pub use lit::Lit;
pub use marmite::Marmite;
pub use meule::Meule;
pub use meunier::Meunier;
pub use michu::Michu;
pub use objet::Objet;
pub use porte::Porte;
pub use porte_michu::PorteMichu;
pub use puits::Puits;
pub use sacs_farine::SacsFarine;
