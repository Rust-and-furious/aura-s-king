// ============================================================
// Ce module gère le chargement du monde depuis un fichier JSON externe.
// Il constitue la séparation stricte entre le moteur de jeu (Rust) et
// les données (JSON), conformément aux contraintes du projet.
//
// Fonctionnement :
//   1. Désérialisation via des DTOs intermédiaires (serde).
//   2. Construction d'index (String → usize) pour zones et entités.
//   3. Résolution des références texte → indices usize.
//   4. Instanciation des structs concrets (Box<dyn Interactable>).
// ============================================================

use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;

use crate::entities::{
    Balai, Chat, CleMaison, Epouvantail, Fenetre, Interactable, Lit, Marmite, Meule, Meunier,
    Michu, Objet, Porte, PorteMichu, Puits, SacsFarine,
};
use crate::player::Player;
use crate::world::{InterestPoint, WorldManager, Zone};

// ──────────────────────────────────────────────────────────────
// Erreurs de chargement
// ──────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
    /// Une référence (ID texte) est introuvable dans les index construits.
    UnknownRef(String),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "Erreur de lecture du fichier : {e}"),
            LoadError::Json(e) => write!(f, "Erreur de parsing JSON : {e}"),
            LoadError::UnknownRef(r) => write!(f, "Référence inconnue dans le JSON : '{r}'"),
        }
    }
}

impl std::error::Error for LoadError {}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(e: serde_json::Error) -> Self {
        LoadError::Json(e)
    }
}

// ──────────────────────────────────────────────────────────────
// DTOs — structures de désérialisation (JSON → Rust intermédiaire)
// ──────────────────────────────────────────────────────────────

/// DTO racine du fichier JSON.
#[derive(Deserialize)]
struct WorldDto {
    start_zone: String,
    max_ticks: usize,
    start_aura: f64,
    intro_text: String,
    zones: Vec<ZoneDto>,
    entities: Vec<EntityDto>,
}

/// DTO d'une zone.
#[derive(Deserialize)]
struct ZoneDto {
    id: String,
    description: String,
    #[serde(default)]
    connected_zones: Vec<String>,
    #[serde(default)]
    interactables: Vec<String>,
    #[serde(default)]
    interest_points: Vec<InterestPointDto>,
}

/// DTO d'un point d'intérêt.
#[derive(Deserialize)]
struct InterestPointDto {
    /// L'ID textuel est conservé dans le JSON pour la lisibilité, mais
    /// n'est pas utilisé à l'exécution : l'index de la zone parente sert d'ID.
    #[allow(dead_code)]
    id: String,
    description: String,
    #[serde(default)]
    interactables: Vec<String>,
}

/// DTO des entités, discriminées par le champ `"type"`.
/// Chaque variante correspond à un type concret qui implémente `Interactable`.
/// Pour ajouter un nouveau type d'entité : ajouter une variante ici et son
/// cas dans la factory `build_entity` ci-dessous.
#[derive(Deserialize)]
#[serde(tag = "type")]
enum EntityDto {
    Lit {
        id: String,
        name: String,
        description: String,
    },
    Marmite {
        id: String,
        name: String,
        description: String,
        has_key: bool,
        key_revealed: bool,
        key_entity_id: String,
    },
    CleMaison {
        id: String,
        name: String,
        description: String,
    },
    Balai {
        id: String,
        name: String,
        description: String,
    },
    Fenetre {
        id: String,
        name: String,
        description: String,
        est_ouverte: bool,
        est_cassee: bool,
        target_zone: String,
    },
    Porte {
        id: String,
        name: String,
        description: String,
        est_ouverte: bool,
        is_locked: bool,
        key_entity_id: String,
        target_zone: String,
    },
    // ── Entités de la Plaine ──────────────────────────────────
    Objet {
        id: String,
        name: String,
        description: String,
        #[serde(default)]
        aura_ramassage: f64,
    },
    Puits {
        id: String,
        name: String,
        description: String,
        corde_entity_id: String,
    },
    Epouvantail {
        id: String,
        name: String,
        description: String,
        chapeau_entity_id: String,
    },
    Meunier {
        id: String,
        name: String,
        description: String,
        marmite_entity_id: String,
        farine_entity_id: String,
    },
    Meule {
        id: String,
        name: String,
        description: String,
    },
    SacsFarine {
        id: String,
        name: String,
        description: String,
        piece_entity_id: String,
    },
    PorteMichu {
        id: String,
        name: String,
        description: String,
        #[serde(default)]
        est_ouverte: bool,
        michu_entity_id: String,
        chat_entity_id: String,
    },
    Michu {
        id: String,
        name: String,
        description: String,
        biscuit_entity_id: String,
        broche_entity_id: String,
        chapeau_entity_id: String,
    },
    Chat {
        id: String,
        name: String,
        description: String,
    },
}

