use crate::entities::Interactable;
use crate::player::Player;

pub struct WorldManager {
    pub current_tick: usize,
    pub max_ticks: usize,
    pub player: Player,
    pub zones: Vec<Zone>,
    pub entities: Vec<Box<dyn Interactable>>,
    /// Fin de partie : None = en cours, Some(true) = victoire, Some(false) = défaite.
    /// Renseigné par le Roi lors de l'évaluation finale ; lu par la boucle de jeu.
    pub fin_partie: Option<bool>,
}

impl WorldManager {
    /// Formate le temps actuel en heures et minutes, en partant de 08h00.
    pub fn format_time(&self) -> String {
        let total_minutes = 8 * 60 + self.current_tick;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        format!("{:02}h{:02}", hours, minutes)
    }

    // retire un objet ramassé de la zone : de la liste directe ET des points d'intérêt
    pub fn remove_interactable_from_zone(&mut self, zone_id: usize, entity_id: usize) {
        let zone = &mut self.zones[zone_id];
        zone.interactables.retain(|&x| x != entity_id);
        for ip in &mut zone.interest_points {
            ip.interactables.retain(|&x| x != entity_id);
        }
    }
}

pub struct Zone {
    pub id: usize,
    pub description: String,
    pub interest_points: Vec<InterestPoint>,
    pub interactables: Vec<usize>,
    pub connected_zones: Vec<usize>,
}

pub struct InterestPoint {
    pub id: usize,
    pub description: String,
    pub interactables: Vec<usize>,
}

