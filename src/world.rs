pub struct WorldManager {
    // Parser le json + update le json celon les interactions
    pub zones: Vec<usize>,
    pub entities: Vec<usize>,
}

pub struct Zone {
    pub description: String,
    pub connected_zones: Vec<usize>, // id des zones
    pub interest_points: Vec<InterestPoint>,
    pub interactables: Vec<usize>, // ids dans la collection du WorldManager
}

pub struct InterestPoint {
    pub description: String,
    pub interactables: Vec<usize>,
}
