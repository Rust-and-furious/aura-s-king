// world.rs — Le WorldManager : source unique de vérité du jeu.

use crate::entities::Interactable;
use crate::player::Player;

pub struct WorldManager {
    pub current_tick: usize,
    pub max_ticks: usize,
    pub player: Player,
    pub zones: Vec<Zone>,
    pub entities: Vec<Box<dyn Interactable>>, // stockage central de toutes les entités
}

pub struct Zone {
    pub id: usize,
    pub description: String,
    pub interest_points: Vec<InterestPoint>,
    pub interactables: Vec<usize>,   // IDs dans WorldManager.entities
    pub connected_zones: Vec<usize>, // IDs d'autres zones
}

pub struct InterestPoint {
    pub id: usize,
    pub description: String,
    pub interactables: Vec<usize>, // IDs dans WorldManager.entities
}
