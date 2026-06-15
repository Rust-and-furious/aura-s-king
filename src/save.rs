use crate::world::WorldManager;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub current_tick: usize,
    pub player: PlayerSaveData,
    pub zones: Vec<ZoneSaveData>,
    pub entities: HashMap<usize, HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSaveData {
    pub aura: f64,
    pub zone: usize,
    pub inventory: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ZoneSaveData {
    pub id: usize,
    pub interactables: Vec<usize>,
    pub interest_points: Vec<InterestPointSaveData>,
}

#[derive(Serialize, Deserialize)]
pub struct InterestPointSaveData {
    pub id: usize,
    pub interactables: Vec<usize>,
}

/// Sauvegarde l'état complet du jeu dans un fichier JSON.
pub fn save_game(world: &WorldManager, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let player = PlayerSaveData {
        aura: world.player.aura,
        zone: world.player.zone,
        inventory: world.player.inventory.clone(),
    };

    let zones = world.zones.iter().map(|z| ZoneSaveData {
        id: z.id,
        interactables: z.interactables.clone(),
        interest_points: z.interest_points.iter().map(|ip| InterestPointSaveData {
            id: ip.id,
            interactables: ip.interactables.clone(),
        }).collect(),
    }).collect();

    let mut entities = HashMap::new();
    for ent in &world.entities {
        let state = ent.save_state();
        if !state.is_empty() {
            entities.insert(ent.id(), state);
        }
    }

    let save_data = SaveData {
        current_tick: world.current_tick,
        player,
        zones,
        entities,
    };

    // Crée les dossiers parents si nécessaire
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &save_data)?;
    Ok(())
}

/// Restaure l'état complet du jeu depuis un fichier JSON de sauvegarde.
pub fn load_game(world: &mut WorldManager, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let save_data: SaveData = serde_json::from_reader(file)?;

    world.current_tick = save_data.current_tick;
    world.player.aura = save_data.player.aura;
    world.player.zone = save_data.player.zone;
    world.player.inventory = save_data.player.inventory;

    // Restaure les zones
    for zone_save in save_data.zones {
        if let Some(zone) = world.zones.iter_mut().find(|z| z.id == zone_save.id) {
            zone.interactables = zone_save.interactables;
            for ip_save in zone_save.interest_points {
                if let Some(ip) = zone.interest_points.iter_mut().find(|ip| ip.id == ip_save.id) {
                    ip.interactables = ip_save.interactables;
                }
            }
        }
    }

    // Restaure les entités
    for ent in &mut world.entities {
        if let Some(state) = save_data.entities.get(&ent.id()) {
            ent.load_state(state);
        }
    }

    Ok(())
}