impl EntityDto {
    fn id(&self) -> &str {
        match self {
            EntityDto::Lit { id, .. } => id,
            EntityDto::Marmite { id, .. } => id,
            EntityDto::CleMaison { id, .. } => id,
            EntityDto::Balai { id, .. } => id,
            EntityDto::Fenetre { id, .. } => id,
            EntityDto::Porte { id, .. } => id,
            EntityDto::Objet { id, .. } => id,
            EntityDto::Puits { id, .. } => id,
            EntityDto::Epouvantail { id, .. } => id,
            EntityDto::Meunier { id, .. } => id,
            EntityDto::Meule { id, .. } => id,
            EntityDto::SacsFarine { id, .. } => id,
            EntityDto::PorteMichu { id, .. } => id,
            EntityDto::Michu { id, .. } => id,
            EntityDto::Chat { id, .. } => id,
        }
    }
}

// ──────────────────────────────────────────────────────────────
// Résultat du chargement
// ──────────────────────────────────────────────────────────────

/// Résultat retourné par `load_from_json`.
/// L'`intro_text` est séparé du `WorldManager` pour ne pas modifier
/// la structure de ce dernier (conformité au diagramme de classe).
pub struct LoadedWorld {
    pub world: WorldManager,
    pub intro_text: String,
}

// ──────────────────────────────────────────────────────────────
// Fonction de chargement principale
// ──────────────────────────────────────────────────────────────

/// Charge et construit un `WorldManager` complet depuis un fichier JSON.
///
/// Le fichier JSON utilise des IDs texte pour toutes les références
/// (zones, entités). Cette fonction les résout en indices `usize` via
/// deux passes, avant d'instancier les structs concrets.
pub fn load_from_json(path: &str) -> Result<LoadedWorld, LoadError> {
    let content = std::fs::read_to_string(path)?;
    load_from_str(&content)
}

/// Variante de [`load_from_json`] qui prend le contenu JSON directement en
/// mémoire plutôt qu'un chemin de fichier.
///
/// Utile pour les tests unitaires : on peut charger un mini-monde décrit dans
/// une chaîne, sans dépendre d'un fichier sur le disque.
pub fn load_from_str(content: &str) -> Result<LoadedWorld, LoadError> {
    // ── Désérialisation ───────────────────────────────────────
    let dto: WorldDto = serde_json::from_str(content)?;

    // ── Passe 1 : construction des index String → usize ──────
    let entity_index: HashMap<String, usize> = dto
        .entities
        .iter()
        .enumerate()
        .map(|(i, e)| (e.id().to_string(), i))
        .collect();

    let zone_index: HashMap<String, usize> = dto
        .zones
        .iter()
        .enumerate()
        .map(|(i, z)| (z.id.clone(), i))
        .collect();

    // Helper : résoudre un ID texte en index usize ou retourner une erreur.
    let resolve_entity = |id: &str| -> Result<usize, LoadError> {
        entity_index
            .get(id)
            .copied()
            .ok_or_else(|| LoadError::UnknownRef(id.to_string()))
    };
    let resolve_zone = |id: &str| -> Result<usize, LoadError> {
        zone_index
            .get(id)
            .copied()
            .ok_or_else(|| LoadError::UnknownRef(id.to_string()))
    };

    // ── Passe 2 : factory — construction des entités concrètes ──
    let entities: Vec<Box<dyn Interactable>> = dto
        .entities
        .into_iter()
        .enumerate()
        .map(|(i, e)| build_entity(i, e, &resolve_entity, &resolve_zone))
        .collect::<Result<_, _>>()?;

    // ── Construction des zones ────────────────────────────────
    // Chaque point d'intérêt reçoit un ID unique (compteur global), et non plus
    // l'index de sa zone parente : deux points d'intérêt distincts ne doivent
    // jamais partager le même ID.
    let mut zones: Vec<Zone> = Vec::with_capacity(dto.zones.len());
    let mut next_ip_id: usize = 0;

    for (i, z) in dto.zones.into_iter().enumerate() {
        let connected_zones = z
            .connected_zones
            .iter()
            .map(|id| resolve_zone(id))
            .collect::<Result<Vec<_>, _>>()?;

        let interactables = z
            .interactables
            .iter()
            .map(|id| resolve_entity(id))
            .collect::<Result<Vec<_>, _>>()?;

        let mut interest_points = Vec::with_capacity(z.interest_points.len());
        for ip in z.interest_points {
            let ip_interactables = ip
                .interactables
                .iter()
                .map(|id| resolve_entity(id))
                .collect::<Result<Vec<_>, _>>()?;
            interest_points.push(InterestPoint {
                id: next_ip_id,
                description: ip.description,
                interactables: ip_interactables,
            });
            next_ip_id += 1;
        }

        zones.push(Zone {
            id: i,
            description: z.description,
            interest_points,
            interactables,
            connected_zones,
        });
    }

    // ── Joueur ────────────────────────────────────────────────
    let player = Player {
        aura: dto.start_aura,
        zone: resolve_zone(&dto.start_zone)?,
        inventory: Vec::new(),
    };

    // ── Assemblage ────────────────────────────────────────────
    let world = WorldManager {
        current_tick: 0,
        max_ticks: dto.max_ticks,
        player,
        zones,
        entities,
    };

    Ok(LoadedWorld {
        world,
        intro_text: dto.intro_text,
    })
}

