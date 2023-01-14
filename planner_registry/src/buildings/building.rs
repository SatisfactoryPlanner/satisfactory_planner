use crate::items::recipe::Recipe;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Machine {
    None,
    Assembler,
    Blender,
    Constructor,
    Foundry,
    Manufacturer,
    Packager,
    ParticleAccelerator,
    Refinery,
    Smelter,
}

#[derive(Debug, Clone)]
pub struct Building {
    pub recipe: Recipe,
}
