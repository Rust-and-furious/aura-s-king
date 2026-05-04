use crate::entities::Objet;

pub struct Player {
    pub aura: f64,
    pub zone: usize,
    pub inventory: Vec<Objet>,
}