// ──────────────────────────────────────────────────────────────
// Factory : EntityDto → Box<dyn Interactable>
// ──────────────────────────────────────────────────────────────

fn build_entity(
    id: usize,
    dto: EntityDto,
    resolve_entity: &impl Fn(&str) -> Result<usize, LoadError>,
    resolve_zone: &impl Fn(&str) -> Result<usize, LoadError>,
) -> Result<Box<dyn Interactable>, LoadError> {
    match dto {
        EntityDto::Lit { name, description, .. } => Ok(Box::new(Lit { id, name, description })),

        EntityDto::Marmite {
            name,
            description,
            has_key,
            key_revealed,
            key_entity_id,
            ..
        } => Ok(Box::new(Marmite {
            id,
            name,
            description,
            has_key,
            key_revealed,
            key_entity_id: resolve_entity(&key_entity_id)?,
        })),

        EntityDto::CleMaison { name, description, .. } => {
            Ok(Box::new(CleMaison { id, name, description }))
        }

        EntityDto::Balai { name, description, .. } => Ok(Box::new(Balai { id, name, description })),

        EntityDto::Fenetre {
            name,
            description,
            est_ouverte,
            est_cassee,
            target_zone,
            ..
        } => Ok(Box::new(Fenetre {
            id,
            name,
            description,
            est_ouverte,
            est_cassee,
            target_zone: resolve_zone(&target_zone)?,
        })),

        EntityDto::Porte {
            name,
            description,
            est_ouverte,
            is_locked,
            key_entity_id,
            target_zone,
            ..
        } => Ok(Box::new(Porte {
            id,
            name,
            description,
            est_ouverte,
            is_locked,
            key_entity_id: resolve_entity(&key_entity_id)?,
            target_zone: resolve_zone(&target_zone)?,
        })),

        // ── Entités de la Plaine ──────────────────────────────
        EntityDto::Objet {
            name,
            description,
            aura_ramassage,
            ..
        } => Ok(Box::new(Objet {
            id,
            name,
            description,
            aura_ramassage,
        })),

        EntityDto::Puits {
            name,
            description,
            corde_entity_id,
            ..
        } => Ok(Box::new(Puits {
            id,
            name,
            description,
            corde_id: resolve_entity(&corde_entity_id)?,
            deja_crie: false,
        })),

        EntityDto::Epouvantail {
            name,
            description,
            chapeau_entity_id,
            ..
        } => Ok(Box::new(Epouvantail {
            id,
            name,
            description,
            chapeau_id: resolve_entity(&chapeau_entity_id)?,
            chapeau_pris: false,
            deja_attaque: false,
        })),

        EntityDto::Meunier {
            name,
            description,
            marmite_entity_id,
            farine_entity_id,
            ..
        } => Ok(Box::new(Meunier {
            id,
            name,
            description,
            marmite_id: resolve_entity(&marmite_entity_id)?,
            farine_id: resolve_entity(&farine_entity_id)?,
            travail_donne: false,
        })),

        EntityDto::Meule { name, description, .. } => {
            Ok(Box::new(Meule { id, name, description }))
        }

        EntityDto::SacsFarine {
            name,
            description,
            piece_entity_id,
            ..
        } => Ok(Box::new(SacsFarine {
            id,
            name,
            description,
            piece_id: resolve_entity(&piece_entity_id)?,
            piece_trouvee: false,
        })),

        EntityDto::PorteMichu {
            name,
            description,
            est_ouverte,
            michu_entity_id,
            chat_entity_id,
            ..
        } => Ok(Box::new(PorteMichu {
            id,
            name,
            description,
            est_ouverte,
            michu_id: resolve_entity(&michu_entity_id)?,
            chat_id: resolve_entity(&chat_entity_id)?,
        })),

        EntityDto::Michu {
            name,
            description,
            biscuit_entity_id,
            broche_entity_id,
            chapeau_entity_id,
            ..
        } => Ok(Box::new(Michu {
            id,
            name,
            description,
            biscuit_id: resolve_entity(&biscuit_entity_id)?,
            broche_id: resolve_entity(&broche_entity_id)?,
            chapeau_id: resolve_entity(&chapeau_entity_id)?,
            biscuit_donne: false,
        })),

        EntityDto::Chat { name, description, .. } => {
            Ok(Box::new(Chat { id, name, description }))
        }
    }
}
