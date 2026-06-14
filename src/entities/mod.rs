use crate::actions::Action;
use crate::player::Player;
use crate::world::WorldManager;

// ============================================================
// Ce module centralise toutes les entités interactives du jeu, avec une interface commune.
// Chaque entité implémente le trait Interactable, qui définit les actions possibles et leur exécution.
// Cela permet au moteur de jeu de traiter toutes les entités de manière uniforme, sans connaître leurs détails spécifiques.
// ============================================================

use std::collections::HashMap;

pub trait Saveable {
    /// Exporte l'état mutable de l'entité.
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        HashMap::new()
    }
    /// Restaure l'état mutable depuis la map.
    fn load_state(&mut self, _state: &HashMap<String, serde_json::Value>) {}
}

pub trait Interactable: Saveable {
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

pub mod arbre_tordu;
pub mod balai;
pub mod barde;
pub mod barque;
pub mod canne;
pub mod champignon;
pub mod chat;
pub mod chene;
pub mod cle;
pub mod coffre;
pub mod enclume;
pub mod epouvantail;
pub mod ermite;
pub mod esprit;
pub mod fenetre;
pub mod fontaine;
pub mod forgeron;
pub mod fossoyeur;
pub mod gardes;
pub mod gargouille;
pub mod lac;
pub mod lit;
pub mod marchand;
pub mod marmite;
pub mod meule;
pub mod meunier;
pub mod michu;
pub mod objet;
pub mod panneau;
pub mod pont_levis;
pub mod porte;
pub mod porte_michu;
pub mod poules;
pub mod puits;
pub mod renard;
pub mod roi;
pub mod sacs_farine;
pub mod seau;
pub mod souche;
pub mod tavernier;
pub mod tombe;
pub mod tonneau;

pub use arbre_tordu::ArbreTordu;
pub use balai::Balai;
pub use barde::Barde;
pub use barque::Barque;
pub use canne::Canne;
pub use champignon::Champignon;
pub use chat::Chat;
pub use chene::Chene;
pub use cle::CleMaison;
pub use coffre::Coffre;
pub use enclume::Enclume;
pub use epouvantail::Epouvantail;
pub use ermite::Ermite;
pub use esprit::Esprit;
pub use fenetre::Fenetre;
pub use fontaine::Fontaine;
pub use forgeron::Forgeron;
pub use fossoyeur::Fossoyeur;
pub use gardes::Gardes;
pub use gargouille::Gargouille;
pub use lac::Lac;
pub use lit::Lit;
pub use marchand::Marchand;
pub use marmite::Marmite;
pub use meule::Meule;
pub use meunier::Meunier;
pub use michu::Michu;
pub use objet::Objet;
pub use panneau::Panneau;
pub use pont_levis::PontLevis;
pub use porte::Porte;
pub use porte_michu::PorteMichu;
pub use poules::Poules;
pub use puits::Puits;
pub use renard::Renard;
pub use roi::Roi;
pub use sacs_farine::SacsFarine;
pub use seau::Seau;
pub use souche::Souche;
pub use tavernier::Tavernier;
pub use tombe::Tombe;
pub use tonneau::Tonneau;
