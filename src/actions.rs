use crate::player::Player;
use crate::world::WorldManager;

pub struct Dialogue {
    pub texte: String,
}

impl Dialogue {
    pub fn executer(&self, _source: &mut Player, _world: &mut WorldManager) {}
}

pub struct Fouiller {}

impl Fouiller {
    pub fn executer(&self, _source: &mut Player, _world: &mut WorldManager) {}
    pub fn find_loot(&self) {}
}

pub struct Ramasser {}

impl Ramasser {
    pub fn executer(&self, _source: &mut Player, _world: &mut WorldManager) {}
    pub fn add_item_to_inventory(&self) {}
}
