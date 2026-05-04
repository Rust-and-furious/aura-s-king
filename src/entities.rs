use crate::actions::{Dialogue, Fouiller, Ramasser};

pub trait Interactable {
    fn description(&self) -> String;
}

pub struct Npc {
    pub name: String,
    pub is_hostile: bool,
    pub dialogues: Vec<Dialogue>,
}

impl Interactable for Npc {
    fn description(&self) -> String {
        self.name.clone()
    }
}

pub struct Furniture {
    pub name: String,
    pub durability: i32,
    pub fouiller_actions: Vec<Fouiller>,
}

impl Interactable for Furniture {
    fn description(&self) -> String {
        self.name.clone()
    }
}

pub struct Objet {
    pub name: String,
    pub weight: i32,
    pub durability: i32,
    pub ramasser_actions: Vec<Ramasser>,
}

impl Interactable for Objet {
    fn description(&self) -> String {
        self.name.clone()
    }
}
