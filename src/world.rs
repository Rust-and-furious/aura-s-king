use crate::entities::{Balai, CleMaison, Fenetre, Interactable, Lit, Marmite, Porte};
use crate::player::Player;

pub struct WorldManager {
    pub current_tick: usize,
    pub max_ticks: usize,
    pub player: Player,
    pub zones: Vec<Zone>,
    pub entities: Vec<Box<dyn Interactable>>,
}

impl WorldManager {
    /// Formate le temps actuel en heures et minutes, en partant de 08h00.
    pub fn format_time(&self) -> String {
        let total_minutes = 8 * 60 + self.current_tick;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        format!("{:02}h{:02}", hours, minutes)
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

/// fonction temporaire pour charger la premiére zone
/// Sert d'exemple de construction du monde, à remplacer par un système de chargement via le JSON plus tard
pub fn load_first_zone() -> WorldManager {
    let player = Player {
        aura: 0.0,
        zone: 0,
        inventory: Vec::new(),
    };

    let entities: Vec<Box<dyn Interactable>> = vec![
        Box::new(Lit {
            id: 0,
            name: "Lit".to_string(),
            description: "Une paillasse qui gratte sur laquelle vous passez vos nuits à regretter vos choix de vie.".to_string(),
        }),
        Box::new(Marmite {
            id: 1,
            name: "Marmite".to_string(),
            description: "Une marmite en fonte contenant un reste de soupe tiède à l'odeur suspecte de chou.".to_string(),
            has_key: true,
            key_revealed: false,
            key_entity_id: 2,
        }),
        Box::new(CleMaison {
            id: 2,
            name: "Clé de la maison".to_string(),
            description: "Une clé en fer un peu rouillée, trouvée au fond de la marmite.".to_string(),
        }),
        Box::new(Balai {
            id: 3,
            name: "Balai".to_string(),
            description: "Un vieux balai usé. Très bon pour faire semblant de travailler.".to_string(),
        }),
        Box::new(Fenetre {
            id: 4,
            name: "Fenêtre".to_string(),
            description: "Une fenêtre donnant sur la plaine verdoyante (et miteuse).".to_string(),
            est_ouverte: false,
            est_cassee: false,
            target_zone: 1,
        }),
        Box::new(Porte {
            id: 5,
            name: "Porte".to_string(),
            description: "La lourde porte en chêne fermant votre modeste demeure.".to_string(),
            est_ouverte: false,
            is_locked: true,
            key_entity_id: 2,
            target_zone: 1,
        }),
    ];

    let zone_maison = Zone {
        id: 0,
        description: "Votre modeste demeure. Il y fait sombre et l'air est lourd de regrets et d'odeur de chou.".to_string(),
        interest_points: Vec::new(),
        interactables: vec![0, 1, 3, 4, 5], // La clé cachée (index 2) est absente au début
        connected_zones: vec![1],
    };

    let zone_plaine = Zone {
        id: 1,
        description: "La Plaine. L'air frais vous frappe le visage. Vous y êtes enfin !"
            .to_string(),
        interest_points: Vec::new(),
        interactables: Vec::new(),
        connected_zones: Vec::new(),
    };

    WorldManager {
        current_tick: 0,
        max_ticks: 720, // 08h00 à 20h00
        player,
        zones: vec![zone_maison, zone_plaine],
        entities,
    }
}
