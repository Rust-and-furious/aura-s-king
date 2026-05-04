use crate::entities::Interactable;

pub struct WorldManager {
    // Type utilisé dans les paramètres des méthodes executer, 
    // non détaillé dans le diagramme
}

pub struct Zone {
    pub description: String,
    pub connected_zones: Vec<usize>,
    pub interest_points: Vec<InterestPoint>,
}

pub struct InterestPoint {
    pub description: String,
    pub interactables: Vec<Box<dyn Interactable>>,
}
