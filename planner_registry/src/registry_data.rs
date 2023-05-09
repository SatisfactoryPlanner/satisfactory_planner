use std::{collections::HashMap, sync::Arc};
use crate::{
    buildings::building::Machine,
    items::{item::Item, recipe::Recipe, ItemAmount},
};
#[allow(non_snake_case)]
#[allow(clippy::redundant_clone)]
pub(crate) fn get_registry() -> (Vec<Arc<Item>>, HashMap<&'static str, Vec<Recipe>>) {
let mut item_registry: Vec<Arc<Item>> = Vec::new();
let mut recipe_registry: HashMap<&'static str, Vec<Recipe>> = HashMap::new();
{
let reinforced_iron_plate = Arc::new(Item { name: "Reinforced Iron Plate", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/IronPlateReinforced/Desc_IronPlateReinforced
item_registry.push(reinforced_iron_plate.clone());
let iron_plate = Arc::new(Item { name: "Iron Plate", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/IronPlate/Desc_IronPlate
item_registry.push(iron_plate.clone());
let rubber = Arc::new(Item { name: "Rubber", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Rubber/Desc_Rubber
item_registry.push(rubber.clone());
let modular_frame = Arc::new(Item { name: "Modular Frame", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrame/Desc_ModularFrame
item_registry.push(modular_frame.clone());
let screw = Arc::new(Item { name: "Screw", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/IronScrew/Desc_IronScrew
item_registry.push(screw.clone());
let cable = Arc::new(Item { name: "Cable", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Cable/Desc_Cable
item_registry.push(cable.clone());
let wire = Arc::new(Item { name: "Wire", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Wire/Desc_Wire
item_registry.push(wire.clone());
let heavy_oil_residue = Arc::new(Item { name: "Heavy Oil Residue", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/HeavyOilResidue/Desc_HeavyOilResidue
item_registry.push(heavy_oil_residue.clone());
let empty_canister = Arc::new(Item { name: "Empty Canister", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/FluidCanister/Desc_FluidCanister
item_registry.push(empty_canister.clone());
let copper_sheet = Arc::new(Item { name: "Copper Sheet", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CopperSheet/Desc_CopperSheet
item_registry.push(copper_sheet.clone());
let iron_ingot = Arc::new(Item { name: "Iron Ingot", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/IronIngot/Desc_IronIngot
item_registry.push(iron_ingot.clone());
let plastic = Arc::new(Item { name: "Plastic", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Plastic/Desc_Plastic
item_registry.push(plastic.clone());
let steel_ingot = Arc::new(Item { name: "Steel Ingot", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelIngot/Desc_SteelIngot
item_registry.push(steel_ingot.clone());
let iron_ore = Arc::new(Item { name: "Iron Ore", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreIron/Desc_OreIron
item_registry.push(iron_ore.clone());
let petroleum_coke = Arc::new(Item { name: "Petroleum Coke", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/PetroleumCoke/Desc_PetroleumCoke
item_registry.push(petroleum_coke.clone());
let copper_ingot = Arc::new(Item { name: "Copper Ingot", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CopperIngot/Desc_CopperIngot
item_registry.push(copper_ingot.clone());
let copper_ore = Arc::new(Item { name: "Copper Ore", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreCopper/Desc_OreCopper
item_registry.push(copper_ore.clone());
let rotor = Arc::new(Item { name: "Rotor", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Rotor/Desc_Rotor
item_registry.push(rotor.clone());
let packaged_fuel = Arc::new(Item { name: "Packaged Fuel", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Fuel/Desc_Fuel
item_registry.push(packaged_fuel.clone());
let packaged_water = Arc::new(Item { name: "Packaged Water", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/Water/Desc_PackagedWater
item_registry.push(packaged_water.clone());
let aluminum_scrap = Arc::new(Item { name: "Aluminum Scrap", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumScrap/Desc_AluminumScrap
item_registry.push(aluminum_scrap.clone());
let alumina_solution = Arc::new(Item { name: "Alumina Solution", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Alumina/Desc_AluminaSolution
item_registry.push(alumina_solution.clone());
let circuit_board = Arc::new(Item { name: "Circuit Board", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CircuitBoard/Desc_CircuitBoard
item_registry.push(circuit_board.clone());
let versatile_framework = Arc::new(Item { name: "Versatile Framework", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_2
item_registry.push(versatile_framework.clone());
let steel_beam = Arc::new(Item { name: "Steel Beam", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelPlate/Desc_SteelPlate
item_registry.push(steel_beam.clone());
let caterium_ingot = Arc::new(Item { name: "Caterium Ingot", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GoldIngot/Desc_GoldIngot
item_registry.push(caterium_ingot.clone());
let heavy_modular_frame = Arc::new(Item { name: "Heavy Modular Frame", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrameHeavy/Desc_ModularFrameHeavy
item_registry.push(heavy_modular_frame.clone());
let encased_industrial_beam = Arc::new(Item { name: "Encased Industrial Beam", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelPlateReinforced/Desc_SteelPlateReinforced
item_registry.push(encased_industrial_beam.clone());
let crude_oil = Arc::new(Item { name: "Crude Oil", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/CrudeOil/Desc_LiquidOil
item_registry.push(crude_oil.clone());
let automated_wiring = Arc::new(Item { name: "Automated Wiring", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_3
item_registry.push(automated_wiring.clone());
let stator = Arc::new(Item { name: "Stator", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Stator/Desc_Stator
item_registry.push(stator.clone());
let high_speed_connector = Arc::new(Item { name: "High-Speed Connector", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/HighSpeedConnector/Desc_HighSpeedConnector
item_registry.push(high_speed_connector.clone());
let smart_plating = Arc::new(Item { name: "Smart Plating", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_1
item_registry.push(smart_plating.clone());
let fabric = Arc::new(Item { name: "Fabric", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Fabric
item_registry.push(fabric.clone());
let polymer_resin = Arc::new(Item { name: "Polymer Resin", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/PolymerResin/Desc_PolymerResin
item_registry.push(polymer_resin.clone());
let water = Arc::new(Item { name: "Water", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/Water/Desc_Water
item_registry.push(water.clone());
let caterium_ore = Arc::new(Item { name: "Caterium Ore", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreGold/Desc_OreGold
item_registry.push(caterium_ore.clone());
let quartz_crystal = Arc::new(Item { name: "Quartz Crystal", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/QuartzCrystal/Desc_QuartzCrystal
item_registry.push(quartz_crystal.clone());
let raw_quartz = Arc::new(Item { name: "Raw Quartz", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/RawQuartz/Desc_RawQuartz
item_registry.push(raw_quartz.clone());
let fuel = Arc::new(Item { name: "Fuel", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Fuel/Desc_LiquidFuel
item_registry.push(fuel.clone());
let concrete = Arc::new(Item { name: "Concrete", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Cement/Desc_Cement
item_registry.push(concrete.clone());
let limestone = Arc::new(Item { name: "Limestone", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/Stone/Desc_Stone
item_registry.push(limestone.clone());
let iron_rod = Arc::new(Item { name: "Iron Rod", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/IronRod/Desc_IronRod
item_registry.push(iron_rod.clone());
let turbofuel = Arc::new(Item { name: "Turbofuel", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Turbofuel/Desc_LiquidTurboFuel
item_registry.push(turbofuel.clone());
let compacted_coal = Arc::new(Item { name: "Compacted Coal", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CompactedCoal/Desc_CompactedCoal
item_registry.push(compacted_coal.clone());
let aluminum_ingot = Arc::new(Item { name: "Aluminum Ingot", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumIngot/Desc_AluminumIngot
item_registry.push(aluminum_ingot.clone());
let aluminum_casing = Arc::new(Item { name: "Aluminum Casing", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumCasing/Desc_AluminumCasing
item_registry.push(aluminum_casing.clone());
let portable_miner = Arc::new(Item { name: "Portable Miner", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/PortableMiner/BP_ItemDescriptorPortableMiner
item_registry.push(portable_miner.clone());
let motor = Arc::new(Item { name: "Motor", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Motor/Desc_Motor
item_registry.push(motor.clone());
let steel_pipe = Arc::new(Item { name: "Steel Pipe", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelPipe/Desc_SteelPipe
item_registry.push(steel_pipe.clone());
let battery = Arc::new(Item { name: "Battery", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Battery/Desc_Battery
item_registry.push(battery.clone());
let sulfur = Arc::new(Item { name: "Sulfur", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/Sulfur/Desc_Sulfur
item_registry.push(sulfur.clone());
let alclad_aluminum_sheet = Arc::new(Item { name: "Alclad Aluminum Sheet", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumPlate/Desc_AluminumPlate
item_registry.push(alclad_aluminum_sheet.clone());
let cooling_system = Arc::new(Item { name: "Cooling System", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CoolingSystem/Desc_CoolingSystem
item_registry.push(cooling_system.clone());
let heat_sink = Arc::new(Item { name: "Heat Sink", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumPlateReinforced/Desc_AluminumPlateReinforced
item_registry.push(heat_sink.clone());
let nitrogen_gas = Arc::new(Item { name: "Nitrogen Gas", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/NitrogenGas/Desc_NitrogenGas
item_registry.push(nitrogen_gas.clone());
let electromagnetic_control_rod = Arc::new(Item { name: "Electromagnetic Control Rod", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/ElectromagneticControlRod/Desc_ElectromagneticControlRod
item_registry.push(electromagnetic_control_rod.clone());
let non_fissile_uranium = Arc::new(Item { name: "Non-fissile Uranium", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Non-FissibleUranium/Desc_NonFissibleUranium
item_registry.push(non_fissile_uranium.clone());
let uranium = Arc::new(Item { name: "Uranium", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreUranium/Desc_OreUranium
item_registry.push(uranium.clone());
let uranium_waste = Arc::new(Item { name: "Uranium Waste", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/NuclearWaste/Desc_NuclearWaste
item_registry.push(uranium_waste.clone());
let nitric_acid = Arc::new(Item { name: "Nitric Acid", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/NitricAcid/Desc_NitricAcid
item_registry.push(nitric_acid.clone());
let sulfuric_acid = Arc::new(Item { name: "Sulfuric Acid", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SulfuricAcid/Desc_SulfuricAcid
item_registry.push(sulfuric_acid.clone());
let fused_modular_frame = Arc::new(Item { name: "Fused Modular Frame", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrameFused/Desc_ModularFrameFused
item_registry.push(fused_modular_frame.clone());
let encased_plutonium_cell = Arc::new(Item { name: "Encased Plutonium Cell", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/PlutoniumCell/Desc_PlutoniumCell
item_registry.push(encased_plutonium_cell.clone());
let bauxite = Arc::new(Item { name: "Bauxite", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreBauxite/Desc_OreBauxite
item_registry.push(bauxite.clone());
let coal = Arc::new(Item { name: "Coal", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/Coal/Desc_Coal
item_registry.push(coal.clone());
let supercomputer = Arc::new(Item { name: "Supercomputer", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/ComputerSuper/Desc_ComputerSuper
item_registry.push(supercomputer.clone());
let radio_control_unit = Arc::new(Item { name: "Radio Control Unit", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrameLightweight/Desc_ModularFrameLightweight
item_registry.push(radio_control_unit.clone());
let plutonium_fuel_rod = Arc::new(Item { name: "Plutonium Fuel Rod", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/PlutoniumFuelRods/Desc_PlutoniumFuelRod
item_registry.push(plutonium_fuel_rod.clone());
let pressure_conversion_cube = Arc::new(Item { name: "Pressure Conversion Cube", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/PressureConversionCube/Desc_PressureConversionCube
item_registry.push(pressure_conversion_cube.clone());
let crystal_oscillator = Arc::new(Item { name: "Crystal Oscillator", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CrystalOscillator/Desc_CrystalOscillator
item_registry.push(crystal_oscillator.clone());
let computer = Arc::new(Item { name: "Computer", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Computer/Desc_Computer
item_registry.push(computer.clone());
let turbo_motor = Arc::new(Item { name: "Turbo Motor", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/MotorLightweight/Desc_MotorLightweight
item_registry.push(turbo_motor.clone());
let packaged_nitrogen_gas = Arc::new(Item { name: "Packaged Nitrogen Gas", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/PackagedNitrogen/Desc_PackagedNitrogenGas
item_registry.push(packaged_nitrogen_gas.clone());
let beacon = Arc::new(Item { name: "Beacon", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/Beacon/BP_EquipmentDescriptorBeacon
item_registry.push(beacon.clone());
let quickwire = Arc::new(Item { name: "Quickwire", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/HighSpeedWire/Desc_HighSpeedWire
item_registry.push(quickwire.clone());
let silica = Arc::new(Item { name: "Silica", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Silica/Desc_Silica
item_registry.push(silica.clone());
let wood = Arc::new(Item { name: "Wood", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Wood
item_registry.push(wood.clone());
let biomass = Arc::new(Item { name: "Biomass", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_GenericBiomass
item_registry.push(biomass.clone());
let ailimiter = Arc::new(Item { name: "AI Limiter", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CircuitBoardHighSpeed/Desc_CircuitBoardHighSpeed
item_registry.push(ailimiter.clone());
let black_powder = Arc::new(Item { name: "Black Powder", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GunPowder/Desc_Gunpowder
item_registry.push(black_powder.clone());
let uranium_fuel_rod = Arc::new(Item { name: "Uranium Fuel Rod", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/NuclearFuelRod/Desc_NuclearFuelRod
item_registry.push(uranium_fuel_rod.clone());
let encased_uranium_cell = Arc::new(Item { name: "Encased Uranium Cell", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/UraniumCell/Desc_UraniumCell
item_registry.push(encased_uranium_cell.clone());
let plutonium_pellet = Arc::new(Item { name: "Plutonium Pellet", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/PlutoniumPellet/Desc_PlutoniumPellet
item_registry.push(plutonium_pellet.clone());
let hubparts = Arc::new(Item { name: "HUB Parts", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/HUBParts/Desc_HUBParts
item_registry.push(hubparts.clone());
let alien_dnacapsule = Arc::new(Item { name: "Alien DNA Capsule", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AlienDNACapsule/Desc_AlienDNACapsule
item_registry.push(alien_dnacapsule.clone());
let alien_protein = Arc::new(Item { name: "Alien Protein", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AlienProtein/Desc_AlienProtein
item_registry.push(alien_protein.clone());
let solid_biofuel = Arc::new(Item { name: "Solid Biofuel", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/BioFuel/Desc_Biofuel
item_registry.push(solid_biofuel.clone());
let leaves = Arc::new(Item { name: "Leaves", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Leaves
item_registry.push(leaves.clone());
let mycelia = Arc::new(Item { name: "Mycelia", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Mycelia
item_registry.push(mycelia.clone());
let copper_powder = Arc::new(Item { name: "Copper Powder", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CopperDust/Desc_CopperDust
item_registry.push(copper_powder.clone());
let empty_fluid_tank = Arc::new(Item { name: "Empty Fluid Tank", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GasTank/Desc_GasTank
item_registry.push(empty_fluid_tank.clone());
let power_shard = Arc::new(Item { name: "Power Shard", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_CrystalShard
item_registry.push(power_shard.clone());
let blue_power_slug = Arc::new(Item { name: "Blue Power Slug", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_Crystal
item_registry.push(blue_power_slug.clone());
let yellow_power_slug = Arc::new(Item { name: "Yellow Power Slug", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_Crystal_mk2
item_registry.push(yellow_power_slug.clone());
let purple_power_slug = Arc::new(Item { name: "Purple Power Slug", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_Crystal_mk3
item_registry.push(purple_power_slug.clone());
let hatcher_remains = Arc::new(Item { name: "Hatcher Remains", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_HatcherParts
item_registry.push(hatcher_remains.clone());
let hog_remains = Arc::new(Item { name: "Hog Remains", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_HogParts
item_registry.push(hog_remains.clone());
let plasma_spitter_remains = Arc::new(Item { name: "Plasma Spitter Remains", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_SpitterParts
item_registry.push(plasma_spitter_remains.clone());
let stinger_remains = Arc::new(Item { name: "Stinger Remains", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_StingerParts
item_registry.push(stinger_remains.clone());
let blade_runners = Arc::new(Item { name: "Blade Runners", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/JumpingStilts/BP_EquipmentDescriptorJumpingStilts
item_registry.push(blade_runners.clone());
let candy_cane_basher = Arc::new(Item { name: "Candy Cane Basher", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/StunSpear/BP_EquipmentDescriptorCandyCane
item_registry.push(candy_cane_basher.clone());
let xeno_zapper = Arc::new(Item { name: "Xeno-Zapper", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/ShockShank/BP_EquipmentDescriptorShockShank
item_registry.push(xeno_zapper.clone());
let candy_cane = Arc::new(Item { name: "Candy Cane", raw: false}); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_CandyCane
item_registry.push(candy_cane.clone());
let ficsmasgift = Arc::new(Item { name: "FICSMAS Gift", raw: false}); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_Gift
item_registry.push(ficsmasgift.clone());
let rifle_ammo = Arc::new(Item { name: "Rifle Ammo", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/CartridgeStandard/Desc_CartridgeStandard
item_registry.push(rifle_ammo.clone());
let smokeless_powder = Arc::new(Item { name: "Smokeless Powder", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GunPowder/Desc_GunpowderMK2
item_registry.push(smokeless_powder.clone());
let chainsaw = Arc::new(Item { name: "Chainsaw", raw: false}); // FactoryGame/Content/FactoryGame/Equipment/Chainsaw/Desc_Chainsaw
item_registry.push(chainsaw.clone());
let color_cartridge = Arc::new(Item { name: "Color Cartridge", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/ColorCartridge/Desc_ColorCartridge
item_registry.push(color_cartridge.clone());
let flower_petals = Arc::new(Item { name: "Flower Petals", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_FlowerPetals
item_registry.push(flower_petals.clone());
let gas_filter = Arc::new(Item { name: "Gas Filter", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Filter/Desc_Filter
item_registry.push(gas_filter.clone());
let iodine_infused_filter = Arc::new(Item { name: "Iodine Infused Filter", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/IodineInfusedFilter/Desc_HazmatFilter
item_registry.push(iodine_infused_filter.clone());
let gas_mask = Arc::new(Item { name: "Gas Mask", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/GasMask/BP_EquipmentDescriptorGasmask
item_registry.push(gas_mask.clone());
let hazmat_suit = Arc::new(Item { name: "Hazmat Suit", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/HazmatSuit/BP_EquipmentDescriptorHazmatSuit
item_registry.push(hazmat_suit.clone());
let hover_pack = Arc::new(Item { name: "Hover Pack", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/HoverPack/BP_EquipmentDescriptorHoverPack
item_registry.push(hover_pack.clone());
let jetpack = Arc::new(Item { name: "Jetpack", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/JetPack/BP_EquipmentDescriptorJetPack
item_registry.push(jetpack.clone());
let medicinal_inhaler = Arc::new(Item { name: "Medicinal Inhaler", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/Medkit/Desc_Medkit
item_registry.push(medicinal_inhaler.clone());
let paleberry = Arc::new(Item { name: "Paleberry", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Environment/Berry/Desc_Berry
item_registry.push(paleberry.clone());
let beryl_nut = Arc::new(Item { name: "Beryl Nut", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Environment/Nut/Desc_Nut
item_registry.push(beryl_nut.clone());
let nobelisk = Arc::new(Item { name: "Nobelisk", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/NobeliskExplosive/Desc_NobeliskExplosive
item_registry.push(nobelisk.clone());
let nobelisk_detonator = Arc::new(Item { name: "Nobelisk Detonator", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/NobeliskDetonator/BP_EquipmentDescriptorNobeliskDetonator
item_registry.push(nobelisk_detonator.clone());
let object_scanner = Arc::new(Item { name: "Object Scanner", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/GemstoneScanner/BP_EquipmentDescriptorObjectScanner
item_registry.push(object_scanner.clone());
let bacon_agaric = Arc::new(Item { name: "Bacon Agaric", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Environment/DesertShroom/Desc_Shroom
item_registry.push(bacon_agaric.clone());
let parachute = Arc::new(Item { name: "Parachute", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/Beacon/Desc_Parachute
item_registry.push(parachute.clone());
let rebar_gun = Arc::new(Item { name: "Rebar Gun", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/NailGun/Desc_RebarGunProjectile
item_registry.push(rebar_gun.clone());
let rifle = Arc::new(Item { name: "Rifle", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/Rifle/BP_EquipmentDescriptorRifle
item_registry.push(rifle.clone());
let iron_rebar = Arc::new(Item { name: "Iron Rebar", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpikedRebar/Desc_SpikedRebar
item_registry.push(iron_rebar.clone());
let xeno_basher = Arc::new(Item { name: "Xeno-Basher", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/StunSpear/BP_EquipmentDescriptorStunSpear
item_registry.push(xeno_basher.clone());
let zipline = Arc::new(Item { name: "Zipline", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/Zipline/BP_EqDescZipLine
item_registry.push(zipline.clone());
let snowball = Arc::new(Item { name: "Snowball", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SnowballProjectile/Desc_SnowballProjectile
item_registry.push(snowball.clone());
let actual_snow = Arc::new(Item { name: "Actual Snow", raw: false}); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_Snow
item_registry.push(actual_snow.clone());
let ficsmastree_branch = Arc::new(Item { name: "FICSMAS Tree Branch", raw: false}); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_XmasBranch
item_registry.push(ficsmastree_branch.clone());
let copper_ficsmasornament = Arc::new(Item { name: "Copper FICSMAS Ornament", raw: false}); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_XmasBall3
item_registry.push(copper_ficsmasornament.clone());
let iron_ficsmasornament = Arc::new(Item { name: "Iron FICSMAS Ornament", raw: false}); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_XmasBall4
item_registry.push(iron_ficsmasornament.clone());
let liquid_biofuel = Arc::new(Item { name: "Liquid Biofuel", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/BioFuel/Desc_LiquidBiofuel
item_registry.push(liquid_biofuel.clone());
let packaged_liquid_biofuel = Arc::new(Item { name: "Packaged Liquid Biofuel", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/BioFuel/Desc_PackagedBiofuel
item_registry.push(packaged_liquid_biofuel.clone());
let packaged_oil = Arc::new(Item { name: "Packaged Oil", raw: true}); // FactoryGame/Content/FactoryGame/Resource/RawResources/CrudeOil/Desc_PackagedOil
item_registry.push(packaged_oil.clone());
let packaged_heavy_oil_residue = Arc::new(Item { name: "Packaged Heavy Oil Residue", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/HeavyOilResidue/Desc_PackagedOilResidue
item_registry.push(packaged_heavy_oil_residue.clone());
let packaged_turbofuel = Arc::new(Item { name: "Packaged Turbofuel", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Turbofuel/Desc_TurboFuel
item_registry.push(packaged_turbofuel.clone());
let packaged_alumina_solution = Arc::new(Item { name: "Packaged Alumina Solution", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/Alumina/Desc_PackagedAlumina
item_registry.push(packaged_alumina_solution.clone());
let packaged_nitric_acid = Arc::new(Item { name: "Packaged Nitric Acid", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/NitricAcid/Desc_PackagedNitricAcid
item_registry.push(packaged_nitric_acid.clone());
let packaged_sulfuric_acid = Arc::new(Item { name: "Packaged Sulfuric Acid", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SulfuricAcid/Desc_PackagedSulfuricAcid
item_registry.push(packaged_sulfuric_acid.clone());
let modular_engine = Arc::new(Item { name: "Modular Engine", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_4
item_registry.push(modular_engine.clone());
let adaptive_control_unit = Arc::new(Item { name: "Adaptive Control Unit", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_5
item_registry.push(adaptive_control_unit.clone());
let magnetic_field_generator = Arc::new(Item { name: "Magnetic Field Generator", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_6
item_registry.push(magnetic_field_generator.clone());
let assembly_director_system = Arc::new(Item { name: "Assembly Director System", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_7
item_registry.push(assembly_director_system.clone());
let thermal_propulsion_rocket = Arc::new(Item { name: "Thermal Propulsion Rocket", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_8
item_registry.push(thermal_propulsion_rocket.clone());
let nuclear_pasta = Arc::new(Item { name: "Nuclear Pasta", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_9
item_registry.push(nuclear_pasta.clone());
let factory_cart = Arc::new(Item { name: "Factory Cart™", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/GolfCart/Desc_GolfCart
item_registry.push(factory_cart.clone());
let golden_factory_cart = Arc::new(Item { name: "Golden Factory Cart™", raw: false}); // FactoryGame/Content/FactoryGame/Resource/Equipment/GolfCart/Desc_GolfCartGold
item_registry.push(golden_factory_cart.clone());

let alternate_adhered_iron_plate_recipe = Recipe { name: "Alternate: Adhered Iron Plate", machine: Machine::Assembler, input: vec![ItemAmount {item: iron_plate.clone(), amount: 3 }, ItemAmount {item: rubber.clone(), amount: 1 }, ], output: ItemAmount {item: reinforced_iron_plate.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_AdheredIronPlate
recipe_registry.entry("Reinforced Iron Plate").or_insert_with(Vec::new).push(alternate_adhered_iron_plate_recipe);
let alternate_bolted_frame_recipe = Recipe { name: "Alternate: Bolted Frame", machine: Machine::Assembler, input: vec![ItemAmount {item: reinforced_iron_plate.clone(), amount: 3 }, ItemAmount {item: screw.clone(), amount: 56 }, ], output: ItemAmount {item: modular_frame.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_BoltedFrame
recipe_registry.entry("Modular Frame").or_insert_with(Vec::new).push(alternate_bolted_frame_recipe);
let alternate_coated_cable_recipe = Recipe { name: "Alternate: Coated Cable", machine: Machine::Refinery, input: vec![ItemAmount {item: wire.clone(), amount: 5 }, ItemAmount {item: heavy_oil_residue.clone(), amount: 2 }, ], output: ItemAmount {item: cable.clone(), amount: 9 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_CoatedCable
recipe_registry.entry("Cable").or_insert_with(Vec::new).push(alternate_coated_cable_recipe);
let alternate_coated_iron_canister_recipe = Recipe { name: "Alternate: Coated Iron Canister", machine: Machine::Assembler, input: vec![ItemAmount {item: iron_plate.clone(), amount: 2 }, ItemAmount {item: copper_sheet.clone(), amount: 1 }, ], output: ItemAmount {item: empty_canister.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 4f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_CoatedIronCanister
recipe_registry.entry("Empty Canister").or_insert_with(Vec::new).push(alternate_coated_iron_canister_recipe);
let alternate_coated_iron_plate_recipe = Recipe { name: "Alternate: Coated Iron Plate", machine: Machine::Assembler, input: vec![ItemAmount {item: iron_ingot.clone(), amount: 10 }, ItemAmount {item: plastic.clone(), amount: 2 }, ], output: ItemAmount {item: iron_plate.clone(), amount: 15 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_CoatedIronPlate
recipe_registry.entry("Iron Plate").or_insert_with(Vec::new).push(alternate_coated_iron_plate_recipe);
let alternate_coke_steel_ingot_recipe = Recipe { name: "Alternate: Coke Steel Ingot", machine: Machine::Foundry, input: vec![ItemAmount {item: iron_ore.clone(), amount: 15 }, ItemAmount {item: petroleum_coke.clone(), amount: 15 }, ], output: ItemAmount {item: steel_ingot.clone(), amount: 20 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_CokeSteelIngot
recipe_registry.entry("Steel Ingot").or_insert_with(Vec::new).push(alternate_coke_steel_ingot_recipe);
let alternate_copper_alloy_ingot_recipe = Recipe { name: "Alternate: Copper Alloy Ingot", machine: Machine::Foundry, input: vec![ItemAmount {item: copper_ore.clone(), amount: 10 }, ItemAmount {item: iron_ore.clone(), amount: 5 }, ], output: ItemAmount {item: copper_ingot.clone(), amount: 20 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_CopperAlloyIngot
recipe_registry.entry("Copper Ingot").or_insert_with(Vec::new).push(alternate_copper_alloy_ingot_recipe);
let alternate_copper_rotor_recipe = Recipe { name: "Alternate: Copper Rotor", machine: Machine::Assembler, input: vec![ItemAmount {item: copper_sheet.clone(), amount: 6 }, ItemAmount {item: screw.clone(), amount: 52 }, ], output: ItemAmount {item: rotor.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_CopperRotor
recipe_registry.entry("Rotor").or_insert_with(Vec::new).push(alternate_copper_rotor_recipe);
let alternate_diluted_packaged_fuel_recipe = Recipe { name: "Alternate: Diluted Packaged Fuel", machine: Machine::Refinery, input: vec![ItemAmount {item: heavy_oil_residue.clone(), amount: 1 }, ItemAmount {item: packaged_water.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_fuel.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 2f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_DilutedPackagedFuel
recipe_registry.entry("Packaged Fuel").or_insert_with(Vec::new).push(alternate_diluted_packaged_fuel_recipe);
let alternate_electrode_aluminum_scrap_recipe = Recipe { name: "Alternate: Electrode - Aluminum Scrap", machine: Machine::Refinery, input: vec![ItemAmount {item: alumina_solution.clone(), amount: 12 }, ItemAmount {item: petroleum_coke.clone(), amount: 4 }, ], output: ItemAmount {item: aluminum_scrap.clone(), amount: 20 }, byproduct: Some(ItemAmount {item: water.clone(), amount: 7 }), manufacturing_duration: 4f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_ElectroAluminumScrap
recipe_registry.entry("Aluminum Scrap").or_insert_with(Vec::new).push(alternate_electrode_aluminum_scrap_recipe);
let alternate_electrode_circuit_board_recipe = Recipe { name: "Alternate: Electrode Circuit Board", machine: Machine::Assembler, input: vec![ItemAmount {item: rubber.clone(), amount: 6 }, ItemAmount {item: petroleum_coke.clone(), amount: 9 }, ], output: ItemAmount {item: circuit_board.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_ElectrodeCircuitBoard
recipe_registry.entry("Circuit Board").or_insert_with(Vec::new).push(alternate_electrode_circuit_board_recipe);
let alternate_flexible_framework_recipe = Recipe { name: "Alternate: Flexible Framework", machine: Machine::Manufacturer, input: vec![ItemAmount {item: modular_frame.clone(), amount: 1 }, ItemAmount {item: steel_beam.clone(), amount: 6 }, ItemAmount {item: rubber.clone(), amount: 8 }, ], output: ItemAmount {item: versatile_framework.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_FlexibleFramework
recipe_registry.entry("Versatile Framework").or_insert_with(Vec::new).push(alternate_flexible_framework_recipe);
let alternate_fused_wire_recipe = Recipe { name: "Alternate: Fused Wire", machine: Machine::Assembler, input: vec![ItemAmount {item: copper_ingot.clone(), amount: 4 }, ItemAmount {item: caterium_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: wire.clone(), amount: 30 }, byproduct: None, manufacturing_duration: 20f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_FusedWire
recipe_registry.entry("Wire").or_insert_with(Vec::new).push(alternate_fused_wire_recipe);
let alternate_heavy_flexible_frame_recipe = Recipe { name: "Alternate: Heavy Flexible Frame", machine: Machine::Manufacturer, input: vec![ItemAmount {item: modular_frame.clone(), amount: 5 }, ItemAmount {item: encased_industrial_beam.clone(), amount: 3 }, ItemAmount {item: rubber.clone(), amount: 20 }, ItemAmount {item: screw.clone(), amount: 104 }, ], output: ItemAmount {item: heavy_modular_frame.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_HeavyFlexibleFrame
recipe_registry.entry("Heavy Modular Frame").or_insert_with(Vec::new).push(alternate_heavy_flexible_frame_recipe);
let alternate_heavy_oil_residue_recipe = Recipe { name: "Alternate: Heavy Oil Residue", machine: Machine::Refinery, input: vec![ItemAmount {item: crude_oil.clone(), amount: 3 }, ], output: ItemAmount {item: heavy_oil_residue.clone(), amount: 4 }, byproduct: Some(ItemAmount {item: polymer_resin.clone(), amount: 2 }), manufacturing_duration: 6f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_HeavyOilResidue
recipe_registry.entry("Heavy Oil Residue").or_insert_with(Vec::new).push(alternate_heavy_oil_residue_recipe);
let alternate_automated_speed_wiring_recipe = Recipe { name: "Alternate: Automated Speed Wiring", machine: Machine::Manufacturer, input: vec![ItemAmount {item: stator.clone(), amount: 2 }, ItemAmount {item: wire.clone(), amount: 40 }, ItemAmount {item: high_speed_connector.clone(), amount: 1 }, ], output: ItemAmount {item: automated_wiring.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 32f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_HighSpeedWiring
recipe_registry.entry("Automated Wiring").or_insert_with(Vec::new).push(alternate_automated_speed_wiring_recipe);
let alternate_plastic_smart_plating_recipe = Recipe { name: "Alternate: Plastic Smart Plating", machine: Machine::Manufacturer, input: vec![ItemAmount {item: reinforced_iron_plate.clone(), amount: 1 }, ItemAmount {item: rotor.clone(), amount: 1 }, ItemAmount {item: plastic.clone(), amount: 3 }, ], output: ItemAmount {item: smart_plating.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_PlasticSmartPlating
recipe_registry.entry("Smart Plating").or_insert_with(Vec::new).push(alternate_plastic_smart_plating_recipe);
let alternate_polyester_fabric_recipe = Recipe { name: "Alternate: Polyester Fabric", machine: Machine::Refinery, input: vec![ItemAmount {item: polymer_resin.clone(), amount: 1 }, ItemAmount {item: water.clone(), amount: 1 }, ], output: ItemAmount {item: fabric.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 2f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_PolyesterFabric
recipe_registry.entry("Fabric").or_insert_with(Vec::new).push(alternate_polyester_fabric_recipe);
let alternate_polymer_resin_recipe = Recipe { name: "Alternate: Polymer Resin", machine: Machine::Refinery, input: vec![ItemAmount {item: crude_oil.clone(), amount: 6 }, ], output: ItemAmount {item: polymer_resin.clone(), amount: 13 }, byproduct: Some(ItemAmount {item: heavy_oil_residue.clone(), amount: 2 }), manufacturing_duration: 6f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_PolymerResin
recipe_registry.entry("Polymer Resin").or_insert_with(Vec::new).push(alternate_polymer_resin_recipe);
let alternate_pure_caterium_ingot_recipe = Recipe { name: "Alternate: Pure Caterium Ingot", machine: Machine::Refinery, input: vec![ItemAmount {item: caterium_ore.clone(), amount: 2 }, ItemAmount {item: water.clone(), amount: 2 }, ], output: ItemAmount {item: caterium_ingot.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 5f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_PureCateriumIngot
recipe_registry.entry("Caterium Ingot").or_insert_with(Vec::new).push(alternate_pure_caterium_ingot_recipe);
let alternate_pure_copper_ingot_recipe = Recipe { name: "Alternate: Pure Copper Ingot", machine: Machine::Refinery, input: vec![ItemAmount {item: copper_ore.clone(), amount: 6 }, ItemAmount {item: water.clone(), amount: 4 }, ], output: ItemAmount {item: copper_ingot.clone(), amount: 15 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_PureCopperIngot
recipe_registry.entry("Copper Ingot").or_insert_with(Vec::new).push(alternate_pure_copper_ingot_recipe);
let alternate_pure_iron_ingot_recipe = Recipe { name: "Alternate: Pure Iron Ingot", machine: Machine::Refinery, input: vec![ItemAmount {item: iron_ore.clone(), amount: 7 }, ItemAmount {item: water.clone(), amount: 4 }, ], output: ItemAmount {item: iron_ingot.clone(), amount: 13 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_PureIronIngot
recipe_registry.entry("Iron Ingot").or_insert_with(Vec::new).push(alternate_pure_iron_ingot_recipe);
let alternate_pure_quartz_crystal_recipe = Recipe { name: "Alternate: Pure Quartz Crystal", machine: Machine::Refinery, input: vec![ItemAmount {item: raw_quartz.clone(), amount: 9 }, ItemAmount {item: water.clone(), amount: 5 }, ], output: ItemAmount {item: quartz_crystal.clone(), amount: 7 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_PureQuartzCrystal
recipe_registry.entry("Quartz Crystal").or_insert_with(Vec::new).push(alternate_pure_quartz_crystal_recipe);
let alternate_recycled_rubber_recipe = Recipe { name: "Alternate: Recycled Rubber", machine: Machine::Refinery, input: vec![ItemAmount {item: plastic.clone(), amount: 6 }, ItemAmount {item: fuel.clone(), amount: 6 }, ], output: ItemAmount {item: rubber.clone(), amount: 12 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_RecycledRubber
recipe_registry.entry("Rubber").or_insert_with(Vec::new).push(alternate_recycled_rubber_recipe);
let alternate_rubber_concrete_recipe = Recipe { name: "Alternate: Rubber Concrete", machine: Machine::Assembler, input: vec![ItemAmount {item: limestone.clone(), amount: 10 }, ItemAmount {item: rubber.clone(), amount: 2 }, ], output: ItemAmount {item: concrete.clone(), amount: 9 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_RubberConcrete
recipe_registry.entry("Concrete").or_insert_with(Vec::new).push(alternate_rubber_concrete_recipe);
let alternate_steamed_copper_sheet_recipe = Recipe { name: "Alternate: Steamed Copper Sheet", machine: Machine::Refinery, input: vec![ItemAmount {item: copper_ingot.clone(), amount: 3 }, ItemAmount {item: water.clone(), amount: 3 }, ], output: ItemAmount {item: copper_sheet.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_SteamedCopperSheet
recipe_registry.entry("Copper Sheet").or_insert_with(Vec::new).push(alternate_steamed_copper_sheet_recipe);
let alternate_steel_canister_recipe = Recipe { name: "Alternate: Steel Canister", machine: Machine::Constructor, input: vec![ItemAmount {item: steel_ingot.clone(), amount: 3 }, ], output: ItemAmount {item: empty_canister.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 3f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_SteelCanister
recipe_registry.entry("Empty Canister").or_insert_with(Vec::new).push(alternate_steel_canister_recipe);
let alternate_steel_coated_plate_recipe = Recipe { name: "Alternate: Steel Coated Plate", machine: Machine::Assembler, input: vec![ItemAmount {item: steel_ingot.clone(), amount: 3 }, ItemAmount {item: plastic.clone(), amount: 2 }, ], output: ItemAmount {item: iron_plate.clone(), amount: 18 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_SteelCoatedPlate
recipe_registry.entry("Iron Plate").or_insert_with(Vec::new).push(alternate_steel_coated_plate_recipe);
let alternate_steel_rod_recipe = Recipe { name: "Alternate: Steel Rod", machine: Machine::Constructor, input: vec![ItemAmount {item: steel_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: iron_rod.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 5f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_SteelRod
recipe_registry.entry("Iron Rod").or_insert_with(Vec::new).push(alternate_steel_rod_recipe);
let alternate_turbo_heavy_fuel_recipe = Recipe { name: "Alternate: Turbo Heavy Fuel", machine: Machine::Refinery, input: vec![ItemAmount {item: heavy_oil_residue.clone(), amount: 5 }, ItemAmount {item: compacted_coal.clone(), amount: 4 }, ], output: ItemAmount {item: turbofuel.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_TurboHeavyFuel
recipe_registry.entry("Turbofuel").or_insert_with(Vec::new).push(alternate_turbo_heavy_fuel_recipe);
let alternate_wet_concrete_recipe = Recipe { name: "Alternate: Wet Concrete", machine: Machine::Refinery, input: vec![ItemAmount {item: limestone.clone(), amount: 6 }, ItemAmount {item: water.clone(), amount: 5 }, ], output: ItemAmount {item: concrete.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 3f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_Alternate_WetConcrete
recipe_registry.entry("Concrete").or_insert_with(Vec::new).push(alternate_wet_concrete_recipe);
let alternate_pure_aluminum_ingot_recipe = Recipe { name: "Alternate: Pure Aluminum Ingot", machine: Machine::Smelter, input: vec![ItemAmount {item: aluminum_scrap.clone(), amount: 2 }, ], output: ItemAmount {item: aluminum_ingot.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 2f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update3/Recipe_PureAluminumIngot
recipe_registry.entry("Aluminum Ingot").or_insert_with(Vec::new).push(alternate_pure_aluminum_ingot_recipe);
let alternate_alclad_casing_recipe = Recipe { name: "Alternate: Alclad Casing", machine: Machine::Assembler, input: vec![ItemAmount {item: aluminum_ingot.clone(), amount: 20 }, ItemAmount {item: copper_ingot.clone(), amount: 10 }, ], output: ItemAmount {item: aluminum_casing.clone(), amount: 15 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_AlcladCasing
recipe_registry.entry("Aluminum Casing").or_insert_with(Vec::new).push(alternate_alclad_casing_recipe);
let alternate_automated_miner_recipe = Recipe { name: "Alternate: Automated Miner", machine: Machine::Manufacturer, input: vec![ItemAmount {item: motor.clone(), amount: 1 }, ItemAmount {item: steel_pipe.clone(), amount: 4 }, ItemAmount {item: iron_rod.clone(), amount: 4 }, ItemAmount {item: iron_plate.clone(), amount: 2 }, ], output: ItemAmount {item: portable_miner.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 60f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_AutomatedMiner
recipe_registry.entry("Portable Miner").or_insert_with(Vec::new).push(alternate_automated_miner_recipe);
let alternate_classic_battery_recipe = Recipe { name: "Alternate: Classic Battery", machine: Machine::Manufacturer, input: vec![ItemAmount {item: sulfur.clone(), amount: 6 }, ItemAmount {item: alclad_aluminum_sheet.clone(), amount: 7 }, ItemAmount {item: plastic.clone(), amount: 8 }, ItemAmount {item: wire.clone(), amount: 12 }, ], output: ItemAmount {item: battery.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_ClassicBattery
recipe_registry.entry("Battery").or_insert_with(Vec::new).push(alternate_classic_battery_recipe);
let alternate_cooling_device_recipe = Recipe { name: "Alternate: Cooling Device", machine: Machine::Blender, input: vec![ItemAmount {item: heat_sink.clone(), amount: 5 }, ItemAmount {item: motor.clone(), amount: 1 }, ItemAmount {item: nitrogen_gas.clone(), amount: 24 }, ], output: ItemAmount {item: cooling_system.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 32f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_CoolingDevice
recipe_registry.entry("Cooling System").or_insert_with(Vec::new).push(alternate_cooling_device_recipe);
let alternate_diluted_fuel_recipe = Recipe { name: "Alternate: Diluted Fuel", machine: Machine::Blender, input: vec![ItemAmount {item: heavy_oil_residue.clone(), amount: 5 }, ItemAmount {item: water.clone(), amount: 10 }, ], output: ItemAmount {item: fuel.clone(), amount: 10 }, byproduct: None, manufacturing_duration: 6f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_DilutedFuel
recipe_registry.entry("Fuel").or_insert_with(Vec::new).push(alternate_diluted_fuel_recipe);
let alternate_electric_motor_recipe = Recipe { name: "Alternate: Electric Motor", machine: Machine::Assembler, input: vec![ItemAmount {item: electromagnetic_control_rod.clone(), amount: 1 }, ItemAmount {item: rotor.clone(), amount: 2 }, ], output: ItemAmount {item: motor.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_ElectricMotor
recipe_registry.entry("Motor").or_insert_with(Vec::new).push(alternate_electric_motor_recipe);
let alternate_fertile_uranium_recipe = Recipe { name: "Alternate: Fertile Uranium", machine: Machine::Blender, input: vec![ItemAmount {item: uranium.clone(), amount: 5 }, ItemAmount {item: uranium_waste.clone(), amount: 5 }, ItemAmount {item: nitric_acid.clone(), amount: 3 }, ItemAmount {item: sulfuric_acid.clone(), amount: 5 }, ], output: ItemAmount {item: non_fissile_uranium.clone(), amount: 20 }, byproduct: Some(ItemAmount {item: water.clone(), amount: 8 }), manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_FertileUranium
recipe_registry.entry("Non-fissile Uranium").or_insert_with(Vec::new).push(alternate_fertile_uranium_recipe);
let alternate_heat_fused_frame_recipe = Recipe { name: "Alternate: Heat-Fused Frame", machine: Machine::Blender, input: vec![ItemAmount {item: heavy_modular_frame.clone(), amount: 1 }, ItemAmount {item: aluminum_ingot.clone(), amount: 50 }, ItemAmount {item: nitric_acid.clone(), amount: 8 }, ItemAmount {item: fuel.clone(), amount: 10 }, ], output: ItemAmount {item: fused_modular_frame.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 20f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_HeatFusedFrame
recipe_registry.entry("Fused Modular Frame").or_insert_with(Vec::new).push(alternate_heat_fused_frame_recipe);
let alternate_instant_scrap_recipe = Recipe { name: "Alternate: Instant Scrap", machine: Machine::Blender, input: vec![ItemAmount {item: bauxite.clone(), amount: 15 }, ItemAmount {item: coal.clone(), amount: 10 }, ItemAmount {item: sulfuric_acid.clone(), amount: 5 }, ItemAmount {item: water.clone(), amount: 6 }, ], output: ItemAmount {item: aluminum_scrap.clone(), amount: 30 }, byproduct: Some(ItemAmount {item: water.clone(), amount: 5 }), manufacturing_duration: 6f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_InstantScrap
recipe_registry.entry("Aluminum Scrap").or_insert_with(Vec::new).push(alternate_instant_scrap_recipe);
let alternate_ocsupercomputer_recipe = Recipe { name: "Alternate: OC Supercomputer", machine: Machine::Assembler, input: vec![ItemAmount {item: radio_control_unit.clone(), amount: 3 }, ItemAmount {item: cooling_system.clone(), amount: 3 }, ], output: ItemAmount {item: supercomputer.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 20f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_OCSupercomputer
recipe_registry.entry("Supercomputer").or_insert_with(Vec::new).push(alternate_ocsupercomputer_recipe);
let alternate_plutonium_fuel_unit_recipe = Recipe { name: "Alternate: Plutonium Fuel Unit", machine: Machine::Assembler, input: vec![ItemAmount {item: encased_plutonium_cell.clone(), amount: 20 }, ItemAmount {item: pressure_conversion_cube.clone(), amount: 1 }, ], output: ItemAmount {item: plutonium_fuel_rod.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 120f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_PlutoniumFuelUnit
recipe_registry.entry("Plutonium Fuel Rod").or_insert_with(Vec::new).push(alternate_plutonium_fuel_unit_recipe);
let alternate_radio_control_system_recipe = Recipe { name: "Alternate: Radio Control System", machine: Machine::Manufacturer, input: vec![ItemAmount {item: crystal_oscillator.clone(), amount: 1 }, ItemAmount {item: circuit_board.clone(), amount: 10 }, ItemAmount {item: aluminum_casing.clone(), amount: 60 }, ItemAmount {item: rubber.clone(), amount: 30 }, ], output: ItemAmount {item: radio_control_unit.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 40f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_RadioControlSystem
recipe_registry.entry("Radio Control Unit").or_insert_with(Vec::new).push(alternate_radio_control_system_recipe);
let alternate_sloppy_alumina_recipe = Recipe { name: "Alternate: Sloppy Alumina", machine: Machine::Refinery, input: vec![ItemAmount {item: bauxite.clone(), amount: 10 }, ItemAmount {item: water.clone(), amount: 10 }, ], output: ItemAmount {item: alumina_solution.clone(), amount: 12 }, byproduct: None, manufacturing_duration: 3f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_SloppyAlumina
recipe_registry.entry("Alumina Solution").or_insert_with(Vec::new).push(alternate_sloppy_alumina_recipe);
let alternate_super_state_computer_recipe = Recipe { name: "Alternate: Super-State Computer", machine: Machine::Manufacturer, input: vec![ItemAmount {item: computer.clone(), amount: 3 }, ItemAmount {item: electromagnetic_control_rod.clone(), amount: 2 }, ItemAmount {item: battery.clone(), amount: 20 }, ItemAmount {item: wire.clone(), amount: 45 }, ], output: ItemAmount {item: supercomputer.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 50f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_SuperStateComputer
recipe_registry.entry("Supercomputer").or_insert_with(Vec::new).push(alternate_super_state_computer_recipe);
let alternate_turbo_blend_fuel_recipe = Recipe { name: "Alternate: Turbo Blend Fuel", machine: Machine::Blender, input: vec![ItemAmount {item: fuel.clone(), amount: 2 }, ItemAmount {item: heavy_oil_residue.clone(), amount: 4 }, ItemAmount {item: sulfur.clone(), amount: 3 }, ItemAmount {item: petroleum_coke.clone(), amount: 3 }, ], output: ItemAmount {item: turbofuel.clone(), amount: 6 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_TurboBlendFuel
recipe_registry.entry("Turbofuel").or_insert_with(Vec::new).push(alternate_turbo_blend_fuel_recipe);
let alternate_turbo_pressure_motor_recipe = Recipe { name: "Alternate: Turbo Pressure Motor", machine: Machine::Manufacturer, input: vec![ItemAmount {item: motor.clone(), amount: 4 }, ItemAmount {item: pressure_conversion_cube.clone(), amount: 1 }, ItemAmount {item: packaged_nitrogen_gas.clone(), amount: 24 }, ItemAmount {item: stator.clone(), amount: 8 }, ], output: ItemAmount {item: turbo_motor.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 32f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/New_Update4/Recipe_Alternate_TurboPressureMotor
recipe_registry.entry("Turbo Motor").or_insert_with(Vec::new).push(alternate_turbo_pressure_motor_recipe);
let alternate_crystal_beacon_recipe = Recipe { name: "Alternate: Crystal Beacon", machine: Machine::Manufacturer, input: vec![ItemAmount {item: steel_beam.clone(), amount: 4 }, ItemAmount {item: steel_pipe.clone(), amount: 16 }, ItemAmount {item: crystal_oscillator.clone(), amount: 1 }, ], output: ItemAmount {item: beacon.clone(), amount: 20 }, byproduct: None, manufacturing_duration: 120f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Beacon_1
recipe_registry.entry("Beacon").or_insert_with(Vec::new).push(alternate_crystal_beacon_recipe);
let alternate_insulated_cable_recipe = Recipe { name: "Alternate: Insulated Cable", machine: Machine::Assembler, input: vec![ItemAmount {item: wire.clone(), amount: 9 }, ItemAmount {item: rubber.clone(), amount: 6 }, ], output: ItemAmount {item: cable.clone(), amount: 20 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Cable_1
recipe_registry.entry("Cable").or_insert_with(Vec::new).push(alternate_insulated_cable_recipe);
let alternate_quickwire_cable_recipe = Recipe { name: "Alternate: Quickwire Cable", machine: Machine::Assembler, input: vec![ItemAmount {item: quickwire.clone(), amount: 3 }, ItemAmount {item: rubber.clone(), amount: 2 }, ], output: ItemAmount {item: cable.clone(), amount: 11 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Cable_2
recipe_registry.entry("Cable").or_insert_with(Vec::new).push(alternate_quickwire_cable_recipe);
let alternate_silicon_circuit_board_recipe = Recipe { name: "Alternate: Silicon Circuit Board", machine: Machine::Assembler, input: vec![ItemAmount {item: copper_sheet.clone(), amount: 11 }, ItemAmount {item: silica.clone(), amount: 11 }, ], output: ItemAmount {item: circuit_board.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_CircuitBoard_1
recipe_registry.entry("Circuit Board").or_insert_with(Vec::new).push(alternate_silicon_circuit_board_recipe);
let alternate_caterium_circuit_board_recipe = Recipe { name: "Alternate: Caterium Circuit Board", machine: Machine::Assembler, input: vec![ItemAmount {item: plastic.clone(), amount: 10 }, ItemAmount {item: quickwire.clone(), amount: 30 }, ], output: ItemAmount {item: circuit_board.clone(), amount: 7 }, byproduct: None, manufacturing_duration: 48f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_CircuitBoard_2
recipe_registry.entry("Circuit Board").or_insert_with(Vec::new).push(alternate_caterium_circuit_board_recipe);
let alternate_charcoal_recipe = Recipe { name: "Alternate: Charcoal", machine: Machine::Constructor, input: vec![ItemAmount {item: wood.clone(), amount: 1 }, ], output: ItemAmount {item: coal.clone(), amount: 10 }, byproduct: None, manufacturing_duration: 4f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Coal_1
recipe_registry.entry("Coal").or_insert_with(Vec::new).push(alternate_charcoal_recipe);
let alternate_biocoal_recipe = Recipe { name: "Alternate: Biocoal", machine: Machine::Constructor, input: vec![ItemAmount {item: biomass.clone(), amount: 5 }, ], output: ItemAmount {item: coal.clone(), amount: 6 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Coal_2
recipe_registry.entry("Coal").or_insert_with(Vec::new).push(alternate_biocoal_recipe);
let alternate_caterium_computer_recipe = Recipe { name: "Alternate: Caterium Computer", machine: Machine::Manufacturer, input: vec![ItemAmount {item: circuit_board.clone(), amount: 7 }, ItemAmount {item: quickwire.clone(), amount: 28 }, ItemAmount {item: rubber.clone(), amount: 12 }, ], output: ItemAmount {item: computer.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Computer_1
recipe_registry.entry("Computer").or_insert_with(Vec::new).push(alternate_caterium_computer_recipe);
let alternate_crystal_computer_recipe = Recipe { name: "Alternate: Crystal Computer", machine: Machine::Assembler, input: vec![ItemAmount {item: circuit_board.clone(), amount: 8 }, ItemAmount {item: crystal_oscillator.clone(), amount: 3 }, ], output: ItemAmount {item: computer.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 64f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Computer_2
recipe_registry.entry("Computer").or_insert_with(Vec::new).push(alternate_crystal_computer_recipe);
let alternate_fine_concrete_recipe = Recipe { name: "Alternate: Fine Concrete", machine: Machine::Assembler, input: vec![ItemAmount {item: silica.clone(), amount: 3 }, ItemAmount {item: limestone.clone(), amount: 12 }, ], output: ItemAmount {item: concrete.clone(), amount: 10 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Concrete
recipe_registry.entry("Concrete").or_insert_with(Vec::new).push(alternate_fine_concrete_recipe);
let alternate_insulated_crystal_oscillator_recipe = Recipe { name: "Alternate: Insulated Crystal Oscillator", machine: Machine::Manufacturer, input: vec![ItemAmount {item: quartz_crystal.clone(), amount: 10 }, ItemAmount {item: rubber.clone(), amount: 7 }, ItemAmount {item: ailimiter.clone(), amount: 1 }, ], output: ItemAmount {item: crystal_oscillator.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 32f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_CrystalOscillator
recipe_registry.entry("Crystal Oscillator").or_insert_with(Vec::new).push(alternate_insulated_crystal_oscillator_recipe);
let alternate_electromagnetic_connection_rod_recipe = Recipe { name: "Alternate: Electromagnetic Connection Rod", machine: Machine::Assembler, input: vec![ItemAmount {item: stator.clone(), amount: 2 }, ItemAmount {item: high_speed_connector.clone(), amount: 1 }, ], output: ItemAmount {item: electromagnetic_control_rod.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 15f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_ElectromagneticControlRod_1
recipe_registry.entry("Electromagnetic Control Rod").or_insert_with(Vec::new).push(alternate_electromagnetic_connection_rod_recipe);
let alternate_encased_industrial_pipe_recipe = Recipe { name: "Alternate: Encased Industrial Pipe", machine: Machine::Assembler, input: vec![ItemAmount {item: steel_pipe.clone(), amount: 7 }, ItemAmount {item: concrete.clone(), amount: 5 }, ], output: ItemAmount {item: encased_industrial_beam.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 15f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_EncasedIndustrialBeam
recipe_registry.entry("Encased Industrial Beam").or_insert_with(Vec::new).push(alternate_encased_industrial_pipe_recipe);
let alternate_compacted_coal_recipe = Recipe { name: "Alternate: Compacted Coal", machine: Machine::Assembler, input: vec![ItemAmount {item: coal.clone(), amount: 5 }, ItemAmount {item: sulfur.clone(), amount: 5 }, ], output: ItemAmount {item: compacted_coal.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_EnrichedCoal
recipe_registry.entry("Compacted Coal").or_insert_with(Vec::new).push(alternate_compacted_coal_recipe);
let alternate_fine_black_powder_recipe = Recipe { name: "Alternate: Fine Black Powder", machine: Machine::Assembler, input: vec![ItemAmount {item: sulfur.clone(), amount: 2 }, ItemAmount {item: compacted_coal.clone(), amount: 1 }, ], output: ItemAmount {item: black_powder.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Gunpowder_1
recipe_registry.entry("Black Powder").or_insert_with(Vec::new).push(alternate_fine_black_powder_recipe);
let alternate_heat_exchanger_recipe = Recipe { name: "Alternate: Heat Exchanger", machine: Machine::Assembler, input: vec![ItemAmount {item: aluminum_casing.clone(), amount: 3 }, ItemAmount {item: rubber.clone(), amount: 3 }, ], output: ItemAmount {item: heat_sink.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 6f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_HeatSink_1
recipe_registry.entry("Heat Sink").or_insert_with(Vec::new).push(alternate_heat_exchanger_recipe);
let alternate_silicon_high_speed_connector_recipe = Recipe { name: "Alternate: Silicon High-Speed Connector", machine: Machine::Manufacturer, input: vec![ItemAmount {item: quickwire.clone(), amount: 60 }, ItemAmount {item: silica.clone(), amount: 25 }, ItemAmount {item: circuit_board.clone(), amount: 2 }, ], output: ItemAmount {item: high_speed_connector.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 40f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_HighSpeedConnector
recipe_registry.entry("High-Speed Connector").or_insert_with(Vec::new).push(alternate_silicon_high_speed_connector_recipe);
let alternate_iron_alloy_ingot_recipe = Recipe { name: "Alternate: Iron Alloy Ingot", machine: Machine::Foundry, input: vec![ItemAmount {item: iron_ore.clone(), amount: 2 }, ItemAmount {item: copper_ore.clone(), amount: 2 }, ], output: ItemAmount {item: iron_ingot.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 6f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_IngotIron
recipe_registry.entry("Iron Ingot").or_insert_with(Vec::new).push(alternate_iron_alloy_ingot_recipe);
let alternate_solid_steel_ingot_recipe = Recipe { name: "Alternate: Solid Steel Ingot", machine: Machine::Foundry, input: vec![ItemAmount {item: iron_ingot.clone(), amount: 2 }, ItemAmount {item: coal.clone(), amount: 2 }, ], output: ItemAmount {item: steel_ingot.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 3f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_IngotSteel_1
recipe_registry.entry("Steel Ingot").or_insert_with(Vec::new).push(alternate_solid_steel_ingot_recipe);
let alternate_compacted_steel_ingot_recipe = Recipe { name: "Alternate: Compacted Steel Ingot", machine: Machine::Foundry, input: vec![ItemAmount {item: iron_ore.clone(), amount: 6 }, ItemAmount {item: compacted_coal.clone(), amount: 3 }, ], output: ItemAmount {item: steel_ingot.clone(), amount: 10 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_IngotSteel_2
recipe_registry.entry("Steel Ingot").or_insert_with(Vec::new).push(alternate_compacted_steel_ingot_recipe);
let alternate_steeled_frame_recipe = Recipe { name: "Alternate: Steeled Frame", machine: Machine::Assembler, input: vec![ItemAmount {item: reinforced_iron_plate.clone(), amount: 2 }, ItemAmount {item: steel_pipe.clone(), amount: 10 }, ], output: ItemAmount {item: modular_frame.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 60f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_ModularFrame
recipe_registry.entry("Modular Frame").or_insert_with(Vec::new).push(alternate_steeled_frame_recipe);
let alternate_heavy_encased_frame_recipe = Recipe { name: "Alternate: Heavy Encased Frame", machine: Machine::Manufacturer, input: vec![ItemAmount {item: modular_frame.clone(), amount: 8 }, ItemAmount {item: encased_industrial_beam.clone(), amount: 10 }, ItemAmount {item: steel_pipe.clone(), amount: 36 }, ItemAmount {item: concrete.clone(), amount: 22 }, ], output: ItemAmount {item: heavy_modular_frame.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 64f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_ModularFrameHeavy
recipe_registry.entry("Heavy Modular Frame").or_insert_with(Vec::new).push(alternate_heavy_encased_frame_recipe);
let alternate_rigour_motor_recipe = Recipe { name: "Alternate: Rigour Motor", machine: Machine::Manufacturer, input: vec![ItemAmount {item: rotor.clone(), amount: 3 }, ItemAmount {item: stator.clone(), amount: 3 }, ItemAmount {item: crystal_oscillator.clone(), amount: 1 }, ], output: ItemAmount {item: motor.clone(), amount: 6 }, byproduct: None, manufacturing_duration: 48f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Motor_1
recipe_registry.entry("Motor").or_insert_with(Vec::new).push(alternate_rigour_motor_recipe);
let alternate_uranium_fuel_unit_recipe = Recipe { name: "Alternate: Uranium Fuel Unit", machine: Machine::Manufacturer, input: vec![ItemAmount {item: encased_uranium_cell.clone(), amount: 100 }, ItemAmount {item: electromagnetic_control_rod.clone(), amount: 10 }, ItemAmount {item: crystal_oscillator.clone(), amount: 3 }, ItemAmount {item: beacon.clone(), amount: 6 }, ], output: ItemAmount {item: uranium_fuel_rod.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 300f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_NuclearFuelRod_1
recipe_registry.entry("Uranium Fuel Rod").or_insert_with(Vec::new).push(alternate_uranium_fuel_unit_recipe);
let alternate_recycled_plastic_recipe = Recipe { name: "Alternate: Recycled Plastic", machine: Machine::Refinery, input: vec![ItemAmount {item: rubber.clone(), amount: 6 }, ItemAmount {item: fuel.clone(), amount: 6 }, ], output: ItemAmount {item: plastic.clone(), amount: 12 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Plastic_1
recipe_registry.entry("Plastic").or_insert_with(Vec::new).push(alternate_recycled_plastic_recipe);
let alternate_fused_quickwire_recipe = Recipe { name: "Alternate: Fused Quickwire", machine: Machine::Assembler, input: vec![ItemAmount {item: caterium_ingot.clone(), amount: 1 }, ItemAmount {item: copper_ingot.clone(), amount: 5 }, ], output: ItemAmount {item: quickwire.clone(), amount: 12 }, byproduct: None, manufacturing_duration: 8f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Quickwire
recipe_registry.entry("Quickwire").or_insert_with(Vec::new).push(alternate_fused_quickwire_recipe);
let alternate_radio_connection_unit_recipe = Recipe { name: "Alternate: Radio Connection Unit", machine: Machine::Manufacturer, input: vec![ItemAmount {item: heat_sink.clone(), amount: 4 }, ItemAmount {item: high_speed_connector.clone(), amount: 2 }, ItemAmount {item: quartz_crystal.clone(), amount: 12 }, ], output: ItemAmount {item: radio_control_unit.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_RadioControlUnit_1
recipe_registry.entry("Radio Control Unit").or_insert_with(Vec::new).push(alternate_radio_connection_unit_recipe);
let alternate_bolted_iron_plate_recipe = Recipe { name: "Alternate: Bolted Iron Plate", machine: Machine::Assembler, input: vec![ItemAmount {item: iron_plate.clone(), amount: 18 }, ItemAmount {item: screw.clone(), amount: 50 }, ], output: ItemAmount {item: reinforced_iron_plate.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_ReinforcedIronPlate_1
recipe_registry.entry("Reinforced Iron Plate").or_insert_with(Vec::new).push(alternate_bolted_iron_plate_recipe);
let alternate_stitched_iron_plate_recipe = Recipe { name: "Alternate: Stitched Iron Plate", machine: Machine::Assembler, input: vec![ItemAmount {item: iron_plate.clone(), amount: 10 }, ItemAmount {item: wire.clone(), amount: 20 }, ], output: ItemAmount {item: reinforced_iron_plate.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 32f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_ReinforcedIronPlate_2
recipe_registry.entry("Reinforced Iron Plate").or_insert_with(Vec::new).push(alternate_stitched_iron_plate_recipe);
let alternate_steel_rotor_recipe = Recipe { name: "Alternate: Steel Rotor", machine: Machine::Assembler, input: vec![ItemAmount {item: steel_pipe.clone(), amount: 2 }, ItemAmount {item: wire.clone(), amount: 6 }, ], output: ItemAmount {item: rotor.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Rotor
recipe_registry.entry("Rotor").or_insert_with(Vec::new).push(alternate_steel_rotor_recipe);
let alternate_cast_screw_recipe = Recipe { name: "Alternate: Cast Screw", machine: Machine::Constructor, input: vec![ItemAmount {item: iron_ingot.clone(), amount: 5 }, ], output: ItemAmount {item: screw.clone(), amount: 20 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Screw
recipe_registry.entry("Screw").or_insert_with(Vec::new).push(alternate_cast_screw_recipe);
let alternate_steel_screw_recipe = Recipe { name: "Alternate: Steel Screw", machine: Machine::Constructor, input: vec![ItemAmount {item: steel_beam.clone(), amount: 1 }, ], output: ItemAmount {item: screw.clone(), amount: 52 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Screw_2
recipe_registry.entry("Screw").or_insert_with(Vec::new).push(alternate_steel_screw_recipe);
let alternate_cheap_silica_recipe = Recipe { name: "Alternate: Cheap Silica", machine: Machine::Assembler, input: vec![ItemAmount {item: raw_quartz.clone(), amount: 3 }, ItemAmount {item: limestone.clone(), amount: 5 }, ], output: ItemAmount {item: silica.clone(), amount: 7 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Silica
recipe_registry.entry("Silica").or_insert_with(Vec::new).push(alternate_cheap_silica_recipe);
let alternate_quickwire_stator_recipe = Recipe { name: "Alternate: Quickwire Stator", machine: Machine::Assembler, input: vec![ItemAmount {item: steel_pipe.clone(), amount: 4 }, ItemAmount {item: quickwire.clone(), amount: 15 }, ], output: ItemAmount {item: stator.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 15f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Stator
recipe_registry.entry("Stator").or_insert_with(Vec::new).push(alternate_quickwire_stator_recipe);
let turbofuel_recipe = Recipe { name: "Turbofuel", machine: Machine::Refinery, input: vec![ItemAmount {item: fuel.clone(), amount: 6 }, ItemAmount {item: compacted_coal.clone(), amount: 4 }, ], output: ItemAmount {item: turbofuel.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 16f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Turbofuel
recipe_registry.entry("Turbofuel").or_insert_with(Vec::new).push(turbofuel_recipe);
let alternate_turbo_electric_motor_recipe = Recipe { name: "Alternate: Turbo Electric Motor", machine: Machine::Manufacturer, input: vec![ItemAmount {item: motor.clone(), amount: 7 }, ItemAmount {item: radio_control_unit.clone(), amount: 9 }, ItemAmount {item: electromagnetic_control_rod.clone(), amount: 5 }, ItemAmount {item: rotor.clone(), amount: 7 }, ], output: ItemAmount {item: turbo_motor.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 64f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_TurboMotor_1
recipe_registry.entry("Turbo Motor").or_insert_with(Vec::new).push(alternate_turbo_electric_motor_recipe);
let alternate_infused_uranium_cell_recipe = Recipe { name: "Alternate: Infused Uranium Cell", machine: Machine::Manufacturer, input: vec![ItemAmount {item: uranium.clone(), amount: 5 }, ItemAmount {item: silica.clone(), amount: 3 }, ItemAmount {item: sulfur.clone(), amount: 5 }, ItemAmount {item: quickwire.clone(), amount: 15 }, ], output: ItemAmount {item: encased_uranium_cell.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 12f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_UraniumCell_1
recipe_registry.entry("Encased Uranium Cell").or_insert_with(Vec::new).push(alternate_infused_uranium_cell_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Constructor, input: vec![ItemAmount {item: iron_ingot.clone(), amount: 5 }, ], output: ItemAmount {item: wire.clone(), amount: 9 }, byproduct: None, manufacturing_duration: 24f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Wire_1
recipe_registry.entry("Wire").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);
let alternate_caterium_wire_recipe = Recipe { name: "Alternate: Caterium Wire", machine: Machine::Constructor, input: vec![ItemAmount {item: caterium_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: wire.clone(), amount: 8 }, byproduct: None, manufacturing_duration: 4f32, alternate: true }; // FactoryGame/Content/FactoryGame/Recipes/AlternateRecipes/Parts/Recipe_Alternate_Wire_2
recipe_registry.entry("Wire").or_insert_with(Vec::new).push(alternate_caterium_wire_recipe);
let ailimiter_recipe = Recipe { name: "AI Limiter", machine: Machine::Assembler, input: vec![ItemAmount {item: copper_sheet.clone(), amount: 5 }, ItemAmount {item: quickwire.clone(), amount: 20 }, ], output: ItemAmount {item: ailimiter.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_AILimiter
recipe_registry.entry("AI Limiter").or_insert_with(Vec::new).push(ailimiter_recipe);
let circuit_board_recipe = Recipe { name: "Circuit Board", machine: Machine::Assembler, input: vec![ItemAmount {item: copper_sheet.clone(), amount: 2 }, ItemAmount {item: plastic.clone(), amount: 4 }, ], output: ItemAmount {item: circuit_board.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 8f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_CircuitBoard
recipe_registry.entry("Circuit Board").or_insert_with(Vec::new).push(circuit_board_recipe);
let electromagnetic_control_rod_recipe = Recipe { name: "Electromagnetic Control Rod", machine: Machine::Assembler, input: vec![ItemAmount {item: stator.clone(), amount: 3 }, ItemAmount {item: ailimiter.clone(), amount: 2 }, ], output: ItemAmount {item: electromagnetic_control_rod.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 30f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_ElectromagneticControlRod
recipe_registry.entry("Electromagnetic Control Rod").or_insert_with(Vec::new).push(electromagnetic_control_rod_recipe);
let encased_industrial_beam_recipe = Recipe { name: "Encased Industrial Beam", machine: Machine::Assembler, input: vec![ItemAmount {item: steel_beam.clone(), amount: 4 }, ItemAmount {item: concrete.clone(), amount: 5 }, ], output: ItemAmount {item: encased_industrial_beam.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 10f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_EncasedIndustrialBeam
recipe_registry.entry("Encased Industrial Beam").or_insert_with(Vec::new).push(encased_industrial_beam_recipe);
let heat_sink_recipe = Recipe { name: "Heat Sink", machine: Machine::Assembler, input: vec![ItemAmount {item: alclad_aluminum_sheet.clone(), amount: 5 }, ItemAmount {item: copper_sheet.clone(), amount: 3 }, ], output: ItemAmount {item: heat_sink.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 8f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_HeatSink
recipe_registry.entry("Heat Sink").or_insert_with(Vec::new).push(heat_sink_recipe);
let reinforced_iron_plate_recipe = Recipe { name: "Reinforced Iron Plate", machine: Machine::Assembler, input: vec![ItemAmount {item: iron_plate.clone(), amount: 6 }, ItemAmount {item: screw.clone(), amount: 12 }, ], output: ItemAmount {item: reinforced_iron_plate.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_IronPlateReinforced
recipe_registry.entry("Reinforced Iron Plate").or_insert_with(Vec::new).push(reinforced_iron_plate_recipe);
let modular_frame_recipe = Recipe { name: "Modular Frame", machine: Machine::Assembler, input: vec![ItemAmount {item: reinforced_iron_plate.clone(), amount: 3 }, ItemAmount {item: iron_rod.clone(), amount: 12 }, ], output: ItemAmount {item: modular_frame.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 60f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_ModularFrame
recipe_registry.entry("Modular Frame").or_insert_with(Vec::new).push(modular_frame_recipe);
let motor_recipe = Recipe { name: "Motor", machine: Machine::Assembler, input: vec![ItemAmount {item: rotor.clone(), amount: 2 }, ItemAmount {item: stator.clone(), amount: 2 }, ], output: ItemAmount {item: motor.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_Motor
recipe_registry.entry("Motor").or_insert_with(Vec::new).push(motor_recipe);
let encased_plutonium_cell_recipe = Recipe { name: "Encased Plutonium Cell", machine: Machine::Assembler, input: vec![ItemAmount {item: plutonium_pellet.clone(), amount: 2 }, ItemAmount {item: concrete.clone(), amount: 4 }, ], output: ItemAmount {item: encased_plutonium_cell.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_PlutoniumCell
recipe_registry.entry("Encased Plutonium Cell").or_insert_with(Vec::new).push(encased_plutonium_cell_recipe);
let pressure_conversion_cube_recipe = Recipe { name: "Pressure Conversion Cube", machine: Machine::Assembler, input: vec![ItemAmount {item: fused_modular_frame.clone(), amount: 1 }, ItemAmount {item: radio_control_unit.clone(), amount: 2 }, ], output: ItemAmount {item: pressure_conversion_cube.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 60f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_PressureConversionCube
recipe_registry.entry("Pressure Conversion Cube").or_insert_with(Vec::new).push(pressure_conversion_cube_recipe);
let rotor_recipe = Recipe { name: "Rotor", machine: Machine::Assembler, input: vec![ItemAmount {item: iron_rod.clone(), amount: 5 }, ItemAmount {item: screw.clone(), amount: 25 }, ], output: ItemAmount {item: rotor.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 15f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_Rotor
recipe_registry.entry("Rotor").or_insert_with(Vec::new).push(rotor_recipe);
let stator_recipe = Recipe { name: "Stator", machine: Machine::Assembler, input: vec![ItemAmount {item: steel_pipe.clone(), amount: 3 }, ItemAmount {item: wire.clone(), amount: 8 }, ], output: ItemAmount {item: stator.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_Stator
recipe_registry.entry("Stator").or_insert_with(Vec::new).push(stator_recipe);
let encased_uranium_cell_recipe = Recipe { name: "Encased Uranium Cell", machine: Machine::Blender, input: vec![ItemAmount {item: uranium.clone(), amount: 10 }, ItemAmount {item: concrete.clone(), amount: 3 }, ItemAmount {item: sulfuric_acid.clone(), amount: 8 }, ], output: ItemAmount {item: encased_uranium_cell.clone(), amount: 5 }, byproduct: Some(ItemAmount {item: sulfuric_acid.clone(), amount: 2 }), manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Assembler/Recipe_UraniumCell
recipe_registry.entry("Encased Uranium Cell").or_insert_with(Vec::new).push(encased_uranium_cell_recipe);
let cooling_system_recipe = Recipe { name: "Cooling System", machine: Machine::Blender, input: vec![ItemAmount {item: heat_sink.clone(), amount: 2 }, ItemAmount {item: rubber.clone(), amount: 2 }, ItemAmount {item: water.clone(), amount: 5 }, ItemAmount {item: nitrogen_gas.clone(), amount: 25 }, ], output: ItemAmount {item: cooling_system.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 10f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Blender/Recipe_CoolingSystem
recipe_registry.entry("Cooling System").or_insert_with(Vec::new).push(cooling_system_recipe);
let fused_modular_frame_recipe = Recipe { name: "Fused Modular Frame", machine: Machine::Blender, input: vec![ItemAmount {item: heavy_modular_frame.clone(), amount: 1 }, ItemAmount {item: aluminum_casing.clone(), amount: 50 }, ItemAmount {item: nitrogen_gas.clone(), amount: 25 }, ], output: ItemAmount {item: fused_modular_frame.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 40f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Blender/Recipe_FusedModularFrame
recipe_registry.entry("Fused Modular Frame").or_insert_with(Vec::new).push(fused_modular_frame_recipe);
let nitric_acid_recipe = Recipe { name: "Nitric Acid", machine: Machine::Blender, input: vec![ItemAmount {item: nitrogen_gas.clone(), amount: 12 }, ItemAmount {item: water.clone(), amount: 3 }, ItemAmount {item: iron_plate.clone(), amount: 1 }, ], output: ItemAmount {item: nitric_acid.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Blender/Recipe_NitricAcid
recipe_registry.entry("Nitric Acid").or_insert_with(Vec::new).push(nitric_acid_recipe);
let non_fissile_uranium_recipe = Recipe { name: "Non-fissile Uranium", machine: Machine::Blender, input: vec![ItemAmount {item: uranium_waste.clone(), amount: 15 }, ItemAmount {item: silica.clone(), amount: 10 }, ItemAmount {item: nitric_acid.clone(), amount: 6 }, ItemAmount {item: sulfuric_acid.clone(), amount: 6 }, ], output: ItemAmount {item: non_fissile_uranium.clone(), amount: 20 }, byproduct: Some(ItemAmount {item: water.clone(), amount: 6 }), manufacturing_duration: 24f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Blender/Recipe_NonFissileUranium
recipe_registry.entry("Non-fissile Uranium").or_insert_with(Vec::new).push(non_fissile_uranium_recipe);
let alien_dnacapsule_recipe = Recipe { name: "Alien DNA Capsule", machine: Machine::Constructor, input: vec![ItemAmount {item: alien_protein.clone(), amount: 1 }, ], output: ItemAmount {item: alien_dnacapsule.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_AlienDNACapsule
recipe_registry.entry("Alien DNA Capsule").or_insert_with(Vec::new).push(alien_dnacapsule_recipe);
let aluminum_casing_recipe = Recipe { name: "Aluminum Casing", machine: Machine::Constructor, input: vec![ItemAmount {item: aluminum_ingot.clone(), amount: 3 }, ], output: ItemAmount {item: aluminum_casing.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_AluminumCasing
recipe_registry.entry("Aluminum Casing").or_insert_with(Vec::new).push(aluminum_casing_recipe);
let alclad_aluminum_sheet_recipe = Recipe { name: "Alclad Aluminum Sheet", machine: Machine::Assembler, input: vec![ItemAmount {item: aluminum_ingot.clone(), amount: 3 }, ItemAmount {item: copper_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: alclad_aluminum_sheet.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_AluminumSheet
recipe_registry.entry("Alclad Aluminum Sheet").or_insert_with(Vec::new).push(alclad_aluminum_sheet_recipe);
let solid_biofuel_recipe = Recipe { name: "Solid Biofuel", machine: Machine::Constructor, input: vec![ItemAmount {item: biomass.clone(), amount: 8 }, ], output: ItemAmount {item: solid_biofuel.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Biofuel
recipe_registry.entry("Solid Biofuel").or_insert_with(Vec::new).push(solid_biofuel_recipe);
let biomass_recipe = Recipe { name: "Biomass", machine: Machine::Constructor, input: vec![ItemAmount {item: alien_protein.clone(), amount: 1 }, ], output: ItemAmount {item: biomass.clone(), amount: 100 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Biomass_AlienProtein
recipe_registry.entry("Biomass").or_insert_with(Vec::new).push(biomass_recipe);
let biomass_recipe = Recipe { name: "Biomass", machine: Machine::Constructor, input: vec![ItemAmount {item: leaves.clone(), amount: 10 }, ], output: ItemAmount {item: biomass.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 5f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Biomass_Leaves
recipe_registry.entry("Biomass").or_insert_with(Vec::new).push(biomass_recipe);
let biomass_recipe = Recipe { name: "Biomass", machine: Machine::Constructor, input: vec![ItemAmount {item: mycelia.clone(), amount: 1 }, ], output: ItemAmount {item: biomass.clone(), amount: 10 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Biomass_Mycelia
recipe_registry.entry("Biomass").or_insert_with(Vec::new).push(biomass_recipe);
let biomass_recipe = Recipe { name: "Biomass", machine: Machine::Constructor, input: vec![ItemAmount {item: wood.clone(), amount: 4 }, ], output: ItemAmount {item: biomass.clone(), amount: 20 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Biomass_Wood
recipe_registry.entry("Biomass").or_insert_with(Vec::new).push(biomass_recipe);
let cable_recipe = Recipe { name: "Cable", machine: Machine::Constructor, input: vec![ItemAmount {item: wire.clone(), amount: 2 }, ], output: ItemAmount {item: cable.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Cable
recipe_registry.entry("Cable").or_insert_with(Vec::new).push(cable_recipe);
let concrete_recipe = Recipe { name: "Concrete", machine: Machine::Constructor, input: vec![ItemAmount {item: limestone.clone(), amount: 3 }, ], output: ItemAmount {item: concrete.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Concrete
recipe_registry.entry("Concrete").or_insert_with(Vec::new).push(concrete_recipe);
let copper_powder_recipe = Recipe { name: "Copper Powder", machine: Machine::Constructor, input: vec![ItemAmount {item: copper_ingot.clone(), amount: 30 }, ], output: ItemAmount {item: copper_powder.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_CopperDust
recipe_registry.entry("Copper Powder").or_insert_with(Vec::new).push(copper_powder_recipe);
let copper_sheet_recipe = Recipe { name: "Copper Sheet", machine: Machine::Constructor, input: vec![ItemAmount {item: copper_ingot.clone(), amount: 2 }, ], output: ItemAmount {item: copper_sheet.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_CopperSheet
recipe_registry.entry("Copper Sheet").or_insert_with(Vec::new).push(copper_sheet_recipe);
let fabric_recipe = Recipe { name: "Fabric", machine: Machine::Assembler, input: vec![ItemAmount {item: mycelia.clone(), amount: 1 }, ItemAmount {item: biomass.clone(), amount: 5 }, ], output: ItemAmount {item: fabric.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Fabric
recipe_registry.entry("Fabric").or_insert_with(Vec::new).push(fabric_recipe);
let empty_canister_recipe = Recipe { name: "Empty Canister", machine: Machine::Constructor, input: vec![ItemAmount {item: plastic.clone(), amount: 2 }, ], output: ItemAmount {item: empty_canister.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_FluidCanister
recipe_registry.entry("Empty Canister").or_insert_with(Vec::new).push(empty_canister_recipe);
let empty_fluid_tank_recipe = Recipe { name: "Empty Fluid Tank", machine: Machine::Constructor, input: vec![ItemAmount {item: aluminum_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: empty_fluid_tank.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_GasTank
recipe_registry.entry("Empty Fluid Tank").or_insert_with(Vec::new).push(empty_fluid_tank_recipe);
let iron_plate_recipe = Recipe { name: "Iron Plate", machine: Machine::Constructor, input: vec![ItemAmount {item: iron_ingot.clone(), amount: 3 }, ], output: ItemAmount {item: iron_plate.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_IronPlate
recipe_registry.entry("Iron Plate").or_insert_with(Vec::new).push(iron_plate_recipe);
let iron_rod_recipe = Recipe { name: "Iron Rod", machine: Machine::Constructor, input: vec![ItemAmount {item: iron_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: iron_rod.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_IronRod
recipe_registry.entry("Iron Rod").or_insert_with(Vec::new).push(iron_rod_recipe);
let uranium_fuel_rod_recipe = Recipe { name: "Uranium Fuel Rod", machine: Machine::Manufacturer, input: vec![ItemAmount {item: encased_uranium_cell.clone(), amount: 50 }, ItemAmount {item: encased_industrial_beam.clone(), amount: 3 }, ItemAmount {item: electromagnetic_control_rod.clone(), amount: 5 }, ], output: ItemAmount {item: uranium_fuel_rod.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 150f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_NuclearFuelRod
recipe_registry.entry("Uranium Fuel Rod").or_insert_with(Vec::new).push(uranium_fuel_rod_recipe);
let power_shard_recipe = Recipe { name: "Power Shard", machine: Machine::Constructor, input: vec![ItemAmount {item: blue_power_slug.clone(), amount: 1 }, ], output: ItemAmount {item: power_shard.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 8f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_PowerCrystalShard_1
recipe_registry.entry("Power Shard").or_insert_with(Vec::new).push(power_shard_recipe);
let power_shard_recipe = Recipe { name: "Power Shard", machine: Machine::Constructor, input: vec![ItemAmount {item: yellow_power_slug.clone(), amount: 1 }, ], output: ItemAmount {item: power_shard.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_PowerCrystalShard_2
recipe_registry.entry("Power Shard").or_insert_with(Vec::new).push(power_shard_recipe);
let power_shard_recipe = Recipe { name: "Power Shard", machine: Machine::Constructor, input: vec![ItemAmount {item: purple_power_slug.clone(), amount: 1 }, ], output: ItemAmount {item: power_shard.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 24f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_PowerCrystalShard_3
recipe_registry.entry("Power Shard").or_insert_with(Vec::new).push(power_shard_recipe);
let alien_protein_recipe = Recipe { name: "Alien Protein", machine: Machine::Constructor, input: vec![ItemAmount {item: hatcher_remains.clone(), amount: 1 }, ], output: ItemAmount {item: alien_protein.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Protein_Crab
recipe_registry.entry("Alien Protein").or_insert_with(Vec::new).push(alien_protein_recipe);
let alien_protein_recipe = Recipe { name: "Alien Protein", machine: Machine::Constructor, input: vec![ItemAmount {item: hog_remains.clone(), amount: 1 }, ], output: ItemAmount {item: alien_protein.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Protein_Hog
recipe_registry.entry("Alien Protein").or_insert_with(Vec::new).push(alien_protein_recipe);
let alien_protein_recipe = Recipe { name: "Alien Protein", machine: Machine::Constructor, input: vec![ItemAmount {item: plasma_spitter_remains.clone(), amount: 1 }, ], output: ItemAmount {item: alien_protein.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Protein_Spitter
recipe_registry.entry("Alien Protein").or_insert_with(Vec::new).push(alien_protein_recipe);
let alien_protein_recipe = Recipe { name: "Alien Protein", machine: Machine::Constructor, input: vec![ItemAmount {item: stinger_remains.clone(), amount: 1 }, ], output: ItemAmount {item: alien_protein.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Protein_Stinger
recipe_registry.entry("Alien Protein").or_insert_with(Vec::new).push(alien_protein_recipe);
let quartz_crystal_recipe = Recipe { name: "Quartz Crystal", machine: Machine::Constructor, input: vec![ItemAmount {item: raw_quartz.clone(), amount: 5 }, ], output: ItemAmount {item: quartz_crystal.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 8f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_QuartzCrystal
recipe_registry.entry("Quartz Crystal").or_insert_with(Vec::new).push(quartz_crystal_recipe);
let quickwire_recipe = Recipe { name: "Quickwire", machine: Machine::Constructor, input: vec![ItemAmount {item: caterium_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: quickwire.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 5f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Quickwire
recipe_registry.entry("Quickwire").or_insert_with(Vec::new).push(quickwire_recipe);
let screw_recipe = Recipe { name: "Screw", machine: Machine::Constructor, input: vec![ItemAmount {item: iron_rod.clone(), amount: 1 }, ], output: ItemAmount {item: screw.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Screw
recipe_registry.entry("Screw").or_insert_with(Vec::new).push(screw_recipe);
let silica_recipe = Recipe { name: "Silica", machine: Machine::Constructor, input: vec![ItemAmount {item: raw_quartz.clone(), amount: 3 }, ], output: ItemAmount {item: silica.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 8f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Silica
recipe_registry.entry("Silica").or_insert_with(Vec::new).push(silica_recipe);
let steel_beam_recipe = Recipe { name: "Steel Beam", machine: Machine::Constructor, input: vec![ItemAmount {item: steel_ingot.clone(), amount: 4 }, ], output: ItemAmount {item: steel_beam.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_SteelBeam
recipe_registry.entry("Steel Beam").or_insert_with(Vec::new).push(steel_beam_recipe);
let steel_pipe_recipe = Recipe { name: "Steel Pipe", machine: Machine::Constructor, input: vec![ItemAmount {item: steel_ingot.clone(), amount: 3 }, ], output: ItemAmount {item: steel_pipe.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_SteelPipe
recipe_registry.entry("Steel Pipe").or_insert_with(Vec::new).push(steel_pipe_recipe);
let wire_recipe = Recipe { name: "Wire", machine: Machine::Constructor, input: vec![ItemAmount {item: copper_ingot.clone(), amount: 1 }, ], output: ItemAmount {item: wire.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Constructor/Recipe_Wire
recipe_registry.entry("Wire").or_insert_with(Vec::new).push(wire_recipe);
let beacon_recipe = Recipe { name: "Beacon", machine: Machine::Manufacturer, input: vec![ItemAmount {item: iron_plate.clone(), amount: 3 }, ItemAmount {item: iron_rod.clone(), amount: 1 }, ItemAmount {item: wire.clone(), amount: 15 }, ItemAmount {item: cable.clone(), amount: 2 }, ], output: ItemAmount {item: beacon.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 8f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_Beacon
recipe_registry.entry("Beacon").or_insert_with(Vec::new).push(beacon_recipe);
let rifle_ammo_recipe = Recipe { name: "Rifle Ammo", machine: Machine::Assembler, input: vec![ItemAmount {item: copper_sheet.clone(), amount: 3 }, ItemAmount {item: smokeless_powder.clone(), amount: 2 }, ], output: ItemAmount {item: rifle_ammo.clone(), amount: 15 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_Cartridge
recipe_registry.entry("Rifle Ammo").or_insert_with(Vec::new).push(rifle_ammo_recipe);
let color_cartridge_recipe = Recipe { name: "Color Cartridge", machine: Machine::Constructor, input: vec![ItemAmount {item: flower_petals.clone(), amount: 5 }, ], output: ItemAmount {item: color_cartridge.clone(), amount: 10 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_ColorCartridge
recipe_registry.entry("Color Cartridge").or_insert_with(Vec::new).push(color_cartridge_recipe);
let gas_filter_recipe = Recipe { name: "Gas Filter", machine: Machine::Manufacturer, input: vec![ItemAmount {item: coal.clone(), amount: 5 }, ItemAmount {item: rubber.clone(), amount: 2 }, ItemAmount {item: fabric.clone(), amount: 2 }, ], output: ItemAmount {item: gas_filter.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 8f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_FilterGasMask
recipe_registry.entry("Gas Filter").or_insert_with(Vec::new).push(gas_filter_recipe);
let iodine_infused_filter_recipe = Recipe { name: "Iodine Infused Filter", machine: Machine::Manufacturer, input: vec![ItemAmount {item: gas_filter.clone(), amount: 1 }, ItemAmount {item: quickwire.clone(), amount: 8 }, ItemAmount {item: aluminum_casing.clone(), amount: 1 }, ], output: ItemAmount {item: iodine_infused_filter.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 16f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_FilterHazmat
recipe_registry.entry("Iodine Infused Filter").or_insert_with(Vec::new).push(iodine_infused_filter_recipe);
let black_powder_recipe = Recipe { name: "Black Powder", machine: Machine::Assembler, input: vec![ItemAmount {item: coal.clone(), amount: 1 }, ItemAmount {item: sulfur.clone(), amount: 1 }, ], output: ItemAmount {item: black_powder.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_Gunpowder
recipe_registry.entry("Black Powder").or_insert_with(Vec::new).push(black_powder_recipe);
let smokeless_powder_recipe = Recipe { name: "Smokeless Powder", machine: Machine::Refinery, input: vec![ItemAmount {item: black_powder.clone(), amount: 2 }, ItemAmount {item: heavy_oil_residue.clone(), amount: 1 }, ], output: ItemAmount {item: smokeless_powder.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_GunpowderMK2
recipe_registry.entry("Smokeless Powder").or_insert_with(Vec::new).push(smokeless_powder_recipe);
let nobelisk_recipe = Recipe { name: "Nobelisk", machine: Machine::Assembler, input: vec![ItemAmount {item: black_powder.clone(), amount: 2 }, ItemAmount {item: steel_pipe.clone(), amount: 2 }, ], output: ItemAmount {item: nobelisk.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_Nobelisk
recipe_registry.entry("Nobelisk").or_insert_with(Vec::new).push(nobelisk_recipe);
let iron_rebar_recipe = Recipe { name: "Iron Rebar", machine: Machine::Constructor, input: vec![ItemAmount {item: iron_rod.clone(), amount: 1 }, ], output: ItemAmount {item: iron_rebar.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Equipment/Recipe_SpikedRebar
recipe_registry.entry("Iron Rebar").or_insert_with(Vec::new).push(iron_rebar_recipe);
let snowball_recipe = Recipe { name: "Snowball", machine: Machine::Constructor, input: vec![ItemAmount {item: actual_snow.clone(), amount: 3 }, ], output: ItemAmount {item: snowball.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 12f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Holiday/Recipe_Snowball
recipe_registry.entry("Snowball").or_insert_with(Vec::new).push(snowball_recipe);
let battery_recipe = Recipe { name: "Battery", machine: Machine::Blender, input: vec![ItemAmount {item: sulfuric_acid.clone(), amount: 2 }, ItemAmount {item: alumina_solution.clone(), amount: 2 }, ItemAmount {item: aluminum_casing.clone(), amount: 1 }, ], output: ItemAmount {item: battery.clone(), amount: 1 }, byproduct: Some(ItemAmount {item: water.clone(), amount: 1 }), manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_Battery
recipe_registry.entry("Battery").or_insert_with(Vec::new).push(battery_recipe);
let computer_recipe = Recipe { name: "Computer", machine: Machine::Manufacturer, input: vec![ItemAmount {item: circuit_board.clone(), amount: 10 }, ItemAmount {item: cable.clone(), amount: 9 }, ItemAmount {item: plastic.clone(), amount: 18 }, ItemAmount {item: screw.clone(), amount: 52 }, ], output: ItemAmount {item: computer.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 24f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_Computer
recipe_registry.entry("Computer").or_insert_with(Vec::new).push(computer_recipe);
let supercomputer_recipe = Recipe { name: "Supercomputer", machine: Machine::Manufacturer, input: vec![ItemAmount {item: computer.clone(), amount: 2 }, ItemAmount {item: ailimiter.clone(), amount: 2 }, ItemAmount {item: high_speed_connector.clone(), amount: 3 }, ItemAmount {item: plastic.clone(), amount: 28 }, ], output: ItemAmount {item: supercomputer.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 32f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_ComputerSuper
recipe_registry.entry("Supercomputer").or_insert_with(Vec::new).push(supercomputer_recipe);
let crystal_oscillator_recipe = Recipe { name: "Crystal Oscillator", machine: Machine::Manufacturer, input: vec![ItemAmount {item: quartz_crystal.clone(), amount: 36 }, ItemAmount {item: cable.clone(), amount: 28 }, ItemAmount {item: reinforced_iron_plate.clone(), amount: 5 }, ], output: ItemAmount {item: crystal_oscillator.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 120f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_CrystalOscillator
recipe_registry.entry("Crystal Oscillator").or_insert_with(Vec::new).push(crystal_oscillator_recipe);
let high_speed_connector_recipe = Recipe { name: "High-Speed Connector", machine: Machine::Manufacturer, input: vec![ItemAmount {item: quickwire.clone(), amount: 56 }, ItemAmount {item: cable.clone(), amount: 10 }, ItemAmount {item: circuit_board.clone(), amount: 1 }, ], output: ItemAmount {item: high_speed_connector.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 16f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_HighSpeedConnector
recipe_registry.entry("High-Speed Connector").or_insert_with(Vec::new).push(high_speed_connector_recipe);
let heavy_modular_frame_recipe = Recipe { name: "Heavy Modular Frame", machine: Machine::Manufacturer, input: vec![ItemAmount {item: modular_frame.clone(), amount: 5 }, ItemAmount {item: steel_pipe.clone(), amount: 15 }, ItemAmount {item: encased_industrial_beam.clone(), amount: 5 }, ItemAmount {item: screw.clone(), amount: 100 }, ], output: ItemAmount {item: heavy_modular_frame.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 30f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_ModularFrameHeavy
recipe_registry.entry("Heavy Modular Frame").or_insert_with(Vec::new).push(heavy_modular_frame_recipe);
let turbo_motor_recipe = Recipe { name: "Turbo Motor", machine: Machine::Manufacturer, input: vec![ItemAmount {item: cooling_system.clone(), amount: 4 }, ItemAmount {item: radio_control_unit.clone(), amount: 2 }, ItemAmount {item: motor.clone(), amount: 4 }, ItemAmount {item: rubber.clone(), amount: 24 }, ], output: ItemAmount {item: turbo_motor.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 32f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_MotorTurbo
recipe_registry.entry("Turbo Motor").or_insert_with(Vec::new).push(turbo_motor_recipe);
let plutonium_fuel_rod_recipe = Recipe { name: "Plutonium Fuel Rod", machine: Machine::Manufacturer, input: vec![ItemAmount {item: encased_plutonium_cell.clone(), amount: 30 }, ItemAmount {item: steel_beam.clone(), amount: 18 }, ItemAmount {item: electromagnetic_control_rod.clone(), amount: 6 }, ItemAmount {item: heat_sink.clone(), amount: 10 }, ], output: ItemAmount {item: plutonium_fuel_rod.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 240f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_PlutoniumFuelRod
recipe_registry.entry("Plutonium Fuel Rod").or_insert_with(Vec::new).push(plutonium_fuel_rod_recipe);
let radio_control_unit_recipe = Recipe { name: "Radio Control Unit", machine: Machine::Manufacturer, input: vec![ItemAmount {item: aluminum_casing.clone(), amount: 32 }, ItemAmount {item: crystal_oscillator.clone(), amount: 1 }, ItemAmount {item: computer.clone(), amount: 1 }, ], output: ItemAmount {item: radio_control_unit.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 48f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Manufacturer/Recipe_RadioControlUnit
recipe_registry.entry("Radio Control Unit").or_insert_with(Vec::new).push(radio_control_unit_recipe);
let alumina_solution_recipe = Recipe { name: "Alumina Solution", machine: Machine::Refinery, input: vec![ItemAmount {item: bauxite.clone(), amount: 12 }, ItemAmount {item: water.clone(), amount: 18 }, ], output: ItemAmount {item: alumina_solution.clone(), amount: 12 }, byproduct: Some(ItemAmount {item: silica.clone(), amount: 5 }), manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_AluminaSolution
recipe_registry.entry("Alumina Solution").or_insert_with(Vec::new).push(alumina_solution_recipe);
let aluminum_scrap_recipe = Recipe { name: "Aluminum Scrap", machine: Machine::Refinery, input: vec![ItemAmount {item: alumina_solution.clone(), amount: 4 }, ItemAmount {item: coal.clone(), amount: 2 }, ], output: ItemAmount {item: aluminum_scrap.clone(), amount: 6 }, byproduct: Some(ItemAmount {item: water.clone(), amount: 2 }), manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_AluminumScrap
recipe_registry.entry("Aluminum Scrap").or_insert_with(Vec::new).push(aluminum_scrap_recipe);
let packaged_fuel_recipe = Recipe { name: "Packaged Fuel", machine: Machine::Packager, input: vec![ItemAmount {item: fuel.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_fuel.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_Fuel
recipe_registry.entry("Packaged Fuel").or_insert_with(Vec::new).push(packaged_fuel_recipe);
let liquid_biofuel_recipe = Recipe { name: "Liquid Biofuel", machine: Machine::Refinery, input: vec![ItemAmount {item: solid_biofuel.clone(), amount: 6 }, ItemAmount {item: water.clone(), amount: 3 }, ], output: ItemAmount {item: liquid_biofuel.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_LiquidBiofuel
recipe_registry.entry("Liquid Biofuel").or_insert_with(Vec::new).push(liquid_biofuel_recipe);
let fuel_recipe = Recipe { name: "Fuel", machine: Machine::Refinery, input: vec![ItemAmount {item: crude_oil.clone(), amount: 6 }, ], output: ItemAmount {item: fuel.clone(), amount: 4 }, byproduct: Some(ItemAmount {item: polymer_resin.clone(), amount: 3 }), manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_LiquidFuel
recipe_registry.entry("Fuel").or_insert_with(Vec::new).push(fuel_recipe);
let packaged_liquid_biofuel_recipe = Recipe { name: "Packaged Liquid Biofuel", machine: Machine::Packager, input: vec![ItemAmount {item: liquid_biofuel.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_liquid_biofuel.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_PackagedBiofuel
recipe_registry.entry("Packaged Liquid Biofuel").or_insert_with(Vec::new).push(packaged_liquid_biofuel_recipe);
let packaged_oil_recipe = Recipe { name: "Packaged Oil", machine: Machine::Packager, input: vec![ItemAmount {item: crude_oil.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_oil.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_PackagedCrudeOil
recipe_registry.entry("Packaged Oil").or_insert_with(Vec::new).push(packaged_oil_recipe);
let packaged_heavy_oil_residue_recipe = Recipe { name: "Packaged Heavy Oil Residue", machine: Machine::Packager, input: vec![ItemAmount {item: heavy_oil_residue.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_heavy_oil_residue.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_PackagedOilResidue
recipe_registry.entry("Packaged Heavy Oil Residue").or_insert_with(Vec::new).push(packaged_heavy_oil_residue_recipe);
let packaged_turbofuel_recipe = Recipe { name: "Packaged Turbofuel", machine: Machine::Packager, input: vec![ItemAmount {item: turbofuel.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_turbofuel.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_PackagedTurboFuel
recipe_registry.entry("Packaged Turbofuel").or_insert_with(Vec::new).push(packaged_turbofuel_recipe);
let packaged_water_recipe = Recipe { name: "Packaged Water", machine: Machine::Packager, input: vec![ItemAmount {item: water.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_water.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_PackagedWater
recipe_registry.entry("Packaged Water").or_insert_with(Vec::new).push(packaged_water_recipe);
let petroleum_coke_recipe = Recipe { name: "Petroleum Coke", machine: Machine::Refinery, input: vec![ItemAmount {item: heavy_oil_residue.clone(), amount: 4 }, ], output: ItemAmount {item: petroleum_coke.clone(), amount: 12 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_PetroleumCoke
recipe_registry.entry("Petroleum Coke").or_insert_with(Vec::new).push(petroleum_coke_recipe);
let plastic_recipe = Recipe { name: "Plastic", machine: Machine::Refinery, input: vec![ItemAmount {item: crude_oil.clone(), amount: 3 }, ], output: ItemAmount {item: plastic.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: heavy_oil_residue.clone(), amount: 1 }), manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_Plastic
recipe_registry.entry("Plastic").or_insert_with(Vec::new).push(plastic_recipe);
let fuel_recipe = Recipe { name: "Fuel", machine: Machine::Refinery, input: vec![ItemAmount {item: heavy_oil_residue.clone(), amount: 6 }, ], output: ItemAmount {item: fuel.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_ResidualFuel
recipe_registry.entry("Fuel").or_insert_with(Vec::new).push(fuel_recipe);
let plastic_recipe = Recipe { name: "Plastic", machine: Machine::Refinery, input: vec![ItemAmount {item: polymer_resin.clone(), amount: 6 }, ItemAmount {item: water.clone(), amount: 2 }, ], output: ItemAmount {item: plastic.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_ResidualPlastic
recipe_registry.entry("Plastic").or_insert_with(Vec::new).push(plastic_recipe);
let rubber_recipe = Recipe { name: "Rubber", machine: Machine::Refinery, input: vec![ItemAmount {item: polymer_resin.clone(), amount: 4 }, ItemAmount {item: water.clone(), amount: 4 }, ], output: ItemAmount {item: rubber.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_ResidualRubber
recipe_registry.entry("Rubber").or_insert_with(Vec::new).push(rubber_recipe);
let rubber_recipe = Recipe { name: "Rubber", machine: Machine::Refinery, input: vec![ItemAmount {item: crude_oil.clone(), amount: 3 }, ], output: ItemAmount {item: rubber.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: heavy_oil_residue.clone(), amount: 2 }), manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_Rubber
recipe_registry.entry("Rubber").or_insert_with(Vec::new).push(rubber_recipe);
let sulfuric_acid_recipe = Recipe { name: "Sulfuric Acid", machine: Machine::Refinery, input: vec![ItemAmount {item: sulfur.clone(), amount: 5 }, ItemAmount {item: water.clone(), amount: 5 }, ], output: ItemAmount {item: sulfuric_acid.clone(), amount: 5 }, byproduct: None, manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_SulfuricAcid
recipe_registry.entry("Sulfuric Acid").or_insert_with(Vec::new).push(sulfuric_acid_recipe);
let liquid_biofuel_recipe = Recipe { name: "Liquid Biofuel", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_liquid_biofuel.clone(), amount: 2 }, ], output: ItemAmount {item: liquid_biofuel.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 2 }), manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_UnpackageBioFuel
recipe_registry.entry("Liquid Biofuel").or_insert_with(Vec::new).push(liquid_biofuel_recipe);
let fuel_recipe = Recipe { name: "Fuel", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_fuel.clone(), amount: 2 }, ], output: ItemAmount {item: fuel.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 2 }), manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_UnpackageFuel
recipe_registry.entry("Fuel").or_insert_with(Vec::new).push(fuel_recipe);
let crude_oil_recipe = Recipe { name: "Crude Oil", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_oil.clone(), amount: 2 }, ], output: ItemAmount {item: crude_oil.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 2 }), manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_UnpackageOil
recipe_registry.entry("Crude Oil").or_insert_with(Vec::new).push(crude_oil_recipe);
let heavy_oil_residue_recipe = Recipe { name: "Heavy Oil Residue", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_heavy_oil_residue.clone(), amount: 2 }, ], output: ItemAmount {item: heavy_oil_residue.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 2 }), manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_UnpackageOilResidue
recipe_registry.entry("Heavy Oil Residue").or_insert_with(Vec::new).push(heavy_oil_residue_recipe);
let turbofuel_recipe = Recipe { name: "Turbofuel", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_turbofuel.clone(), amount: 2 }, ], output: ItemAmount {item: turbofuel.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 2 }), manufacturing_duration: 6f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_UnpackageTurboFuel
recipe_registry.entry("Turbofuel").or_insert_with(Vec::new).push(turbofuel_recipe);
let water_recipe = Recipe { name: "Water", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_water.clone(), amount: 2 }, ], output: ItemAmount {item: water.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 2 }), manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/OilRefinery/Recipe_UnpackageWater
recipe_registry.entry("Water").or_insert_with(Vec::new).push(water_recipe);
let packaged_alumina_solution_recipe = Recipe { name: "Packaged Alumina Solution", machine: Machine::Packager, input: vec![ItemAmount {item: alumina_solution.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_alumina_solution.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_PackagedAlumina
recipe_registry.entry("Packaged Alumina Solution").or_insert_with(Vec::new).push(packaged_alumina_solution_recipe);
let packaged_nitric_acid_recipe = Recipe { name: "Packaged Nitric Acid", machine: Machine::Packager, input: vec![ItemAmount {item: nitric_acid.clone(), amount: 1 }, ItemAmount {item: empty_fluid_tank.clone(), amount: 1 }, ], output: ItemAmount {item: packaged_nitric_acid.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_PackagedNitricAcid
recipe_registry.entry("Packaged Nitric Acid").or_insert_with(Vec::new).push(packaged_nitric_acid_recipe);
let packaged_nitrogen_gas_recipe = Recipe { name: "Packaged Nitrogen Gas", machine: Machine::Packager, input: vec![ItemAmount {item: nitrogen_gas.clone(), amount: 4 }, ItemAmount {item: empty_fluid_tank.clone(), amount: 1 }, ], output: ItemAmount {item: packaged_nitrogen_gas.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_PackagedNitrogen
recipe_registry.entry("Packaged Nitrogen Gas").or_insert_with(Vec::new).push(packaged_nitrogen_gas_recipe);
let packaged_sulfuric_acid_recipe = Recipe { name: "Packaged Sulfuric Acid", machine: Machine::Packager, input: vec![ItemAmount {item: sulfuric_acid.clone(), amount: 2 }, ItemAmount {item: empty_canister.clone(), amount: 2 }, ], output: ItemAmount {item: packaged_sulfuric_acid.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_PackagedSulfuricAcid
recipe_registry.entry("Packaged Sulfuric Acid").or_insert_with(Vec::new).push(packaged_sulfuric_acid_recipe);
let alumina_solution_recipe = Recipe { name: "Alumina Solution", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_alumina_solution.clone(), amount: 2 }, ], output: ItemAmount {item: alumina_solution.clone(), amount: 2 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 2 }), manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_UnpackageAlumina
recipe_registry.entry("Alumina Solution").or_insert_with(Vec::new).push(alumina_solution_recipe);
let nitric_acid_recipe = Recipe { name: "Nitric Acid", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_nitric_acid.clone(), amount: 1 }, ], output: ItemAmount {item: nitric_acid.clone(), amount: 1 }, byproduct: Some(ItemAmount {item: empty_fluid_tank.clone(), amount: 1 }), manufacturing_duration: 3f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_UnpackageNitricAcid
recipe_registry.entry("Nitric Acid").or_insert_with(Vec::new).push(nitric_acid_recipe);
let nitrogen_gas_recipe = Recipe { name: "Nitrogen Gas", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_nitrogen_gas.clone(), amount: 1 }, ], output: ItemAmount {item: nitrogen_gas.clone(), amount: 4 }, byproduct: Some(ItemAmount {item: empty_fluid_tank.clone(), amount: 1 }), manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_UnpackageNitrogen
recipe_registry.entry("Nitrogen Gas").or_insert_with(Vec::new).push(nitrogen_gas_recipe);
let sulfuric_acid_recipe = Recipe { name: "Sulfuric Acid", machine: Machine::Packager, input: vec![ItemAmount {item: packaged_sulfuric_acid.clone(), amount: 1 }, ], output: ItemAmount {item: sulfuric_acid.clone(), amount: 1 }, byproduct: Some(ItemAmount {item: empty_canister.clone(), amount: 1 }), manufacturing_duration: 1f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Packager/Recipe_UnpackageSulfuricAcid
recipe_registry.entry("Sulfuric Acid").or_insert_with(Vec::new).push(sulfuric_acid_recipe);
let aluminum_ingot_recipe = Recipe { name: "Aluminum Ingot", machine: Machine::Foundry, input: vec![ItemAmount {item: aluminum_scrap.clone(), amount: 6 }, ItemAmount {item: silica.clone(), amount: 5 }, ], output: ItemAmount {item: aluminum_ingot.clone(), amount: 4 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Smelter/Recipe_IngotAluminum
recipe_registry.entry("Aluminum Ingot").or_insert_with(Vec::new).push(aluminum_ingot_recipe);
let caterium_ingot_recipe = Recipe { name: "Caterium Ingot", machine: Machine::Smelter, input: vec![ItemAmount {item: caterium_ore.clone(), amount: 3 }, ], output: ItemAmount {item: caterium_ingot.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Smelter/Recipe_IngotCaterium
recipe_registry.entry("Caterium Ingot").or_insert_with(Vec::new).push(caterium_ingot_recipe);
let copper_ingot_recipe = Recipe { name: "Copper Ingot", machine: Machine::Smelter, input: vec![ItemAmount {item: copper_ore.clone(), amount: 1 }, ], output: ItemAmount {item: copper_ingot.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Smelter/Recipe_IngotCopper
recipe_registry.entry("Copper Ingot").or_insert_with(Vec::new).push(copper_ingot_recipe);
let iron_ingot_recipe = Recipe { name: "Iron Ingot", machine: Machine::Smelter, input: vec![ItemAmount {item: iron_ore.clone(), amount: 1 }, ], output: ItemAmount {item: iron_ingot.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 2f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Smelter/Recipe_IngotIron
recipe_registry.entry("Iron Ingot").or_insert_with(Vec::new).push(iron_ingot_recipe);
let steel_ingot_recipe = Recipe { name: "Steel Ingot", machine: Machine::Foundry, input: vec![ItemAmount {item: iron_ore.clone(), amount: 3 }, ItemAmount {item: coal.clone(), amount: 3 }, ], output: ItemAmount {item: steel_ingot.clone(), amount: 3 }, byproduct: None, manufacturing_duration: 4f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/Smelter/Recipe_IngotSteel
recipe_registry.entry("Steel Ingot").or_insert_with(Vec::new).push(steel_ingot_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Assembler, input: vec![ItemAmount {item: reinforced_iron_plate.clone(), amount: 1 }, ItemAmount {item: rotor.clone(), amount: 1 }, ], output: ItemAmount {item: smart_plating.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 30f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_1
recipe_registry.entry("Smart Plating").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Assembler, input: vec![ItemAmount {item: modular_frame.clone(), amount: 1 }, ItemAmount {item: steel_beam.clone(), amount: 12 }, ], output: ItemAmount {item: versatile_framework.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 24f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_2
recipe_registry.entry("Versatile Framework").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Assembler, input: vec![ItemAmount {item: stator.clone(), amount: 1 }, ItemAmount {item: cable.clone(), amount: 20 }, ], output: ItemAmount {item: automated_wiring.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 24f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_3
recipe_registry.entry("Automated Wiring").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Manufacturer, input: vec![ItemAmount {item: motor.clone(), amount: 2 }, ItemAmount {item: rubber.clone(), amount: 15 }, ItemAmount {item: smart_plating.clone(), amount: 2 }, ], output: ItemAmount {item: modular_engine.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 60f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_4
recipe_registry.entry("Modular Engine").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Manufacturer, input: vec![ItemAmount {item: automated_wiring.clone(), amount: 15 }, ItemAmount {item: circuit_board.clone(), amount: 10 }, ItemAmount {item: heavy_modular_frame.clone(), amount: 2 }, ItemAmount {item: computer.clone(), amount: 2 }, ], output: ItemAmount {item: adaptive_control_unit.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 120f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_5
recipe_registry.entry("Adaptive Control Unit").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Manufacturer, input: vec![ItemAmount {item: versatile_framework.clone(), amount: 5 }, ItemAmount {item: electromagnetic_control_rod.clone(), amount: 2 }, ItemAmount {item: battery.clone(), amount: 10 }, ], output: ItemAmount {item: magnetic_field_generator.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 120f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_6
recipe_registry.entry("Magnetic Field Generator").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);
let assembly_director_system_recipe = Recipe { name: "Assembly Director System", machine: Machine::Assembler, input: vec![ItemAmount {item: adaptive_control_unit.clone(), amount: 2 }, ItemAmount {item: supercomputer.clone(), amount: 1 }, ], output: ItemAmount {item: assembly_director_system.clone(), amount: 1 }, byproduct: None, manufacturing_duration: 80f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_7
recipe_registry.entry("Assembly Director System").or_insert_with(Vec::new).push(assembly_director_system_recipe);
let alternate_iron_wire_recipe = Recipe { name: "Alternate: Iron Wire", machine: Machine::Manufacturer, input: vec![ItemAmount {item: modular_engine.clone(), amount: 5 }, ItemAmount {item: turbo_motor.clone(), amount: 2 }, ItemAmount {item: cooling_system.clone(), amount: 6 }, ItemAmount {item: fused_modular_frame.clone(), amount: 2 }, ], output: ItemAmount {item: thermal_propulsion_rocket.clone(), amount: 2 }, byproduct: None, manufacturing_duration: 120f32, alternate: false }; // FactoryGame/Content/FactoryGame/Recipes/SpaceElevatorParts/Recipe_SpaceElevatorPart_8
recipe_registry.entry("Thermal Propulsion Rocket").or_insert_with(Vec::new).push(alternate_iron_wire_recipe);

}
(item_registry, recipe_registry)
}
