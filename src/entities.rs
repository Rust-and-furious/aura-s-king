use crate::player::Player;
use crate::world::WorldManager;

pub trait Interactable {
    fn description(&self) -> String;
    fn name(&self) -> String;
    fn interagir(&self, player: &mut Player, world: &mut WorldManager);
}

pub struct Npc {
    pub name: String,
    pub description: String,
    pub is_hostile: bool,
}

impl Interactable for Npc {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn interagir(&self, player: &mut Player, world: &mut WorldManager) {
        self.dialoguer(player, world);
    }
}

impl Npc {
    pub fn dialoguer(&self, player: &mut Player, world: &mut WorldManager) {
        println!("{} engage la conversation.", self.name());
    }
}

pub struct Furniture {
    pub name: String,
    pub description: String,
    pub durability: i32,
}

impl Interactable for Furniture {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn interagir(&self, player: &mut Player, world: &mut WorldManager) {
        self.fouiller(player, world);
    }
}

impl Furniture {
    pub fn fouiller(&self, player: &mut Player, world: &mut WorldManager) {
        println!("{} est en train d'être fouillé.", self.name());
    }

    pub fn observer(&self) -> String {
        self.description.clone()
    }
}

pub struct Objet {
    pub name: String,
    pub description: String,
    pub weight: i32,
    pub durability: i32,
}

impl Interactable for Objet {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn interagir(&self, player: &mut Player, world: &mut WorldManager) {
        self.ramasser(player, world);
    }
}

impl Objet {
    pub fn ramasser(&self, player: &mut Player, world: &mut WorldManager) {
        println!("{} est ramassé.", self.name());
    }
}
