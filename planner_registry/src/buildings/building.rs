use crate::items::recipe::Recipe;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Machine {
    None,
    Assembler,
    Blender,
    Constructor,
    Foundry,
    Manufacturer,
    Miner,
    OilExtractor,
    Packager,
    ParticleAccelerator,
    Refinery,
    // ResourceWell
    Smelter,
    WaterExtractor,
}

#[derive(Debug, Clone)]
pub struct Building {
    pub recipe: Recipe,
}
