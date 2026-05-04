use crate::entities::Objet;

pub struct Player {
    pub aura: f64,
    pub zone: usize, // Représente la relation "se trouve dans" (1 Zone)
    pub inventory: Vec<Objet>, // Représente la relation "possède (inventaire)" (0..* Objet)
}
