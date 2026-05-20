use crate::player::Player;
use crate::world::WorldManager;

pub trait Interactable {
    fn description(&self) -> String;
    fn name(&self) -> String;
}

pub struct Npc {
    pub is_hostile: bool,
}

impl Npc {
    pub fn dialoguer(&self) {
        println!("Engages in dialogue.");
    }
}

impl Interactable for Npc {
    fn description(&self) -> String {
        if self.is_hostile {
            "A hostile NPC".to_string()
        } else {
            "A friendly NPC".to_string()
        }
    }

    fn name(&self) -> String {
        "NPC".to_string()
    }
}

impl Npc {
    pub fn dialoguer(&self, player: &mut Player, world_manager: WorldManager) {
        println!("{} engages in dialogue.", self.name());
    }
}

pub struct Furniture {
    pub durability: i32,
}

impl Interactable for Furniture {
    fn description(&self) -> String {
            "A piece of furniture".to_string()
    }

    fn name(&self) -> String {
        "Furniture".to_string()
    }
}

impl Furniture {
    pub fn fouiller(&self, player: &mut Player, world_manager: WorldManager) {
        println!("{} is being searched.", self.name());
    }

    pub fn observer(&self) {
        println!("{} is being observed.", self.name());
    }
}

pub struct Objet {
    pub weight: i32,
    pub durability: i32,
}

impl Interactable for Objet {
    fn description(&self) -> String {
        "An object that can be picked up".to_string()
    }

    fn name(&self) -> String {
        "An object".to_string()
    }
}

impl Objet {
    pub fn ramasser(&self, player: &mut Player, world_manager: WorldManager) {
        println!("{} is being picked up.", self.name());
    }
}
