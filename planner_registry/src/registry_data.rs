use crate::{
    buildings::building::Machine,
    items::{item::Item, recipe::Recipe, ItemAmount},
};
use std::{collections::HashMap, rc::Rc};
#[allow(non_snake_case)]
#[allow(clippy::redundant_clone)]
pub(crate) fn get_registry() -> (Vec<Rc<Item>>, HashMap<&'static str, Vec<Recipe>>) {
    let mut item_registry: Vec<Rc<Item>> = Vec::new();
    let mut recipe_registry: HashMap<&'static str, Vec<Recipe>> = HashMap::new();
    {
        let reinforced_iron_plate = Rc::new(Item {
            name: "Reinforced Iron Plate",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/IronPlateReinforced/Desc_IronPlateReinforced
        item_registry.push(reinforced_iron_plate.clone());
        let iron_plate = Rc::new(Item { name: "Iron Plate" }); // FactoryGame/Content/FactoryGame/Resource/Parts/IronPlate/Desc_IronPlate
        item_registry.push(iron_plate.clone());
        let rubber = Rc::new(Item { name: "Rubber" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Rubber/Desc_Rubber
        item_registry.push(rubber.clone());
        let modular_frame = Rc::new(Item {
            name: "Modular Frame",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrame/Desc_ModularFrame
        item_registry.push(modular_frame.clone());
        let screw = Rc::new(Item { name: "Screw" }); // FactoryGame/Content/FactoryGame/Resource/Parts/IronScrew/Desc_IronScrew
        item_registry.push(screw.clone());
        let cable = Rc::new(Item { name: "Cable" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Cable/Desc_Cable
        item_registry.push(cable.clone());
        let wire = Rc::new(Item { name: "Wire" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Wire/Desc_Wire
        item_registry.push(wire.clone());
        let heavy_oil_residue = Rc::new(Item {
            name: "Heavy Oil Residue",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/HeavyOilResidue/Desc_HeavyOilResidue
        item_registry.push(heavy_oil_residue.clone());
        let empty_canister = Rc::new(Item {
            name: "Empty Canister",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/FluidCanister/Desc_FluidCanister
        item_registry.push(empty_canister.clone());
        let copper_sheet = Rc::new(Item {
            name: "Copper Sheet",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/CopperSheet/Desc_CopperSheet
        item_registry.push(copper_sheet.clone());
        let iron_ingot = Rc::new(Item { name: "Iron Ingot" }); // FactoryGame/Content/FactoryGame/Resource/Parts/IronIngot/Desc_IronIngot
        item_registry.push(iron_ingot.clone());
        let plastic = Rc::new(Item { name: "Plastic" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Plastic/Desc_Plastic
        item_registry.push(plastic.clone());
        let steel_ingot = Rc::new(Item {
            name: "Steel Ingot",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelIngot/Desc_SteelIngot
        item_registry.push(steel_ingot.clone());
        let iron_ore = Rc::new(Item { name: "Iron Ore" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreIron/Desc_OreIron
        item_registry.push(iron_ore.clone());
        let petroleum_coke = Rc::new(Item {
            name: "Petroleum Coke",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/PetroleumCoke/Desc_PetroleumCoke
        item_registry.push(petroleum_coke.clone());
        let copper_ingot = Rc::new(Item {
            name: "Copper Ingot",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/CopperIngot/Desc_CopperIngot
        item_registry.push(copper_ingot.clone());
        let copper_ore = Rc::new(Item { name: "Copper Ore" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreCopper/Desc_OreCopper
        item_registry.push(copper_ore.clone());
        let rotor = Rc::new(Item { name: "Rotor" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Rotor/Desc_Rotor
        item_registry.push(rotor.clone());
        let packaged_fuel = Rc::new(Item {
            name: "Packaged Fuel",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/Fuel/Desc_Fuel
        item_registry.push(packaged_fuel.clone());
        let packaged_water = Rc::new(Item {
            name: "Packaged Water",
        }); // FactoryGame/Content/FactoryGame/Resource/RawResources/Water/Desc_PackagedWater
        item_registry.push(packaged_water.clone());
        let aluminum_scrap = Rc::new(Item {
            name: "Aluminum Scrap",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumScrap/Desc_AluminumScrap
        item_registry.push(aluminum_scrap.clone());
        let alumina_solution = Rc::new(Item {
            name: "Alumina Solution",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/Alumina/Desc_AluminaSolution
        item_registry.push(alumina_solution.clone());
        let circuit_board = Rc::new(Item {
            name: "Circuit Board",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/CircuitBoard/Desc_CircuitBoard
        item_registry.push(circuit_board.clone());
        let versatile_framework = Rc::new(Item {
            name: "Versatile Framework",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_2
        item_registry.push(versatile_framework.clone());
        let steel_beam = Rc::new(Item { name: "Steel Beam" }); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelPlate/Desc_SteelPlate
        item_registry.push(steel_beam.clone());
        let caterium_ingot = Rc::new(Item {
            name: "Caterium Ingot",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/GoldIngot/Desc_GoldIngot
        item_registry.push(caterium_ingot.clone());
        let heavy_modular_frame = Rc::new(Item {
            name: "Heavy Modular Frame",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrameHeavy/Desc_ModularFrameHeavy
        item_registry.push(heavy_modular_frame.clone());
        let encased_industrial_beam = Rc::new(Item {
            name: "Encased Industrial Beam",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelPlateReinforced/Desc_SteelPlateReinforced
        item_registry.push(encased_industrial_beam.clone());
        let crude_oil = Rc::new(Item { name: "Crude Oil" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/CrudeOil/Desc_LiquidOil
        item_registry.push(crude_oil.clone());
        let automated_wiring = Rc::new(Item {
            name: "Automated Wiring",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_3
        item_registry.push(automated_wiring.clone());
        let stator = Rc::new(Item { name: "Stator" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Stator/Desc_Stator
        item_registry.push(stator.clone());
        let high_speed_connector = Rc::new(Item {
            name: "High-Speed Connector",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/HighSpeedConnector/Desc_HighSpeedConnector
        item_registry.push(high_speed_connector.clone());
        let smart_plating = Rc::new(Item {
            name: "Smart Plating",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_1
        item_registry.push(smart_plating.clone());
        let fabric = Rc::new(Item { name: "Fabric" }); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Fabric
        item_registry.push(fabric.clone());
        let polymer_resin = Rc::new(Item {
            name: "Polymer Resin",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/PolymerResin/Desc_PolymerResin
        item_registry.push(polymer_resin.clone());
        let water = Rc::new(Item { name: "Water" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/Water/Desc_Water
        item_registry.push(water.clone());
        let caterium_ore = Rc::new(Item {
            name: "Caterium Ore",
        }); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreGold/Desc_OreGold
        item_registry.push(caterium_ore.clone());
        let quartz_crystal = Rc::new(Item {
            name: "Quartz Crystal",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/QuartzCrystal/Desc_QuartzCrystal
        item_registry.push(quartz_crystal.clone());
        let raw_quartz = Rc::new(Item { name: "Raw Quartz" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/RawQuartz/Desc_RawQuartz
        item_registry.push(raw_quartz.clone());
        let fuel = Rc::new(Item { name: "Fuel" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Fuel/Desc_LiquidFuel
        item_registry.push(fuel.clone());
        let concrete = Rc::new(Item { name: "Concrete" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Cement/Desc_Cement
        item_registry.push(concrete.clone());
        let limestone = Rc::new(Item { name: "Limestone" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/Stone/Desc_Stone
        item_registry.push(limestone.clone());
        let iron_rod = Rc::new(Item { name: "Iron Rod" }); // FactoryGame/Content/FactoryGame/Resource/Parts/IronRod/Desc_IronRod
        item_registry.push(iron_rod.clone());
        let turbofuel = Rc::new(Item { name: "Turbofuel" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Turbofuel/Desc_LiquidTurboFuel
        item_registry.push(turbofuel.clone());
        let compacted_coal = Rc::new(Item {
            name: "Compacted Coal",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/CompactedCoal/Desc_CompactedCoal
        item_registry.push(compacted_coal.clone());
        let aluminum_ingot = Rc::new(Item {
            name: "Aluminum Ingot",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumIngot/Desc_AluminumIngot
        item_registry.push(aluminum_ingot.clone());
        let aluminum_casing = Rc::new(Item {
            name: "Aluminum Casing",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumCasing/Desc_AluminumCasing
        item_registry.push(aluminum_casing.clone());
        let portable_miner = Rc::new(Item {
            name: "Portable Miner",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/PortableMiner/BP_ItemDescriptorPortableMiner
        item_registry.push(portable_miner.clone());
        let motor = Rc::new(Item { name: "Motor" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Motor/Desc_Motor
        item_registry.push(motor.clone());
        let steel_pipe = Rc::new(Item { name: "Steel Pipe" }); // FactoryGame/Content/FactoryGame/Resource/Parts/SteelPipe/Desc_SteelPipe
        item_registry.push(steel_pipe.clone());
        let battery = Rc::new(Item { name: "Battery" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Battery/Desc_Battery
        item_registry.push(battery.clone());
        let sulfur = Rc::new(Item { name: "Sulfur" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/Sulfur/Desc_Sulfur
        item_registry.push(sulfur.clone());
        let alclad_aluminum_sheet = Rc::new(Item {
            name: "Alclad Aluminum Sheet",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumPlate/Desc_AluminumPlate
        item_registry.push(alclad_aluminum_sheet.clone());
        let cooling_system = Rc::new(Item {
            name: "Cooling System",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/CoolingSystem/Desc_CoolingSystem
        item_registry.push(cooling_system.clone());
        let heat_sink = Rc::new(Item { name: "Heat Sink" }); // FactoryGame/Content/FactoryGame/Resource/Parts/AluminumPlateReinforced/Desc_AluminumPlateReinforced
        item_registry.push(heat_sink.clone());
        let nitrogen_gas = Rc::new(Item {
            name: "Nitrogen Gas",
        }); // FactoryGame/Content/FactoryGame/Resource/RawResources/NitrogenGas/Desc_NitrogenGas
        item_registry.push(nitrogen_gas.clone());
        let electromagnetic_control_rod = Rc::new(Item {
            name: "Electromagnetic Control Rod",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/ElectromagneticControlRod/Desc_ElectromagneticControlRod
        item_registry.push(electromagnetic_control_rod.clone());
        let non_fissile_uranium = Rc::new(Item {
            name: "Non-fissile Uranium",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/Non-FissibleUranium/Desc_NonFissibleUranium
        item_registry.push(non_fissile_uranium.clone());
        let uranium = Rc::new(Item { name: "Uranium" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreUranium/Desc_OreUranium
        item_registry.push(uranium.clone());
        let uranium_waste = Rc::new(Item {
            name: "Uranium Waste",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/NuclearWaste/Desc_NuclearWaste
        item_registry.push(uranium_waste.clone());
        let nitric_acid = Rc::new(Item {
            name: "Nitric Acid",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/NitricAcid/Desc_NitricAcid
        item_registry.push(nitric_acid.clone());
        let sulfuric_acid = Rc::new(Item {
            name: "Sulfuric Acid",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SulfuricAcid/Desc_SulfuricAcid
        item_registry.push(sulfuric_acid.clone());
        let fused_modular_frame = Rc::new(Item {
            name: "Fused Modular Frame",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrameFused/Desc_ModularFrameFused
        item_registry.push(fused_modular_frame.clone());
        let encased_plutonium_cell = Rc::new(Item {
            name: "Encased Plutonium Cell",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/PlutoniumCell/Desc_PlutoniumCell
        item_registry.push(encased_plutonium_cell.clone());
        let bauxite = Rc::new(Item { name: "Bauxite" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/OreBauxite/Desc_OreBauxite
        item_registry.push(bauxite.clone());
        let coal = Rc::new(Item { name: "Coal" }); // FactoryGame/Content/FactoryGame/Resource/RawResources/Coal/Desc_Coal
        item_registry.push(coal.clone());
        let supercomputer = Rc::new(Item {
            name: "Supercomputer",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/ComputerSuper/Desc_ComputerSuper
        item_registry.push(supercomputer.clone());
        let radio_control_unit = Rc::new(Item {
            name: "Radio Control Unit",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/ModularFrameLightweight/Desc_ModularFrameLightweight
        item_registry.push(radio_control_unit.clone());
        let plutonium_fuel_rod = Rc::new(Item {
            name: "Plutonium Fuel Rod",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/PlutoniumFuelRods/Desc_PlutoniumFuelRod
        item_registry.push(plutonium_fuel_rod.clone());
        let pressure_conversion_cube = Rc::new(Item {
            name: "Pressure Conversion Cube",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/PressureConversionCube/Desc_PressureConversionCube
        item_registry.push(pressure_conversion_cube.clone());
        let crystal_oscillator = Rc::new(Item {
            name: "Crystal Oscillator",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/CrystalOscillator/Desc_CrystalOscillator
        item_registry.push(crystal_oscillator.clone());
        let computer = Rc::new(Item { name: "Computer" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Computer/Desc_Computer
        item_registry.push(computer.clone());
        let turbo_motor = Rc::new(Item {
            name: "Turbo Motor",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/MotorLightweight/Desc_MotorLightweight
        item_registry.push(turbo_motor.clone());
        let packaged_nitrogen_gas = Rc::new(Item {
            name: "Packaged Nitrogen Gas",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/PackagedNitrogen/Desc_PackagedNitrogenGas
        item_registry.push(packaged_nitrogen_gas.clone());
        let beacon = Rc::new(Item { name: "Beacon" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/Beacon/BP_EquipmentDescriptorBeacon
        item_registry.push(beacon.clone());
        let quickwire = Rc::new(Item { name: "Quickwire" }); // FactoryGame/Content/FactoryGame/Resource/Parts/HighSpeedWire/Desc_HighSpeedWire
        item_registry.push(quickwire.clone());
        let silica = Rc::new(Item { name: "Silica" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Silica/Desc_Silica
        item_registry.push(silica.clone());
        let wood = Rc::new(Item { name: "Wood" }); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Wood
        item_registry.push(wood.clone());
        let biomass = Rc::new(Item { name: "Biomass" }); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_GenericBiomass
        item_registry.push(biomass.clone());
        let ailimiter = Rc::new(Item { name: "AI Limiter" }); // FactoryGame/Content/FactoryGame/Resource/Parts/CircuitBoardHighSpeed/Desc_CircuitBoardHighSpeed
        item_registry.push(ailimiter.clone());
        let black_powder = Rc::new(Item {
            name: "Black Powder",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/GunPowder/Desc_Gunpowder
        item_registry.push(black_powder.clone());
        let uranium_fuel_rod = Rc::new(Item {
            name: "Uranium Fuel Rod",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/NuclearFuelRod/Desc_NuclearFuelRod
        item_registry.push(uranium_fuel_rod.clone());
        let encased_uranium_cell = Rc::new(Item {
            name: "Encased Uranium Cell",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/UraniumCell/Desc_UraniumCell
        item_registry.push(encased_uranium_cell.clone());
        let plutonium_pellet = Rc::new(Item {
            name: "Plutonium Pellet",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/PlutoniumPellet/Desc_PlutoniumPellet
        item_registry.push(plutonium_pellet.clone());
        let assembler = Rc::new(Item { name: "Assembler" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/AssemblerMk1/Desc_AssemblerMk1
        item_registry.push(assembler.clone());
        let blender = Rc::new(Item { name: "Blender" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Blender/Desc_Blender
        item_registry.push(blender.clone());
        let blueprint_designer = Rc::new(Item {
            name: "Blueprint Designer",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/BlueprintDesigner/Desc_BlueprintDesigner
        item_registry.push(blueprint_designer.clone());
        let ceiling_light = Rc::new(Item {
            name: "Ceiling Light",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/CeilingLight/Desc_CeilingLight
        item_registry.push(ceiling_light.clone());
        let constructor = Rc::new(Item {
            name: "Constructor",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConstructorMk1/Desc_ConstructorMk1
        item_registry.push(constructor.clone());
        let conveyor_merger = Rc::new(Item {
            name: "Conveyor Merger",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/CA_Merger/Desc_ConveyorAttachmentMerger
        item_registry.push(conveyor_merger.clone());
        let conveyor_splitter = Rc::new(Item {
            name: "Conveyor Splitter",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/CA_Splitter/Desc_ConveyorAttachmentSplitter
        item_registry.push(conveyor_splitter.clone());
        let programmable_splitter = Rc::new(Item {
            name: "Programmable Splitter",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/CA_SplitterProgrammable/Desc_ConveyorAttachmentSplitterProgrammable
        item_registry.push(programmable_splitter.clone());
        let smart_splitter = Rc::new(Item {
            name: "Smart Splitter",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/CA_SplitterSmart/Desc_ConveyorAttachmentSplitterSmart
        item_registry.push(smart_splitter.clone());
        let conveyor_belt_mk_1 = Rc::new(Item {
            name: "Conveyor Belt Mk.1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorBeltMk1/Desc_ConveyorBeltMk1
        item_registry.push(conveyor_belt_mk_1.clone());
        let conveyor_belt_mk_2 = Rc::new(Item {
            name: "Conveyor Belt Mk.2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorBeltMk2/Desc_ConveyorBeltMk2
        item_registry.push(conveyor_belt_mk_2.clone());
        let conveyor_belt_mk_3 = Rc::new(Item {
            name: "Conveyor Belt Mk.3",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorBeltMk3/Desc_ConveyorBeltMk3
        item_registry.push(conveyor_belt_mk_3.clone());
        let conveyor_belt_mk_4 = Rc::new(Item {
            name: "Conveyor Belt Mk.4",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorBeltMk4/Desc_ConveyorBeltMk4
        item_registry.push(conveyor_belt_mk_4.clone());
        let conveyor_belt_mk_5 = Rc::new(Item {
            name: "Conveyor Belt Mk.5",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorBeltMk5/Desc_ConveyorBeltMk5
        item_registry.push(conveyor_belt_mk_5.clone());
        let conveyor_ceiling_mount = Rc::new(Item {
            name: "Conveyor Ceiling Mount",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorPoleWall/Desc_ConveyorCeilingAttachment
        item_registry.push(conveyor_ceiling_mount.clone());
        let conveyor_lift_mk_1 = Rc::new(Item {
            name: "Conveyor Lift Mk.1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorLiftMk1/Desc_ConveyorLiftMk1
        item_registry.push(conveyor_lift_mk_1.clone());
        let conveyor_lift_mk_2 = Rc::new(Item {
            name: "Conveyor Lift Mk.2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorLiftMk2/Desc_ConveyorLiftMk2
        item_registry.push(conveyor_lift_mk_2.clone());
        let conveyor_lift_mk_3 = Rc::new(Item {
            name: "Conveyor Lift Mk.3",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorLiftMk3/Desc_ConveyorLiftMk3
        item_registry.push(conveyor_lift_mk_3.clone());
        let conveyor_lift_mk_4 = Rc::new(Item {
            name: "Conveyor Lift Mk.4",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorLiftMk4/Desc_ConveyorLiftMk4
        item_registry.push(conveyor_lift_mk_4.clone());
        let conveyor_lift_mk_5 = Rc::new(Item {
            name: "Conveyor Lift Mk.5",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorLiftMk5/Desc_ConveyorLiftMk5
        item_registry.push(conveyor_lift_mk_5.clone());
        let conveyor_pole = Rc::new(Item {
            name: "Conveyor Pole",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorPole/Desc_ConveyorPole
        item_registry.push(conveyor_pole.clone());
        let stackable_conveyor_pole = Rc::new(Item {
            name: "Stackable Conveyor Pole",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorPoleStackable/Desc_ConveyorPoleStackable
        item_registry.push(stackable_conveyor_pole.clone());
        let conveyor_wall_mount = Rc::new(Item {
            name: "Conveyor Wall Mount",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ConveyorPoleWall/Desc_ConveyorPoleWall
        item_registry.push(conveyor_wall_mount.clone());
        let drone_port = Rc::new(Item { name: "Drone Port" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/DroneStation/Desc_DroneStation
        item_registry.push(drone_port.clone());
        let flood_light_tower = Rc::new(Item {
            name: "Flood Light Tower",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Floodlight/Desc_FloodlightPole
        item_registry.push(flood_light_tower.clone());
        let wall_mounted_flood_light = Rc::new(Item {
            name: "Wall Mounted Flood Light",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Floodlight/Desc_FloodlightWall
        item_registry.push(wall_mounted_flood_light.clone());
        let resource_well_extractor = Rc::new(Item {
            name: "Resource Well Extractor",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/FrackingExtractor/Desc_FrackingExtractor
        item_registry.push(resource_well_extractor.clone());
        let resource_well_pressurizer = Rc::new(Item {
            name: "Resource Well Pressurizer",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/FrackingSmasher/Desc_FrackingSmasher
        item_registry.push(resource_well_pressurizer.clone());
        let biomass_burner = Rc::new(Item {
            name: "Biomass Burner",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/GeneratorBiomass/Desc_GeneratorBiomass
        item_registry.push(biomass_burner.clone());
        let coal_generator = Rc::new(Item {
            name: "Coal Generator",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/GeneratorCoal/Desc_GeneratorCoal
        item_registry.push(coal_generator.clone());
        let fuel_generator = Rc::new(Item {
            name: "Fuel Generator",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/GeneratorFuel/Desc_GeneratorFuel
        item_registry.push(fuel_generator.clone());
        let geothermal_generator = Rc::new(Item {
            name: "Geothermal Generator",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/GeneratorGeoThermal/Desc_GeneratorGeoThermal
        item_registry.push(geothermal_generator.clone());
        let nuclear_power_plant = Rc::new(Item {
            name: "Nuclear Power Plant",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/GeneratorNuclear/Desc_GeneratorNuclear
        item_registry.push(nuclear_power_plant.clone());
        let particle_accelerator = Rc::new(Item {
            name: "Particle Accelerator",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/HadronCollider/Desc_HadronCollider
        item_registry.push(particle_accelerator.clone());
        let stackable_hypertube_support = Rc::new(Item {
            name: "Stackable Hypertube Support",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipelineSupport/Desc_HyperPoleStackable
        item_registry.push(stackable_hypertube_support.clone());
        let hypertube_wall_hole = Rc::new(Item {
            name: "Hypertube Wall Hole",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/HyperTubeWallSupport/Desc_HyperTubeWallHole
        item_registry.push(hypertube_wall_hole.clone());
        let hypertube_wall_support = Rc::new(Item {
            name: "Hypertube Wall Support",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/HyperTubeWallSupport/Desc_HyperTubeWallSupport
        item_registry.push(hypertube_wall_support.clone());
        let industrial_fluid_buffer = Rc::new(Item {
            name: "Industrial Fluid Buffer",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/IndustrialFluidContainer/Desc_IndustrialTank
        item_registry.push(industrial_fluid_buffer.clone());
        let old_jump_pad = Rc::new(Item {
            name: "Old Jump Pad",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/JumpPad/Desc_JumpPad
        item_registry.push(old_jump_pad.clone());
        let jump_pad = Rc::new(Item { name: "Jump Pad" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/JumpPad/Desc_JumpPadAdjustable
        item_registry.push(jump_pad.clone());
        let old_tilted_jump_pad = Rc::new(Item {
            name: "Old Tilted Jump Pad",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/JumpPad/Desc_JumpPadTilted
        item_registry.push(old_tilted_jump_pad.clone());
        let lights_control_panel = Rc::new(Item {
            name: "Lights Control Panel",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/LightsControlPanel/Desc_LightsControlPanel
        item_registry.push(lights_control_panel.clone());
        let lookout_tower = Rc::new(Item {
            name: "Lookout Tower",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/LookoutTower/Desc_LookoutTower
        item_registry.push(lookout_tower.clone());
        let mam = Rc::new(Item { name: "MAM" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Mam/Desc_Mam
        item_registry.push(mam.clone());
        let manufacturer = Rc::new(Item {
            name: "Manufacturer",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ManufacturerMk1/Desc_ManufacturerMk1
        item_registry.push(manufacturer.clone());
        let miner_mk_1 = Rc::new(Item { name: "Miner Mk.1" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/MinerMK1/Desc_MinerMk1
        item_registry.push(miner_mk_1.clone());
        let miner_mk_2 = Rc::new(Item { name: "Miner Mk.2" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/MinerMk2/Desc_MinerMk2
        item_registry.push(miner_mk_2.clone());
        let miner_mk_3 = Rc::new(Item { name: "Miner Mk.3" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/MinerMk3/Desc_MinerMk3
        item_registry.push(miner_mk_3.clone());
        let oil_extractor = Rc::new(Item {
            name: "Oil Extractor",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/OilPump/Desc_OilPump
        item_registry.push(oil_extractor.clone());
        let refinery = Rc::new(Item { name: "Refinery" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/OilRefinery/Desc_OilRefinery
        item_registry.push(refinery.clone());
        let packager = Rc::new(Item { name: "Packager" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Packager/Desc_Packager
        item_registry.push(packager.clone());
        let hypertube = Rc::new(Item { name: "Hypertube" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipeHyper/Desc_PipeHyper
        item_registry.push(hypertube.clone());
        let hypertube_support = Rc::new(Item {
            name: "Hypertube Support",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipeHyperSupport/Desc_PipeHyperSupport
        item_registry.push(hypertube_support.clone());
        let pipeline_mk_1 = Rc::new(Item {
            name: "Pipeline Mk.1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Pipeline/Desc_Pipeline
        item_registry.push(pipeline_mk_1.clone());
        let pipeline_mk_1_no_indicator_ = Rc::new(Item {
            name: "Pipeline Mk.1 (No Indicator)",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Pipeline/Desc_Pipeline_NoIndicator
        item_registry.push(pipeline_mk_1_no_indicator_.clone());
        let pipeline_junction_cross = Rc::new(Item {
            name: "Pipeline Junction Cross",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipeJunction/Desc_PipelineJunction_Cross
        item_registry.push(pipeline_junction_cross.clone());
        let pipeline_mk_2 = Rc::new(Item {
            name: "Pipeline Mk.2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipelineMk2/Desc_PipelineMK2
        item_registry.push(pipeline_mk_2.clone());
        let pipeline_mk_2_no_indicator_ = Rc::new(Item {
            name: "Pipeline Mk.2 (No Indicator)",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipelineMk2/Desc_PipelineMK2_NoIndicator
        item_registry.push(pipeline_mk_2_no_indicator_.clone());
        let pipeline_pump_mk_1 = Rc::new(Item {
            name: "Pipeline Pump Mk.1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipePump/Desc_PipelinePump
        item_registry.push(pipeline_pump_mk_1.clone());
        let pipeline_pump_mk_2 = Rc::new(Item {
            name: "Pipeline Pump Mk.2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipePumpMk2/Desc_PipelinePumpMK2
        item_registry.push(pipeline_pump_mk_2.clone());
        let fluid_buffer = Rc::new(Item {
            name: "Fluid Buffer",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/StorageTank/Desc_PipeStorageTank
        item_registry.push(fluid_buffer.clone());
        let pipeline_support = Rc::new(Item {
            name: "Pipeline Support",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipelineSupport/Desc_PipelineSupport
        item_registry.push(pipeline_support.clone());
        let stackable_pipeline_support = Rc::new(Item {
            name: "Stackable Pipeline Support",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipelineSupport/Desc_PipeSupportStackable
        item_registry.push(stackable_pipeline_support.clone());
        let pipeline_wall_support = Rc::new(Item {
            name: "Pipeline Wall Support",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipelineSupportWall/Desc_PipelineSupportWall
        item_registry.push(pipeline_wall_support.clone());
        let pipeline_wall_hole = Rc::new(Item {
            name: "Pipeline Wall Hole",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipelineSupportWallHole/Desc_PipelineSupportWallHole
        item_registry.push(pipeline_wall_hole.clone());
        let power_line = Rc::new(Item { name: "Power Line" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerLine/Desc_PowerLine
        item_registry.push(power_line.clone());
        let power_pole_mk_1 = Rc::new(Item {
            name: "Power Pole Mk.1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleMk1/Desc_PowerPoleMk1
        item_registry.push(power_pole_mk_1.clone());
        let power_pole_mk_2 = Rc::new(Item {
            name: "Power Pole Mk.2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleMk2/Desc_PowerPoleMk2
        item_registry.push(power_pole_mk_2.clone());
        let power_pole_mk_3 = Rc::new(Item {
            name: "Power Pole Mk.3",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleMk3/Desc_PowerPoleMk3
        item_registry.push(power_pole_mk_3.clone());
        let wall_outlet_mk_1 = Rc::new(Item {
            name: "Wall Outlet Mk.1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleWall/Desc_PowerPoleWall
        item_registry.push(wall_outlet_mk_1.clone());
        let double_wall_outlet_mk_1 = Rc::new(Item {
            name: "Double Wall Outlet Mk.1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleWallDouble/Desc_PowerPoleWallDouble
        item_registry.push(double_wall_outlet_mk_1.clone());
        let double_wall_outlet_mk_2 = Rc::new(Item {
            name: "Double Wall Outlet Mk.2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleWallDouble/Desc_PowerPoleWallDoubleMk2
        item_registry.push(double_wall_outlet_mk_2.clone());
        let double_wall_outlet_mk_3 = Rc::new(Item {
            name: "Double Wall Outlet Mk.3",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleWallDouble/Desc_PowerPoleWallDoubleMk3
        item_registry.push(double_wall_outlet_mk_3.clone());
        let wall_outlet_mk_2 = Rc::new(Item {
            name: "Wall Outlet Mk.2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleWall/Desc_PowerPoleWallMk2
        item_registry.push(wall_outlet_mk_2.clone());
        let wall_outlet_mk_3 = Rc::new(Item {
            name: "Wall Outlet Mk.3",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerPoleWall/Desc_PowerPoleWallMk3
        item_registry.push(wall_outlet_mk_3.clone());
        let power_storage = Rc::new(Item {
            name: "Power Storage",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerStorage/Desc_PowerStorageMk1
        item_registry.push(power_storage.clone());
        let power_switch = Rc::new(Item {
            name: "Power Switch",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PowerSwitch/Desc_PowerSwitch
        item_registry.push(power_switch.clone());
        let radar_tower = Rc::new(Item {
            name: "Radar Tower",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/RadarTower/Desc_RadarTower
        item_registry.push(radar_tower.clone());
        let railway = Rc::new(Item { name: "Railway" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Train/Track/Desc_RailroadTrack
        item_registry.push(railway.clone());
        let awesomesink = Rc::new(Item {
            name: "AWESOME Sink",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ResourceSink/Desc_ResourceSink
        item_registry.push(awesomesink.clone());
        let awesomeshop = Rc::new(Item {
            name: "AWESOME Shop",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/ResourceSinkShop/Desc_ResourceSinkShop
        item_registry.push(awesomeshop.clone());
        let smelter = Rc::new(Item { name: "Smelter" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SmelterMk1/Desc_SmelterMk1
        item_registry.push(smelter.clone());
        let foundry = Rc::new(Item { name: "Foundry" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/FoundryMk1/Desc_FoundryMk1
        item_registry.push(foundry.clone());
        let large_billboard = Rc::new(Item {
            name: "Large Billboard",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Huge
        item_registry.push(large_billboard.clone());
        let small_billboard = Rc::new(Item {
            name: "Small Billboard",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Large
        item_registry.push(small_billboard.clone());
        let display_sign = Rc::new(Item {
            name: "Display Sign",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Medium
        item_registry.push(display_sign.clone());
        let portrait_sign = Rc::new(Item {
            name: "Portrait Sign",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Portrait
        item_registry.push(portrait_sign.clone());
        let label_sign2m = Rc::new(Item {
            name: "Label Sign 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Small
        item_registry.push(label_sign2m.clone());
        let label_sign4m = Rc::new(Item {
            name: "Label Sign 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_SmallVeryWide
        item_registry.push(label_sign4m.clone());
        let label_sign3m = Rc::new(Item {
            name: "Label Sign 3m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_SmallWide
        item_registry.push(label_sign3m.clone());
        let square_sign2m = Rc::new(Item {
            name: "Square Sign 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Square
        item_registry.push(square_sign2m.clone());
        let square_sign1m = Rc::new(Item {
            name: "Square Sign 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Square_Small
        item_registry.push(square_sign1m.clone());
        let square_sign0_5m = Rc::new(Item {
            name: "Square Sign 0.5m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SignDigital/Desc_StandaloneWidgetSign_Square_Tiny
        item_registry.push(square_sign0_5m.clone());
        let storage_container = Rc::new(Item {
            name: "Storage Container",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/StorageContainerMk1/Desc_StorageContainerMk1
        item_registry.push(storage_container.clone());
        let industrial_storage_container = Rc::new(Item {
            name: "Industrial Storage Container",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/StorageContainerMk2/Desc_StorageContainerMk2
        item_registry.push(industrial_storage_container.clone());
        let hazard_storage_box = Rc::new(Item {
            name: "Hazard Storage Box",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/StoragePlayer/Desc_StorageHazard
        item_registry.push(hazard_storage_box.clone());
        let medical_storage_box = Rc::new(Item {
            name: "Medical Storage Box",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/StoragePlayer/Desc_StorageMedkit
        item_registry.push(medical_storage_box.clone());
        let personal_storage_box = Rc::new(Item {
            name: "Personal Storage Box",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/StoragePlayer/Desc_StoragePlayer
        item_registry.push(personal_storage_box.clone());
        let street_light = Rc::new(Item {
            name: "Street Light",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/StreetLight/Desc_StreetLight
        item_registry.push(street_light.clone());
        let the_hub = Rc::new(Item { name: "The HUB" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/TradingPost/Desc_TradingPost
        item_registry.push(the_hub.clone());
        let hubparts = Rc::new(Item { name: "HUB Parts" }); // FactoryGame/Content/FactoryGame/Resource/Parts/HUBParts/Desc_HUBParts
        item_registry.push(hubparts.clone());
        let truck_station = Rc::new(Item {
            name: "Truck Station",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/TruckStation/Desc_TruckStation
        item_registry.push(truck_station.clone());
        let u_jelly_landing_pad = Rc::new(Item {
            name: "U-Jelly Landing Pad",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/LandingPad/Desc_LandingPad
        item_registry.push(u_jelly_landing_pad.clone());
        let valve = Rc::new(Item { name: "Valve" }); // FactoryGame/Content/FactoryGame/Buildable/Factory/PipeValve/Desc_Valve
        item_registry.push(valve.clone());
        let water_extractor = Rc::new(Item {
            name: "Water Extractor",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/WaterPump/Desc_WaterPump
        item_registry.push(water_extractor.clone());
        let craft_bench = Rc::new(Item {
            name: "Craft Bench",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/WorkBench/Desc_WorkBench
        item_registry.push(craft_bench.clone());
        let equipment_workshop = Rc::new(Item {
            name: "Equipment Workshop",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Workshop/Desc_Workshop
        item_registry.push(equipment_workshop.clone());
        let catwalk_crossing = Rc::new(Item {
            name: "Catwalk Crossing",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Catwalk/Desc_CatwalkCross
        item_registry.push(catwalk_crossing.clone());
        let catwalk_ramp = Rc::new(Item {
            name: "Catwalk Ramp",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Catwalk/Desc_CatwalkRamp
        item_registry.push(catwalk_ramp.clone());
        let catwalk_stairs = Rc::new(Item {
            name: "Catwalk Stairs",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Catwalk/Desc_CatwalkStairs
        item_registry.push(catwalk_stairs.clone());
        let catwalk_straight = Rc::new(Item {
            name: "Catwalk Straight",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Catwalk/Desc_CatwalkStraight
        item_registry.push(catwalk_straight.clone());
        let catwalk_t_crossing = Rc::new(Item {
            name: "Catwalk T-Crossing",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Catwalk/Desc_CatwalkT
        item_registry.push(catwalk_t_crossing.clone());
        let catwalk_corner = Rc::new(Item {
            name: "Catwalk Corner",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Catwalk/Desc_CatwalkTurn
        item_registry.push(catwalk_corner.clone());
        let road_barrier = Rc::new(Item {
            name: "Road Barrier",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Fence/Desc_Concrete_Barrier_01
        item_registry.push(road_barrier.clone());
        let industrial_railing = Rc::new(Item {
            name: "Industrial Railing",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Fence/Desc_Fence_01
        item_registry.push(industrial_railing.clone());
        let modern_railing = Rc::new(Item {
            name: "Modern Railing",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Fence/Desc_Railing_01
        item_registry.push(modern_railing.clone());
        let frame_floor = Rc::new(Item {
            name: "Frame Floor",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_Flat_Frame_01
        item_registry.push(frame_floor.clone());
        let foundation1m = Rc::new(Item {
            name: "Foundation 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_Foundation_8x1_01
        item_registry.push(foundation1m.clone());
        let foundation2m = Rc::new(Item {
            name: "Foundation 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_Foundation_8x2_01
        item_registry.push(foundation2m.clone());
        let foundation4m = Rc::new(Item {
            name: "Foundation 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_Foundation_8x4_01
        item_registry.push(foundation4m.clone());
        let frame_foundation = Rc::new(Item {
            name: "Frame Foundation",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_Foundation_Frame_01
        item_registry.push(frame_foundation.clone());
        let glass_frame_foundation = Rc::new(Item {
            name: "Glass Frame Foundation",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_FoundationGlass_01
        item_registry.push(glass_frame_foundation.clone());
        let big_pillar_support = Rc::new(Item {
            name: "Big Pillar Support",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_PillarBase
        item_registry.push(big_pillar_support.clone());
        let big_metal_pillar = Rc::new(Item {
            name: "Big Metal Pillar",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_PillarMiddle
        item_registry.push(big_metal_pillar.clone());
        let big_concrete_pillar = Rc::new(Item {
            name: "Big Concrete Pillar",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_PillarMiddle_Concrete
        item_registry.push(big_concrete_pillar.clone());
        let big_frame_pillar = Rc::new(Item {
            name: "Big Frame Pillar",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_PillarMiddle_Frame
        item_registry.push(big_frame_pillar.clone());
        let pillar_top = Rc::new(Item { name: "Pillar Top" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_PillarTop
        item_registry.push(pillar_top.clone());
        let quarter_pipe = Rc::new(Item {
            name: "Quarter Pipe",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_QuarterPipe
        item_registry.push(quarter_pipe.clone());
        let inverted_quarter_pipe = Rc::new(Item {
            name: "Inverted Quarter Pipe",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_QuarterPipe_02
        item_registry.push(inverted_quarter_pipe.clone());
        let inner_corner_quarter_pipe = Rc::new(Item {
            name: "Inner Corner Quarter Pipe",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_QuarterPipeCorner_01
        item_registry.push(inner_corner_quarter_pipe.clone());
        let inverted_inner_corner_quarter_pipe = Rc::new(Item {
            name: "Inverted Inner Corner Quarter Pipe",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_QuarterPipeCorner_02
        item_registry.push(inverted_inner_corner_quarter_pipe.clone());
        let outer_corner_quarter_pipe = Rc::new(Item {
            name: "Outer Corner Quarter Pipe",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_QuarterPipeCorner_03
        item_registry.push(outer_corner_quarter_pipe.clone());
        let inverted_outer_corner_quarter_pipe = Rc::new(Item {
            name: "Inverted Outer Corner Quarter Pipe",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Foundation/Desc_QuarterPipeCorner_04
        item_registry.push(inverted_outer_corner_quarter_pipe.clone());
        let ladder = Rc::new(Item { name: "Ladder" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ladder/Desc_Ladder
        item_registry.push(ladder.clone());
        let ramp1m = Rc::new(Item { name: "Ramp 1m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_8x1_01
        item_registry.push(ramp1m.clone());
        let ramp2m = Rc::new(Item { name: "Ramp 2m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_8x2_01
        item_registry.push(ramp2m.clone());
        let ramp4m = Rc::new(Item { name: "Ramp 4m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_8x4_01
        item_registry.push(ramp4m.clone());
        let inv_ramp4m = Rc::new(Item {
            name: "Inv. Ramp 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_8x4_Inverted_01
        item_registry.push(inv_ramp4m.clone());
        let double_ramp8m = Rc::new(Item {
            name: "Double Ramp 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_8x8x8
        item_registry.push(double_ramp8m.clone());
        let down_corner_ramp1m = Rc::new(Item {
            name: "Down Corner Ramp 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Diagonal_8x1_01
        item_registry.push(down_corner_ramp1m.clone());
        let up_corner_ramp1m = Rc::new(Item {
            name: "Up Corner Ramp 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Diagonal_8x1_02
        item_registry.push(up_corner_ramp1m.clone());
        let down_corner_ramp2m = Rc::new(Item {
            name: "Down Corner Ramp 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Diagonal_8x2_01
        item_registry.push(down_corner_ramp2m.clone());
        let up_corner_ramp2m = Rc::new(Item {
            name: "Up Corner Ramp 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Diagonal_8x2_02
        item_registry.push(up_corner_ramp2m.clone());
        let down_corner_ramp4m = Rc::new(Item {
            name: "Down Corner Ramp 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Diagonal_8x4_01
        item_registry.push(down_corner_ramp4m.clone());
        let up_corner_ramp4m = Rc::new(Item {
            name: "Up Corner Ramp 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Diagonal_8x4_02
        item_registry.push(up_corner_ramp4m.clone());
        let frame_ramp = Rc::new(Item { name: "Frame Ramp" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Frame_01
        item_registry.push(frame_ramp.clone());
        let inverted_frame_ramp = Rc::new(Item {
            name: "Inverted Frame Ramp",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_Ramp_Frame_Inverted_01
        item_registry.push(inverted_frame_ramp.clone());
        let double_ramp4m = Rc::new(Item {
            name: "Double Ramp 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampDouble
        item_registry.push(double_ramp4m.clone());
        let double_ramp2m = Rc::new(Item {
            name: "Double Ramp 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampDouble_8x1
        item_registry.push(double_ramp2m.clone());
        let inv_ramp1m = Rc::new(Item {
            name: "Inv. Ramp 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x1
        item_registry.push(inv_ramp1m.clone());
        let inv_up_corner1m = Rc::new(Item {
            name: "Inv. Up Corner 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x1_Corner_01
        item_registry.push(inv_up_corner1m.clone());
        let inv_down_corner1m = Rc::new(Item {
            name: "Inv. Down Corner 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x1_Corner_02
        item_registry.push(inv_down_corner1m.clone());
        let inv_ramp2m = Rc::new(Item {
            name: "Inv. Ramp 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x2_01
        item_registry.push(inv_ramp2m.clone());
        let inv_up_corner2m = Rc::new(Item {
            name: "Inv. Up Corner 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x2_Corner_01
        item_registry.push(inv_up_corner2m.clone());
        let inv_down_corner2m = Rc::new(Item {
            name: "Inv. Down Corner 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x2_Corner_02
        item_registry.push(inv_down_corner2m.clone());
        let inv_up_corner4m = Rc::new(Item {
            name: "Inv. Up Corner 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x4_Corner_01
        item_registry.push(inv_up_corner4m.clone());
        let inv_down_corner4m = Rc::new(Item {
            name: "Inv. Down Corner 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Ramp/Desc_RampInverted_8x4_Corner_02
        item_registry.push(inv_down_corner4m.clone());
        let roof_flat = Rc::new(Item { name: "Roof Flat" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_A_01
        item_registry.push(roof_flat.clone());
        let roof1m = Rc::new(Item { name: "Roof 1m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_A_02
        item_registry.push(roof1m.clone());
        let roof2m = Rc::new(Item { name: "Roof 2m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_A_03
        item_registry.push(roof2m.clone());
        let roof4m = Rc::new(Item { name: "Roof 4m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_A_04
        item_registry.push(roof4m.clone());
        let inner_corner_roof1m = Rc::new(Item {
            name: "Inner Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Metal_InCorner_01
        item_registry.push(inner_corner_roof1m.clone());
        let inner_corner_roof2m = Rc::new(Item {
            name: "Inner Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Metal_InCorner_02
        item_registry.push(inner_corner_roof2m.clone());
        let inner_corner_roof4m = Rc::new(Item {
            name: "Inner Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Metal_InCorner_03
        item_registry.push(inner_corner_roof4m.clone());
        let outer_corner_roof1m = Rc::new(Item {
            name: "Outer Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Metal_OutCorner_01
        item_registry.push(outer_corner_roof1m.clone());
        let outer_corner_roof2m = Rc::new(Item {
            name: "Outer Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Metal_OutCorner_02
        item_registry.push(outer_corner_roof2m.clone());
        let outer_corner_roof4m = Rc::new(Item {
            name: "Outer Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Metal_OutCorner_03
        item_registry.push(outer_corner_roof4m.clone());
        let roof_flat = Rc::new(Item { name: "Roof Flat" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_01
        item_registry.push(roof_flat.clone());
        let roof1m = Rc::new(Item { name: "Roof 1m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_02
        item_registry.push(roof1m.clone());
        let roof2m = Rc::new(Item { name: "Roof 2m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_03
        item_registry.push(roof2m.clone());
        let roof4m = Rc::new(Item { name: "Roof 4m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_04
        item_registry.push(roof4m.clone());
        let inner_corner_roof1m = Rc::new(Item {
            name: "Inner Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_InCorner_01
        item_registry.push(inner_corner_roof1m.clone());
        let inner_corner_roof2m = Rc::new(Item {
            name: "Inner Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_InCorner_02
        item_registry.push(inner_corner_roof2m.clone());
        let inner_corner_roof4m = Rc::new(Item {
            name: "Inner Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_InCorner_03
        item_registry.push(inner_corner_roof4m.clone());
        let outer_corner_roof1m = Rc::new(Item {
            name: "Outer Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_OutCorner_01
        item_registry.push(outer_corner_roof1m.clone());
        let outer_corner_roof2m = Rc::new(Item {
            name: "Outer Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_OutCorner_02
        item_registry.push(outer_corner_roof2m.clone());
        let outer_corner_roof4m = Rc::new(Item {
            name: "Outer Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Orange_OutCorner_03
        item_registry.push(outer_corner_roof4m.clone());
        let roof_flat = Rc::new(Item { name: "Roof Flat" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_01
        item_registry.push(roof_flat.clone());
        let roof1m = Rc::new(Item { name: "Roof 1m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_02
        item_registry.push(roof1m.clone());
        let roof2m = Rc::new(Item { name: "Roof 2m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_03
        item_registry.push(roof2m.clone());
        let roof4m = Rc::new(Item { name: "Roof 4m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_04
        item_registry.push(roof4m.clone());
        let inner_corner_roof1m = Rc::new(Item {
            name: "Inner Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_InCorner_01
        item_registry.push(inner_corner_roof1m.clone());
        let inner_corner_roof2m = Rc::new(Item {
            name: "Inner Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_InCorner_02
        item_registry.push(inner_corner_roof2m.clone());
        let inner_corner_roof4m = Rc::new(Item {
            name: "Inner Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_InCorner_03
        item_registry.push(inner_corner_roof4m.clone());
        let outer_corner_roof1m = Rc::new(Item {
            name: "Outer Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_OutCorner_01
        item_registry.push(outer_corner_roof1m.clone());
        let outer_corner_roof2m = Rc::new(Item {
            name: "Outer Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_OutCorner_02
        item_registry.push(outer_corner_roof2m.clone());
        let outer_corner_roof4m = Rc::new(Item {
            name: "Outer Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Tar_OutCorner_03
        item_registry.push(outer_corner_roof4m.clone());
        let roof_flat = Rc::new(Item { name: "Roof Flat" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_01
        item_registry.push(roof_flat.clone());
        let roof1m = Rc::new(Item { name: "Roof 1m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_02
        item_registry.push(roof1m.clone());
        let roof2m = Rc::new(Item { name: "Roof 2m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_03
        item_registry.push(roof2m.clone());
        let roof4m = Rc::new(Item { name: "Roof 4m" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_04
        item_registry.push(roof4m.clone());
        let inner_corner_roof1m = Rc::new(Item {
            name: "Inner Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_InCorner_01
        item_registry.push(inner_corner_roof1m.clone());
        let inner_corner_roof2m = Rc::new(Item {
            name: "Inner Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_InCorner_02
        item_registry.push(inner_corner_roof2m.clone());
        let inner_corner_roof4m = Rc::new(Item {
            name: "Inner Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_InCorner_03
        item_registry.push(inner_corner_roof4m.clone());
        let outer_corner_roof1m = Rc::new(Item {
            name: "Outer Corner Roof 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_OutCorner_01
        item_registry.push(outer_corner_roof1m.clone());
        let outer_corner_roof2m = Rc::new(Item {
            name: "Outer Corner Roof 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_OutCorner_02
        item_registry.push(outer_corner_roof2m.clone());
        let outer_corner_roof4m = Rc::new(Item {
            name: "Outer Corner Roof 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Roof/Desc_Roof_Window_OutCorner_03
        item_registry.push(outer_corner_roof4m.clone());
        let stairs_left = Rc::new(Item {
            name: "Stairs Left",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Stair/Desc_Stairs_Left_01
        item_registry.push(stairs_left.clone());
        let stairs_right = Rc::new(Item {
            name: "Stairs Right",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Stair/Desc_Stairs_Right_01
        item_registry.push(stairs_right.clone());
        let space_elevator = Rc::new(Item {
            name: "Space Elevator",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/SpaceElevator/Desc_SpaceElevator
        item_registry.push(space_elevator.clone());
        let walkway_crossing = Rc::new(Item {
            name: "Walkway Crossing",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Walkway/Desc_WalkwayCross
        item_registry.push(walkway_crossing.clone());
        let walkway_ramp = Rc::new(Item {
            name: "Walkway Ramp",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Walkway/Desc_WalkwayRamp
        item_registry.push(walkway_ramp.clone());
        let walkway_straight = Rc::new(Item {
            name: "Walkway Straight",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Walkway/Desc_WalkwayStraight
        item_registry.push(walkway_straight.clone());
        let walkway_t_crossing = Rc::new(Item {
            name: "Walkway T-Crossing",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Walkway/Desc_WalkwayT
        item_registry.push(walkway_t_crossing.clone());
        let walkway_turn = Rc::new(Item {
            name: "Walkway Turn",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Walkway/Desc_WalkwayTurn
        item_registry.push(walkway_turn.clone());
        let basic_wall1m = Rc::new(Item {
            name: "Basic Wall 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_8x1
        item_registry.push(basic_wall1m.clone());
        let tilted_wall4m = Rc::new(Item {
            name: "Tilted Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_8x4
        item_registry.push(tilted_wall4m.clone());
        let inv_ramp_wall1m = Rc::new(Item {
            name: "Inv. Ramp Wall 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_FlipTris_8x1
        item_registry.push(inv_ramp_wall1m.clone());
        let inv_ramp_wall2m = Rc::new(Item {
            name: "Inv. Ramp Wall 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_FlipTris_8x2
        item_registry.push(inv_ramp_wall2m.clone());
        let inv_ramp_wall4m = Rc::new(Item {
            name: "Inv. Ramp Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_FlipTris_8x4
        item_registry.push(inv_ramp_wall4m.clone());
        let inv_ramp_wall8m = Rc::new(Item {
            name: "Inv. Ramp Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_FlipTris_8x8
        item_registry.push(inv_ramp_wall8m.clone());
        let ramp_wall1m = Rc::new(Item {
            name: "Ramp Wall 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_Tris_8x1
        item_registry.push(ramp_wall1m.clone());
        let ramp_wall2m = Rc::new(Item {
            name: "Ramp Wall 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_Tris_8x2
        item_registry.push(ramp_wall2m.clone());
        let ramp_wall4m = Rc::new(Item {
            name: "Ramp Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_Tris_8x4
        item_registry.push(ramp_wall4m.clone());
        let ramp_wall8m = Rc::new(Item {
            name: "Ramp Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_SteelWall_Tris_8x8
        item_registry.push(ramp_wall8m.clone());
        let basic_wall4m = Rc::new(Item {
            name: "Basic Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_8x4_01
        item_registry.push(basic_wall4m.clone());
        let basic_wall4m = Rc::new(Item {
            name: "Basic Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_8x4_02
        item_registry.push(basic_wall4m.clone());
        let conveyor_wallx3 = Rc::new(Item {
            name: "Conveyor Wall x3",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_01
        item_registry.push(conveyor_wallx3.clone());
        let conveyor_wallx3 = Rc::new(Item {
            name: "Conveyor Wall x3",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_01_Steel
        item_registry.push(conveyor_wallx3.clone());
        let conveyor_wallx2 = Rc::new(Item {
            name: "Conveyor Wall x2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_02
        item_registry.push(conveyor_wallx2.clone());
        let conveyor_wallx2 = Rc::new(Item {
            name: "Conveyor Wall x2",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_02_Steel
        item_registry.push(conveyor_wallx2.clone());
        let conveyor_wallx1 = Rc::new(Item {
            name: "Conveyor Wall x1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_03
        item_registry.push(conveyor_wallx1.clone());
        let conveyor_wallx1 = Rc::new(Item {
            name: "Conveyor Wall x1",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_03_Steel
        item_registry.push(conveyor_wallx1.clone());
        let wall_conveyor_perpendicular = Rc::new(Item {
            name: "Wall Conveyor Perpendicular",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_04
        item_registry.push(wall_conveyor_perpendicular.clone());
        let wall_conveyor_perpendicular = Rc::new(Item {
            name: "Wall Conveyor Perpendicular",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Conveyor_8x4_04_Steel
        item_registry.push(wall_conveyor_perpendicular.clone());
        let center_door_wall = Rc::new(Item {
            name: "Center Door Wall",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Door_8x4_01
        item_registry.push(center_door_wall.clone());
        let center_door_wall = Rc::new(Item {
            name: "Center Door Wall",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Door_8x4_01_Steel
        item_registry.push(center_door_wall.clone());
        let left_door_wall = Rc::new(Item {
            name: "Left Door Wall",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Door_8x4_02_Steel
        item_registry.push(left_door_wall.clone());
        let side_door_wall = Rc::new(Item {
            name: "Side Door Wall",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Door_8x4_03
        item_registry.push(side_door_wall.clone());
        let side_door_wall = Rc::new(Item {
            name: "Side Door Wall",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Door_8x4_03_Steel
        item_registry.push(side_door_wall.clone());
        let frame_wall = Rc::new(Item { name: "Frame Wall" }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Frame_01
        item_registry.push(frame_wall.clone());
        let gate_hole_wall = Rc::new(Item {
            name: "Gate Hole Wall",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Gate_8x4_01
        item_registry.push(gate_hole_wall.clone());
        let basic_wall1m = Rc::new(Item {
            name: "Basic Wall 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x1
        item_registry.push(basic_wall1m.clone());
        let tilted_corner_wall4m = Rc::new(Item {
            name: "Tilted Corner Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x4_Corner_01
        item_registry.push(tilted_corner_wall4m.clone());
        let tilted_concave_wall4m = Rc::new(Item {
            name: "Tilted Concave Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x4_Corner_02
        item_registry.push(tilted_concave_wall4m.clone());
        let tilted_corner_wall8m = Rc::new(Item {
            name: "Tilted Corner Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x8_Corner_01
        item_registry.push(tilted_corner_wall8m.clone());
        let tilted_concave_wall8m = Rc::new(Item {
            name: "Tilted Concave Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x8_Corner_02
        item_registry.push(tilted_concave_wall8m.clone());
        let tilted_wall4m = Rc::new(Item {
            name: "Tilted Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_Angular_8x4
        item_registry.push(tilted_wall4m.clone());
        let tilted_wall8m = Rc::new(Item {
            name: "Tilted Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_Angular_8x8
        item_registry.push(tilted_wall8m.clone());
        let inv_ramp_wall1m = Rc::new(Item {
            name: "Inv. Ramp Wall 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x1_FlipTris
        item_registry.push(inv_ramp_wall1m.clone());
        let inv_ramp_wall2m = Rc::new(Item {
            name: "Inv. Ramp Wall 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x2_FlipTris
        item_registry.push(inv_ramp_wall2m.clone());
        let inv_ramp_wall4m = Rc::new(Item {
            name: "Inv. Ramp Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x4_FlipTris
        item_registry.push(inv_ramp_wall4m.clone());
        let inv_ramp_wall8m = Rc::new(Item {
            name: "Inv. Ramp Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x8_FlipTris
        item_registry.push(inv_ramp_wall8m.clone());
        let ramp_wall1m = Rc::new(Item {
            name: "Ramp Wall 1m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x1_Tris
        item_registry.push(ramp_wall1m.clone());
        let ramp_wall2m = Rc::new(Item {
            name: "Ramp Wall 2m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x2_Tris
        item_registry.push(ramp_wall2m.clone());
        let ramp_wall4m = Rc::new(Item {
            name: "Ramp Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x4_Tris
        item_registry.push(ramp_wall4m.clone());
        let ramp_wall8m = Rc::new(Item {
            name: "Ramp Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/FicsitWallSet/Desc_Wall_Orange_8x8_Tris
        item_registry.push(ramp_wall8m.clone());
        let single_window = Rc::new(Item {
            name: "Single Window",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Window_8x4_01
        item_registry.push(single_window.clone());
        let frame_window = Rc::new(Item {
            name: "Frame Window",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Window_8x4_02
        item_registry.push(frame_window.clone());
        let panel_window = Rc::new(Item {
            name: "Panel Window",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Window_8x4_03
        item_registry.push(panel_window.clone());
        let panel_window = Rc::new(Item {
            name: "Panel Window",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Window_8x4_03_Steel
        item_registry.push(panel_window.clone());
        let reinforced_window = Rc::new(Item {
            name: "Reinforced Window",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Window_8x4_04
        item_registry.push(reinforced_window.clone());
        let full_frame_window = Rc::new(Item {
            name: "Full Frame Window",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Window_Thin_8x4_01
        item_registry.push(full_frame_window.clone());
        let hex_frame_window = Rc::new(Item {
            name: "Hex Frame Window",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/Desc_Wall_Window_Thin_8x4_02
        item_registry.push(hex_frame_window.clone());
        let tilted_wall4m = Rc::new(Item {
            name: "Tilted Wall 4m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_WallSet_Steel_Angular_8x4
        item_registry.push(tilted_wall4m.clone());
        let tilted_wall8m = Rc::new(Item {
            name: "Tilted Wall 8m",
        }); // FactoryGame/Content/FactoryGame/Buildable/Building/Wall/SteelWallSet/Desc_WallSet_Steel_Angular_8x8
        item_registry.push(tilted_wall8m.clone());
        let alien_dnacapsule = Rc::new(Item {
            name: "Alien DNA Capsule",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AlienDNACapsule/Desc_AlienDNACapsule
        item_registry.push(alien_dnacapsule.clone());
        let alien_protein = Rc::new(Item {
            name: "Alien Protein",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AlienProtein/Desc_AlienProtein
        item_registry.push(alien_protein.clone());
        let solid_biofuel = Rc::new(Item {
            name: "Solid Biofuel",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/BioFuel/Desc_Biofuel
        item_registry.push(solid_biofuel.clone());
        let leaves = Rc::new(Item { name: "Leaves" }); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Leaves
        item_registry.push(leaves.clone());
        let mycelia = Rc::new(Item { name: "Mycelia" }); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_Mycelia
        item_registry.push(mycelia.clone());
        let copper_powder = Rc::new(Item {
            name: "Copper Powder",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/CopperDust/Desc_CopperDust
        item_registry.push(copper_powder.clone());
        let empty_fluid_tank = Rc::new(Item {
            name: "Empty Fluid Tank",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/GasTank/Desc_GasTank
        item_registry.push(empty_fluid_tank.clone());
        let power_shard = Rc::new(Item {
            name: "Power Shard",
        }); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_CrystalShard
        item_registry.push(power_shard.clone());
        let blue_power_slug = Rc::new(Item {
            name: "Blue Power Slug",
        }); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_Crystal
        item_registry.push(blue_power_slug.clone());
        let yellow_power_slug = Rc::new(Item {
            name: "Yellow Power Slug",
        }); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_Crystal_mk2
        item_registry.push(yellow_power_slug.clone());
        let purple_power_slug = Rc::new(Item {
            name: "Purple Power Slug",
        }); // FactoryGame/Content/FactoryGame/Resource/Environment/Crystal/Desc_Crystal_mk3
        item_registry.push(purple_power_slug.clone());
        let hatcher_remains = Rc::new(Item {
            name: "Hatcher Remains",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_HatcherParts
        item_registry.push(hatcher_remains.clone());
        let hog_remains = Rc::new(Item {
            name: "Hog Remains",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_HogParts
        item_registry.push(hog_remains.clone());
        let plasma_spitter_remains = Rc::new(Item {
            name: "Plasma Spitter Remains",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_SpitterParts
        item_registry.push(plasma_spitter_remains.clone());
        let stinger_remains = Rc::new(Item {
            name: "Stinger Remains",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/AnimalParts/Desc_StingerParts
        item_registry.push(stinger_remains.clone());
        let blade_runners = Rc::new(Item {
            name: "Blade Runners",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/JumpingStilts/BP_EquipmentDescriptorJumpingStilts
        item_registry.push(blade_runners.clone());
        let candy_cane_basher = Rc::new(Item {
            name: "Candy Cane Basher",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/StunSpear/BP_EquipmentDescriptorCandyCane
        item_registry.push(candy_cane_basher.clone());
        let xeno_zapper = Rc::new(Item {
            name: "Xeno-Zapper",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/ShockShank/BP_EquipmentDescriptorShockShank
        item_registry.push(xeno_zapper.clone());
        let candy_cane = Rc::new(Item { name: "Candy Cane" }); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_CandyCane
        item_registry.push(candy_cane.clone());
        let ficsmasgift = Rc::new(Item {
            name: "FICSMAS Gift",
        }); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_Gift
        item_registry.push(ficsmasgift.clone());
        let rifle_ammo = Rc::new(Item { name: "Rifle Ammo" }); // FactoryGame/Content/FactoryGame/Resource/Parts/CartridgeStandard/Desc_CartridgeStandard
        item_registry.push(rifle_ammo.clone());
        let smokeless_powder = Rc::new(Item {
            name: "Smokeless Powder",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/GunPowder/Desc_GunpowderMK2
        item_registry.push(smokeless_powder.clone());
        let chainsaw = Rc::new(Item { name: "Chainsaw" }); // FactoryGame/Content/FactoryGame/Equipment/Chainsaw/Desc_Chainsaw
        item_registry.push(chainsaw.clone());
        let color_cartridge = Rc::new(Item {
            name: "Color Cartridge",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/ColorCartridge/Desc_ColorCartridge
        item_registry.push(color_cartridge.clone());
        let flower_petals = Rc::new(Item {
            name: "Flower Petals",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/GenericBiomass/Desc_FlowerPetals
        item_registry.push(flower_petals.clone());
        let gas_filter = Rc::new(Item { name: "Gas Filter" }); // FactoryGame/Content/FactoryGame/Resource/Parts/Filter/Desc_Filter
        item_registry.push(gas_filter.clone());
        let iodine_infused_filter = Rc::new(Item {
            name: "Iodine Infused Filter",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/IodineInfusedFilter/Desc_HazmatFilter
        item_registry.push(iodine_infused_filter.clone());
        let gas_mask = Rc::new(Item { name: "Gas Mask" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/GasMask/BP_EquipmentDescriptorGasmask
        item_registry.push(gas_mask.clone());
        let hazmat_suit = Rc::new(Item {
            name: "Hazmat Suit",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/HazmatSuit/BP_EquipmentDescriptorHazmatSuit
        item_registry.push(hazmat_suit.clone());
        let hover_pack = Rc::new(Item { name: "Hover Pack" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/HoverPack/BP_EquipmentDescriptorHoverPack
        item_registry.push(hover_pack.clone());
        let jetpack = Rc::new(Item { name: "Jetpack" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/JetPack/BP_EquipmentDescriptorJetPack
        item_registry.push(jetpack.clone());
        let medicinal_inhaler = Rc::new(Item {
            name: "Medicinal Inhaler",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/Medkit/Desc_Medkit
        item_registry.push(medicinal_inhaler.clone());
        let paleberry = Rc::new(Item { name: "Paleberry" }); // FactoryGame/Content/FactoryGame/Resource/Environment/Berry/Desc_Berry
        item_registry.push(paleberry.clone());
        let beryl_nut = Rc::new(Item { name: "Beryl Nut" }); // FactoryGame/Content/FactoryGame/Resource/Environment/Nut/Desc_Nut
        item_registry.push(beryl_nut.clone());
        let nobelisk = Rc::new(Item { name: "Nobelisk" }); // FactoryGame/Content/FactoryGame/Resource/Parts/NobeliskExplosive/Desc_NobeliskExplosive
        item_registry.push(nobelisk.clone());
        let nobelisk_detonator = Rc::new(Item {
            name: "Nobelisk Detonator",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/NobeliskDetonator/BP_EquipmentDescriptorNobeliskDetonator
        item_registry.push(nobelisk_detonator.clone());
        let object_scanner = Rc::new(Item {
            name: "Object Scanner",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/GemstoneScanner/BP_EquipmentDescriptorObjectScanner
        item_registry.push(object_scanner.clone());
        let bacon_agaric = Rc::new(Item {
            name: "Bacon Agaric",
        }); // FactoryGame/Content/FactoryGame/Resource/Environment/DesertShroom/Desc_Shroom
        item_registry.push(bacon_agaric.clone());
        let parachute = Rc::new(Item { name: "Parachute" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/Beacon/Desc_Parachute
        item_registry.push(parachute.clone());
        let rebar_gun = Rc::new(Item { name: "Rebar Gun" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/NailGun/Desc_RebarGunProjectile
        item_registry.push(rebar_gun.clone());
        let rifle = Rc::new(Item { name: "Rifle" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/Rifle/BP_EquipmentDescriptorRifle
        item_registry.push(rifle.clone());
        let iron_rebar = Rc::new(Item { name: "Iron Rebar" }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpikedRebar/Desc_SpikedRebar
        item_registry.push(iron_rebar.clone());
        let xeno_basher = Rc::new(Item {
            name: "Xeno-Basher",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/StunSpear/BP_EquipmentDescriptorStunSpear
        item_registry.push(xeno_basher.clone());
        let zipline = Rc::new(Item { name: "Zipline" }); // FactoryGame/Content/FactoryGame/Resource/Equipment/Zipline/BP_EqDescZipLine
        item_registry.push(zipline.clone());
        let snowball = Rc::new(Item { name: "Snowball" }); // FactoryGame/Content/FactoryGame/Resource/Parts/SnowballProjectile/Desc_SnowballProjectile
        item_registry.push(snowball.clone());
        let actual_snow = Rc::new(Item {
            name: "Actual Snow",
        }); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_Snow
        item_registry.push(actual_snow.clone());
        let ficsmasgift_tree = Rc::new(Item {
            name: "FICSMAS Gift Tree",
        }); // FactoryGame/Content/FactoryGame/Buildable/Factory/Holiday/Build_TreeGiftProducer/Desc_TreeGiftProducer
        item_registry.push(ficsmasgift_tree.clone());
        let ficsmastree_branch = Rc::new(Item {
            name: "FICSMAS Tree Branch",
        }); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_XmasBranch
        item_registry.push(ficsmastree_branch.clone());
        let copper_ficsmasornament = Rc::new(Item {
            name: "Copper FICSMAS Ornament",
        }); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_XmasBall3
        item_registry.push(copper_ficsmasornament.clone());
        let iron_ficsmasornament = Rc::new(Item {
            name: "Iron FICSMAS Ornament",
        }); // FactoryGame/Content/FactoryGame/Events/Christmas/Parts/Desc_XmasBall4
        item_registry.push(iron_ficsmasornament.clone());
        let liquid_biofuel = Rc::new(Item {
            name: "Liquid Biofuel",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/BioFuel/Desc_LiquidBiofuel
        item_registry.push(liquid_biofuel.clone());
        let packaged_liquid_biofuel = Rc::new(Item {
            name: "Packaged Liquid Biofuel",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/BioFuel/Desc_PackagedBiofuel
        item_registry.push(packaged_liquid_biofuel.clone());
        let packaged_oil = Rc::new(Item {
            name: "Packaged Oil",
        }); // FactoryGame/Content/FactoryGame/Resource/RawResources/CrudeOil/Desc_PackagedOil
        item_registry.push(packaged_oil.clone());
        let packaged_heavy_oil_residue = Rc::new(Item {
            name: "Packaged Heavy Oil Residue",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/HeavyOilResidue/Desc_PackagedOilResidue
        item_registry.push(packaged_heavy_oil_residue.clone());
        let packaged_turbofuel = Rc::new(Item {
            name: "Packaged Turbofuel",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/Turbofuel/Desc_TurboFuel
        item_registry.push(packaged_turbofuel.clone());
        let packaged_alumina_solution = Rc::new(Item {
            name: "Packaged Alumina Solution",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/Alumina/Desc_PackagedAlumina
        item_registry.push(packaged_alumina_solution.clone());
        let packaged_nitric_acid = Rc::new(Item {
            name: "Packaged Nitric Acid",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/NitricAcid/Desc_PackagedNitricAcid
        item_registry.push(packaged_nitric_acid.clone());
        let packaged_sulfuric_acid = Rc::new(Item {
            name: "Packaged Sulfuric Acid",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SulfuricAcid/Desc_PackagedSulfuricAcid
        item_registry.push(packaged_sulfuric_acid.clone());
        let modular_engine = Rc::new(Item {
            name: "Modular Engine",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_4
        item_registry.push(modular_engine.clone());
        let adaptive_control_unit = Rc::new(Item {
            name: "Adaptive Control Unit",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_5
        item_registry.push(adaptive_control_unit.clone());
        let magnetic_field_generator = Rc::new(Item {
            name: "Magnetic Field Generator",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_6
        item_registry.push(magnetic_field_generator.clone());
        let assembly_director_system = Rc::new(Item {
            name: "Assembly Director System",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_7
        item_registry.push(assembly_director_system.clone());
        let thermal_propulsion_rocket = Rc::new(Item {
            name: "Thermal Propulsion Rocket",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_8
        item_registry.push(thermal_propulsion_rocket.clone());
        let nuclear_pasta = Rc::new(Item {
            name: "Nuclear Pasta",
        }); // FactoryGame/Content/FactoryGame/Resource/Parts/SpaceElevatorParts/Desc_SpaceElevatorPart_9
        item_registry.push(nuclear_pasta.clone());
        let factory_cartT = Rc::new(Item {
            name: "Factory CartT",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/GolfCart/Desc_GolfCart
        item_registry.push(factory_cartT.clone());
        let golden_factory_cartT = Rc::new(Item {
            name: "Golden Factory CartT",
        }); // FactoryGame/Content/FactoryGame/Resource/Equipment/GolfCart/Desc_GolfCartGold
        item_registry.push(golden_factory_cartT.clone());

        let alternate_adhered_iron_plate_recipe = Recipe {
            name: "Alternate: Adhered Iron Plate",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: reinforced_iron_plate.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Reinforced Iron Plate")
            .or_insert_with(Vec::new)
            .push(alternate_adhered_iron_plate_recipe);
        let alternate_bolted_frame_recipe = Recipe {
            name: "Alternate: Bolted Frame",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: reinforced_iron_plate.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 56,
                },
            ],
            output: ItemAmount {
                item: modular_frame.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Modular Frame")
            .or_insert_with(Vec::new)
            .push(alternate_bolted_frame_recipe);
        let alternate_coated_cable_recipe = Recipe {
            name: "Alternate: Coated Cable",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: wire.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: heavy_oil_residue.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: cable.clone(),
                amount: 9,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Cable")
            .or_insert_with(Vec::new)
            .push(alternate_coated_cable_recipe);
        let alternate_coated_iron_canister_recipe = Recipe {
            name: "Alternate: Coated Iron Canister",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: copper_sheet.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: empty_canister.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: true,
        };
        recipe_registry
            .entry("Empty Canister")
            .or_insert_with(Vec::new)
            .push(alternate_coated_iron_canister_recipe);
        let alternate_coated_iron_plate_recipe = Recipe {
            name: "Alternate: Coated Iron Plate",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: iron_ingot.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: plastic.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: iron_plate.clone(),
                amount: 15,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Iron Plate")
            .or_insert_with(Vec::new)
            .push(alternate_coated_iron_plate_recipe);
        let alternate_coke_steel_ingot_recipe = Recipe {
            name: "Alternate: Coke Steel Ingot",
            machine: Machine::Foundry,
            input: vec![
                ItemAmount {
                    item: iron_ore.clone(),
                    amount: 15,
                },
                ItemAmount {
                    item: petroleum_coke.clone(),
                    amount: 15,
                },
            ],
            output: ItemAmount {
                item: steel_ingot.clone(),
                amount: 20,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Steel Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_coke_steel_ingot_recipe);
        let alternate_copper_alloy_ingot_recipe = Recipe {
            name: "Alternate: Copper Alloy Ingot",
            machine: Machine::Foundry,
            input: vec![
                ItemAmount {
                    item: copper_ore.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: iron_ore.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: copper_ingot.clone(),
                amount: 20,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Copper Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_copper_alloy_ingot_recipe);
        let alternate_copper_rotor_recipe = Recipe {
            name: "Alternate: Copper Rotor",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: copper_sheet.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 52,
                },
            ],
            output: ItemAmount {
                item: rotor.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Rotor")
            .or_insert_with(Vec::new)
            .push(alternate_copper_rotor_recipe);
        let alternate_diluted_packaged_fuel_recipe = Recipe {
            name: "Alternate: Diluted Packaged Fuel",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: heavy_oil_residue.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: packaged_water.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_fuel.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: true,
        };
        recipe_registry
            .entry("Packaged Fuel")
            .or_insert_with(Vec::new)
            .push(alternate_diluted_packaged_fuel_recipe);
        let alternate_electrode_aluminum_scrap_recipe = Recipe {
            name: "Alternate: Electrode - Aluminum Scrap",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: alumina_solution.clone(),
                    amount: 12,
                },
                ItemAmount {
                    item: petroleum_coke.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: aluminum_scrap.clone(),
                amount: 20,
            },
            byproduct: Some(ItemAmount {
                item: water.clone(),
                amount: 7,
            }),
            manufacturing_duration: 4f32,
            alternate: true,
        };
        recipe_registry
            .entry("Aluminum Scrap")
            .or_insert_with(Vec::new)
            .push(alternate_electrode_aluminum_scrap_recipe);
        let alternate_electrode_circuit_board_recipe = Recipe {
            name: "Alternate: Electrode Circuit Board",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: rubber.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: petroleum_coke.clone(),
                    amount: 9,
                },
            ],
            output: ItemAmount {
                item: circuit_board.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Circuit Board")
            .or_insert_with(Vec::new)
            .push(alternate_electrode_circuit_board_recipe);
        let alternate_flexible_framework_recipe = Recipe {
            name: "Alternate: Flexible Framework",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: modular_frame.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: steel_beam.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 8,
                },
            ],
            output: ItemAmount {
                item: versatile_framework.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Versatile Framework")
            .or_insert_with(Vec::new)
            .push(alternate_flexible_framework_recipe);
        let alternate_fused_wire_recipe = Recipe {
            name: "Alternate: Fused Wire",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: copper_ingot.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: caterium_ingot.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: wire.clone(),
                amount: 30,
            },
            byproduct: None,
            manufacturing_duration: 20f32,
            alternate: true,
        };
        recipe_registry
            .entry("Wire")
            .or_insert_with(Vec::new)
            .push(alternate_fused_wire_recipe);
        let alternate_heavy_flexible_frame_recipe = Recipe {
            name: "Alternate: Heavy Flexible Frame",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: modular_frame.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: encased_industrial_beam.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 20,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 104,
                },
            ],
            output: ItemAmount {
                item: heavy_modular_frame.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Heavy Modular Frame")
            .or_insert_with(Vec::new)
            .push(alternate_heavy_flexible_frame_recipe);
        let alternate_heavy_oil_residue_recipe = Recipe {
            name: "Alternate: Heavy Oil Residue",
            machine: Machine::Refinery,
            input: vec![ItemAmount {
                item: crude_oil.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: heavy_oil_residue.clone(),
                amount: 4,
            },
            byproduct: Some(ItemAmount {
                item: polymer_resin.clone(),
                amount: 2,
            }),
            manufacturing_duration: 6f32,
            alternate: true,
        };
        recipe_registry
            .entry("Heavy Oil Residue")
            .or_insert_with(Vec::new)
            .push(alternate_heavy_oil_residue_recipe);
        let alternate_automated_speed_wiring_recipe = Recipe {
            name: "Alternate: Automated Speed Wiring",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: stator.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: wire.clone(),
                    amount: 40,
                },
                ItemAmount {
                    item: high_speed_connector.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: automated_wiring.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 32f32,
            alternate: true,
        };
        recipe_registry
            .entry("Automated Wiring")
            .or_insert_with(Vec::new)
            .push(alternate_automated_speed_wiring_recipe);
        let alternate_plastic_smart_plating_recipe = Recipe {
            name: "Alternate: Plastic Smart Plating",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: reinforced_iron_plate.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: rotor.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: plastic.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: smart_plating.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Smart Plating")
            .or_insert_with(Vec::new)
            .push(alternate_plastic_smart_plating_recipe);
        let alternate_polyester_fabric_recipe = Recipe {
            name: "Alternate: Polyester Fabric",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: polymer_resin.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: fabric.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: true,
        };
        recipe_registry
            .entry("Fabric")
            .or_insert_with(Vec::new)
            .push(alternate_polyester_fabric_recipe);
        let alternate_polymer_resin_recipe = Recipe {
            name: "Alternate: Polymer Resin",
            machine: Machine::Refinery,
            input: vec![ItemAmount {
                item: crude_oil.clone(),
                amount: 6,
            }],
            output: ItemAmount {
                item: polymer_resin.clone(),
                amount: 13,
            },
            byproduct: Some(ItemAmount {
                item: heavy_oil_residue.clone(),
                amount: 2,
            }),
            manufacturing_duration: 6f32,
            alternate: true,
        };
        recipe_registry
            .entry("Polymer Resin")
            .or_insert_with(Vec::new)
            .push(alternate_polymer_resin_recipe);
        let alternate_pure_caterium_ingot_recipe = Recipe {
            name: "Alternate: Pure Caterium Ingot",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: caterium_ore.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: caterium_ingot.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 5f32,
            alternate: true,
        };
        recipe_registry
            .entry("Caterium Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_pure_caterium_ingot_recipe);
        let alternate_pure_copper_ingot_recipe = Recipe {
            name: "Alternate: Pure Copper Ingot",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: copper_ore.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: copper_ingot.clone(),
                amount: 15,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Copper Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_pure_copper_ingot_recipe);
        let alternate_pure_iron_ingot_recipe = Recipe {
            name: "Alternate: Pure Iron Ingot",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: iron_ore.clone(),
                    amount: 7,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: iron_ingot.clone(),
                amount: 13,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Iron Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_pure_iron_ingot_recipe);
        let alternate_pure_quartz_crystal_recipe = Recipe {
            name: "Alternate: Pure Quartz Crystal",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: raw_quartz.clone(),
                    amount: 9,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: quartz_crystal.clone(),
                amount: 7,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Quartz Crystal")
            .or_insert_with(Vec::new)
            .push(alternate_pure_quartz_crystal_recipe);
        let alternate_recycled_rubber_recipe = Recipe {
            name: "Alternate: Recycled Rubber",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: plastic.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: fuel.clone(),
                    amount: 6,
                },
            ],
            output: ItemAmount {
                item: rubber.clone(),
                amount: 12,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Rubber")
            .or_insert_with(Vec::new)
            .push(alternate_recycled_rubber_recipe);
        let alternate_rubber_concrete_recipe = Recipe {
            name: "Alternate: Rubber Concrete",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: limestone.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: concrete.clone(),
                amount: 9,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Concrete")
            .or_insert_with(Vec::new)
            .push(alternate_rubber_concrete_recipe);
        let alternate_steamed_copper_sheet_recipe = Recipe {
            name: "Alternate: Steamed Copper Sheet",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: copper_ingot.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: copper_sheet.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Copper Sheet")
            .or_insert_with(Vec::new)
            .push(alternate_steamed_copper_sheet_recipe);
        let alternate_steel_canister_recipe = Recipe {
            name: "Alternate: Steel Canister",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: steel_ingot.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: true,
        };
        recipe_registry
            .entry("Empty Canister")
            .or_insert_with(Vec::new)
            .push(alternate_steel_canister_recipe);
        let alternate_steel_coated_plate_recipe = Recipe {
            name: "Alternate: Steel Coated Plate",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: steel_ingot.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: plastic.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: iron_plate.clone(),
                amount: 18,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Iron Plate")
            .or_insert_with(Vec::new)
            .push(alternate_steel_coated_plate_recipe);
        let alternate_steel_rod_recipe = Recipe {
            name: "Alternate: Steel Rod",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: steel_ingot.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: iron_rod.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 5f32,
            alternate: true,
        };
        recipe_registry
            .entry("Iron Rod")
            .or_insert_with(Vec::new)
            .push(alternate_steel_rod_recipe);
        let alternate_turbo_heavy_fuel_recipe = Recipe {
            name: "Alternate: Turbo Heavy Fuel",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: heavy_oil_residue.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: compacted_coal.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: turbofuel.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Turbofuel")
            .or_insert_with(Vec::new)
            .push(alternate_turbo_heavy_fuel_recipe);
        let alternate_wet_concrete_recipe = Recipe {
            name: "Alternate: Wet Concrete",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: limestone.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: concrete.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: true,
        };
        recipe_registry
            .entry("Concrete")
            .or_insert_with(Vec::new)
            .push(alternate_wet_concrete_recipe);
        let alternate_pure_aluminum_ingot_recipe = Recipe {
            name: "Alternate: Pure Aluminum Ingot",
            machine: Machine::Smelter,
            input: vec![ItemAmount {
                item: aluminum_scrap.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: aluminum_ingot.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Aluminum Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_pure_aluminum_ingot_recipe);
        let alternate_alclad_casing_recipe = Recipe {
            name: "Alternate: Alclad Casing",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: aluminum_ingot.clone(),
                    amount: 20,
                },
                ItemAmount {
                    item: copper_ingot.clone(),
                    amount: 10,
                },
            ],
            output: ItemAmount {
                item: aluminum_casing.clone(),
                amount: 15,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Aluminum Casing")
            .or_insert_with(Vec::new)
            .push(alternate_alclad_casing_recipe);
        let alternate_automated_miner_recipe = Recipe {
            name: "Alternate: Automated Miner",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: motor.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: iron_rod.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: portable_miner.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 60f32,
            alternate: true,
        };
        recipe_registry
            .entry("Portable Miner")
            .or_insert_with(Vec::new)
            .push(alternate_automated_miner_recipe);
        let alternate_classic_battery_recipe = Recipe {
            name: "Alternate: Classic Battery",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: sulfur.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: alclad_aluminum_sheet.clone(),
                    amount: 7,
                },
                ItemAmount {
                    item: plastic.clone(),
                    amount: 8,
                },
                ItemAmount {
                    item: wire.clone(),
                    amount: 12,
                },
            ],
            output: ItemAmount {
                item: battery.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Battery")
            .or_insert_with(Vec::new)
            .push(alternate_classic_battery_recipe);
        let alternate_cooling_device_recipe = Recipe {
            name: "Alternate: Cooling Device",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: heat_sink.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: motor.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: nitrogen_gas.clone(),
                    amount: 24,
                },
            ],
            output: ItemAmount {
                item: cooling_system.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 32f32,
            alternate: true,
        };
        recipe_registry
            .entry("Cooling System")
            .or_insert_with(Vec::new)
            .push(alternate_cooling_device_recipe);
        let alternate_diluted_fuel_recipe = Recipe {
            name: "Alternate: Diluted Fuel",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: heavy_oil_residue.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 10,
                },
            ],
            output: ItemAmount {
                item: fuel.clone(),
                amount: 10,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: true,
        };
        recipe_registry
            .entry("Fuel")
            .or_insert_with(Vec::new)
            .push(alternate_diluted_fuel_recipe);
        let alternate_electric_motor_recipe = Recipe {
            name: "Alternate: Electric Motor",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: electromagnetic_control_rod.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: rotor.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: motor.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Motor")
            .or_insert_with(Vec::new)
            .push(alternate_electric_motor_recipe);
        let alternate_fertile_uranium_recipe = Recipe {
            name: "Alternate: Fertile Uranium",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: uranium.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: uranium_waste.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: nitric_acid.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: sulfuric_acid.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: non_fissile_uranium.clone(),
                amount: 20,
            },
            byproduct: Some(ItemAmount {
                item: water.clone(),
                amount: 8,
            }),
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Non-fissile Uranium")
            .or_insert_with(Vec::new)
            .push(alternate_fertile_uranium_recipe);
        let alternate_heat_fused_frame_recipe = Recipe {
            name: "Alternate: Heat-Fused Frame",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: heavy_modular_frame.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: aluminum_ingot.clone(),
                    amount: 50,
                },
                ItemAmount {
                    item: nitric_acid.clone(),
                    amount: 8,
                },
                ItemAmount {
                    item: fuel.clone(),
                    amount: 10,
                },
            ],
            output: ItemAmount {
                item: fused_modular_frame.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 20f32,
            alternate: true,
        };
        recipe_registry
            .entry("Fused Modular Frame")
            .or_insert_with(Vec::new)
            .push(alternate_heat_fused_frame_recipe);
        let alternate_instant_scrap_recipe = Recipe {
            name: "Alternate: Instant Scrap",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: bauxite.clone(),
                    amount: 15,
                },
                ItemAmount {
                    item: coal.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: sulfuric_acid.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 6,
                },
            ],
            output: ItemAmount {
                item: aluminum_scrap.clone(),
                amount: 30,
            },
            byproduct: Some(ItemAmount {
                item: water.clone(),
                amount: 5,
            }),
            manufacturing_duration: 6f32,
            alternate: true,
        };
        recipe_registry
            .entry("Aluminum Scrap")
            .or_insert_with(Vec::new)
            .push(alternate_instant_scrap_recipe);
        let alternate_ocsupercomputer_recipe = Recipe {
            name: "Alternate: OC Supercomputer",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: radio_control_unit.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: cooling_system.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: supercomputer.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 20f32,
            alternate: true,
        };
        recipe_registry
            .entry("Supercomputer")
            .or_insert_with(Vec::new)
            .push(alternate_ocsupercomputer_recipe);
        let alternate_plutonium_fuel_unit_recipe = Recipe {
            name: "Alternate: Plutonium Fuel Unit",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: encased_plutonium_cell.clone(),
                    amount: 20,
                },
                ItemAmount {
                    item: pressure_conversion_cube.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: plutonium_fuel_rod.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 120f32,
            alternate: true,
        };
        recipe_registry
            .entry("Plutonium Fuel Rod")
            .or_insert_with(Vec::new)
            .push(alternate_plutonium_fuel_unit_recipe);
        let alternate_radio_control_system_recipe = Recipe {
            name: "Alternate: Radio Control System",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: crystal_oscillator.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: circuit_board.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: aluminum_casing.clone(),
                    amount: 60,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 30,
                },
            ],
            output: ItemAmount {
                item: radio_control_unit.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 40f32,
            alternate: true,
        };
        recipe_registry
            .entry("Radio Control Unit")
            .or_insert_with(Vec::new)
            .push(alternate_radio_control_system_recipe);
        let alternate_sloppy_alumina_recipe = Recipe {
            name: "Alternate: Sloppy Alumina",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: bauxite.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 10,
                },
            ],
            output: ItemAmount {
                item: alumina_solution.clone(),
                amount: 12,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: true,
        };
        recipe_registry
            .entry("Alumina Solution")
            .or_insert_with(Vec::new)
            .push(alternate_sloppy_alumina_recipe);
        let alternate_super_state_computer_recipe = Recipe {
            name: "Alternate: Super-State Computer",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: computer.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: electromagnetic_control_rod.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: battery.clone(),
                    amount: 20,
                },
                ItemAmount {
                    item: wire.clone(),
                    amount: 45,
                },
            ],
            output: ItemAmount {
                item: supercomputer.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 50f32,
            alternate: true,
        };
        recipe_registry
            .entry("Supercomputer")
            .or_insert_with(Vec::new)
            .push(alternate_super_state_computer_recipe);
        let alternate_turbo_blend_fuel_recipe = Recipe {
            name: "Alternate: Turbo Blend Fuel",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: fuel.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: heavy_oil_residue.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: sulfur.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: petroleum_coke.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: turbofuel.clone(),
                amount: 6,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Turbofuel")
            .or_insert_with(Vec::new)
            .push(alternate_turbo_blend_fuel_recipe);
        let alternate_turbo_pressure_motor_recipe = Recipe {
            name: "Alternate: Turbo Pressure Motor",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: motor.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: pressure_conversion_cube.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: packaged_nitrogen_gas.clone(),
                    amount: 24,
                },
                ItemAmount {
                    item: stator.clone(),
                    amount: 8,
                },
            ],
            output: ItemAmount {
                item: turbo_motor.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 32f32,
            alternate: true,
        };
        recipe_registry
            .entry("Turbo Motor")
            .or_insert_with(Vec::new)
            .push(alternate_turbo_pressure_motor_recipe);
        let alternate_crystal_beacon_recipe = Recipe {
            name: "Alternate: Crystal Beacon",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: steel_beam.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 16,
                },
                ItemAmount {
                    item: crystal_oscillator.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: beacon.clone(),
                amount: 20,
            },
            byproduct: None,
            manufacturing_duration: 120f32,
            alternate: true,
        };
        recipe_registry
            .entry("Beacon")
            .or_insert_with(Vec::new)
            .push(alternate_crystal_beacon_recipe);
        let alternate_insulated_cable_recipe = Recipe {
            name: "Alternate: Insulated Cable",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: wire.clone(),
                    amount: 9,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 6,
                },
            ],
            output: ItemAmount {
                item: cable.clone(),
                amount: 20,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Cable")
            .or_insert_with(Vec::new)
            .push(alternate_insulated_cable_recipe);
        let alternate_quickwire_cable_recipe = Recipe {
            name: "Alternate: Quickwire Cable",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: cable.clone(),
                amount: 11,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Cable")
            .or_insert_with(Vec::new)
            .push(alternate_quickwire_cable_recipe);
        let alternate_silicon_circuit_board_recipe = Recipe {
            name: "Alternate: Silicon Circuit Board",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: copper_sheet.clone(),
                    amount: 11,
                },
                ItemAmount {
                    item: silica.clone(),
                    amount: 11,
                },
            ],
            output: ItemAmount {
                item: circuit_board.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Circuit Board")
            .or_insert_with(Vec::new)
            .push(alternate_silicon_circuit_board_recipe);
        let alternate_caterium_circuit_board_recipe = Recipe {
            name: "Alternate: Caterium Circuit Board",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: plastic.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 30,
                },
            ],
            output: ItemAmount {
                item: circuit_board.clone(),
                amount: 7,
            },
            byproduct: None,
            manufacturing_duration: 48f32,
            alternate: true,
        };
        recipe_registry
            .entry("Circuit Board")
            .or_insert_with(Vec::new)
            .push(alternate_caterium_circuit_board_recipe);
        let alternate_charcoal_recipe = Recipe {
            name: "Alternate: Charcoal",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: wood.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: coal.clone(),
                amount: 10,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: true,
        };
        recipe_registry
            .entry("Coal")
            .or_insert_with(Vec::new)
            .push(alternate_charcoal_recipe);
        let alternate_biocoal_recipe = Recipe {
            name: "Alternate: Biocoal",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: biomass.clone(),
                amount: 5,
            }],
            output: ItemAmount {
                item: coal.clone(),
                amount: 6,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Coal")
            .or_insert_with(Vec::new)
            .push(alternate_biocoal_recipe);
        let alternate_caterium_computer_recipe = Recipe {
            name: "Alternate: Caterium Computer",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: circuit_board.clone(),
                    amount: 7,
                },
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 28,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 12,
                },
            ],
            output: ItemAmount {
                item: computer.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Computer")
            .or_insert_with(Vec::new)
            .push(alternate_caterium_computer_recipe);
        let alternate_crystal_computer_recipe = Recipe {
            name: "Alternate: Crystal Computer",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: circuit_board.clone(),
                    amount: 8,
                },
                ItemAmount {
                    item: crystal_oscillator.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: computer.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 64f32,
            alternate: true,
        };
        recipe_registry
            .entry("Computer")
            .or_insert_with(Vec::new)
            .push(alternate_crystal_computer_recipe);
        let alternate_fine_concrete_recipe = Recipe {
            name: "Alternate: Fine Concrete",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: silica.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: limestone.clone(),
                    amount: 12,
                },
            ],
            output: ItemAmount {
                item: concrete.clone(),
                amount: 10,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Concrete")
            .or_insert_with(Vec::new)
            .push(alternate_fine_concrete_recipe);
        let alternate_insulated_crystal_oscillator_recipe = Recipe {
            name: "Alternate: Insulated Crystal Oscillator",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: quartz_crystal.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 7,
                },
                ItemAmount {
                    item: ailimiter.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: crystal_oscillator.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 32f32,
            alternate: true,
        };
        recipe_registry
            .entry("Crystal Oscillator")
            .or_insert_with(Vec::new)
            .push(alternate_insulated_crystal_oscillator_recipe);
        let alternate_electromagnetic_connection_rod_recipe = Recipe {
            name: "Alternate: Electromagnetic Connection Rod",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: stator.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: high_speed_connector.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: electromagnetic_control_rod.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 15f32,
            alternate: true,
        };
        recipe_registry
            .entry("Electromagnetic Control Rod")
            .or_insert_with(Vec::new)
            .push(alternate_electromagnetic_connection_rod_recipe);
        let alternate_encased_industrial_pipe_recipe = Recipe {
            name: "Alternate: Encased Industrial Pipe",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 7,
                },
                ItemAmount {
                    item: concrete.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: encased_industrial_beam.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 15f32,
            alternate: true,
        };
        recipe_registry
            .entry("Encased Industrial Beam")
            .or_insert_with(Vec::new)
            .push(alternate_encased_industrial_pipe_recipe);
        let alternate_compacted_coal_recipe = Recipe {
            name: "Alternate: Compacted Coal",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: coal.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: sulfur.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: compacted_coal.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Compacted Coal")
            .or_insert_with(Vec::new)
            .push(alternate_compacted_coal_recipe);
        let alternate_fine_black_powder_recipe = Recipe {
            name: "Alternate: Fine Black Powder",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: sulfur.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: compacted_coal.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: black_powder.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Black Powder")
            .or_insert_with(Vec::new)
            .push(alternate_fine_black_powder_recipe);
        let alternate_heat_exchanger_recipe = Recipe {
            name: "Alternate: Heat Exchanger",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: aluminum_casing.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: heat_sink.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: true,
        };
        recipe_registry
            .entry("Heat Sink")
            .or_insert_with(Vec::new)
            .push(alternate_heat_exchanger_recipe);
        let alternate_silicon_high_speed_connector_recipe = Recipe {
            name: "Alternate: Silicon High-Speed Connector",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 60,
                },
                ItemAmount {
                    item: silica.clone(),
                    amount: 25,
                },
                ItemAmount {
                    item: circuit_board.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: high_speed_connector.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 40f32,
            alternate: true,
        };
        recipe_registry
            .entry("High-Speed Connector")
            .or_insert_with(Vec::new)
            .push(alternate_silicon_high_speed_connector_recipe);
        let alternate_iron_alloy_ingot_recipe = Recipe {
            name: "Alternate: Iron Alloy Ingot",
            machine: Machine::Foundry,
            input: vec![
                ItemAmount {
                    item: iron_ore.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: copper_ore.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: iron_ingot.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: true,
        };
        recipe_registry
            .entry("Iron Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_iron_alloy_ingot_recipe);
        let alternate_solid_steel_ingot_recipe = Recipe {
            name: "Alternate: Solid Steel Ingot",
            machine: Machine::Foundry,
            input: vec![
                ItemAmount {
                    item: iron_ingot.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: coal.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: steel_ingot.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: true,
        };
        recipe_registry
            .entry("Steel Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_solid_steel_ingot_recipe);
        let alternate_compacted_steel_ingot_recipe = Recipe {
            name: "Alternate: Compacted Steel Ingot",
            machine: Machine::Foundry,
            input: vec![
                ItemAmount {
                    item: iron_ore.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: compacted_coal.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: steel_ingot.clone(),
                amount: 10,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Steel Ingot")
            .or_insert_with(Vec::new)
            .push(alternate_compacted_steel_ingot_recipe);
        let alternate_steeled_frame_recipe = Recipe {
            name: "Alternate: Steeled Frame",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: reinforced_iron_plate.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 10,
                },
            ],
            output: ItemAmount {
                item: modular_frame.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 60f32,
            alternate: true,
        };
        recipe_registry
            .entry("Modular Frame")
            .or_insert_with(Vec::new)
            .push(alternate_steeled_frame_recipe);
        let alternate_heavy_encased_frame_recipe = Recipe {
            name: "Alternate: Heavy Encased Frame",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: modular_frame.clone(),
                    amount: 8,
                },
                ItemAmount {
                    item: encased_industrial_beam.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 36,
                },
                ItemAmount {
                    item: concrete.clone(),
                    amount: 22,
                },
            ],
            output: ItemAmount {
                item: heavy_modular_frame.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 64f32,
            alternate: true,
        };
        recipe_registry
            .entry("Heavy Modular Frame")
            .or_insert_with(Vec::new)
            .push(alternate_heavy_encased_frame_recipe);
        let alternate_rigour_motor_recipe = Recipe {
            name: "Alternate: Rigour Motor",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: rotor.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: stator.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: crystal_oscillator.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: motor.clone(),
                amount: 6,
            },
            byproduct: None,
            manufacturing_duration: 48f32,
            alternate: true,
        };
        recipe_registry
            .entry("Motor")
            .or_insert_with(Vec::new)
            .push(alternate_rigour_motor_recipe);
        let alternate_uranium_fuel_unit_recipe = Recipe {
            name: "Alternate: Uranium Fuel Unit",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: encased_uranium_cell.clone(),
                    amount: 100,
                },
                ItemAmount {
                    item: electromagnetic_control_rod.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: crystal_oscillator.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: beacon.clone(),
                    amount: 6,
                },
            ],
            output: ItemAmount {
                item: uranium_fuel_rod.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 300f32,
            alternate: true,
        };
        recipe_registry
            .entry("Uranium Fuel Rod")
            .or_insert_with(Vec::new)
            .push(alternate_uranium_fuel_unit_recipe);
        let alternate_recycled_plastic_recipe = Recipe {
            name: "Alternate: Recycled Plastic",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: rubber.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: fuel.clone(),
                    amount: 6,
                },
            ],
            output: ItemAmount {
                item: plastic.clone(),
                amount: 12,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Plastic")
            .or_insert_with(Vec::new)
            .push(alternate_recycled_plastic_recipe);
        let alternate_fused_quickwire_recipe = Recipe {
            name: "Alternate: Fused Quickwire",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: caterium_ingot.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: copper_ingot.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: quickwire.clone(),
                amount: 12,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: true,
        };
        recipe_registry
            .entry("Quickwire")
            .or_insert_with(Vec::new)
            .push(alternate_fused_quickwire_recipe);
        let alternate_radio_connection_unit_recipe = Recipe {
            name: "Alternate: Radio Connection Unit",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: heat_sink.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: high_speed_connector.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: quartz_crystal.clone(),
                    amount: 12,
                },
            ],
            output: ItemAmount {
                item: radio_control_unit.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Radio Control Unit")
            .or_insert_with(Vec::new)
            .push(alternate_radio_connection_unit_recipe);
        let alternate_bolted_iron_plate_recipe = Recipe {
            name: "Alternate: Bolted Iron Plate",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 18,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 50,
                },
            ],
            output: ItemAmount {
                item: reinforced_iron_plate.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Reinforced Iron Plate")
            .or_insert_with(Vec::new)
            .push(alternate_bolted_iron_plate_recipe);
        let alternate_stitched_iron_plate_recipe = Recipe {
            name: "Alternate: Stitched Iron Plate",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: wire.clone(),
                    amount: 20,
                },
            ],
            output: ItemAmount {
                item: reinforced_iron_plate.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 32f32,
            alternate: true,
        };
        recipe_registry
            .entry("Reinforced Iron Plate")
            .or_insert_with(Vec::new)
            .push(alternate_stitched_iron_plate_recipe);
        let alternate_steel_rotor_recipe = Recipe {
            name: "Alternate: Steel Rotor",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: wire.clone(),
                    amount: 6,
                },
            ],
            output: ItemAmount {
                item: rotor.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Rotor")
            .or_insert_with(Vec::new)
            .push(alternate_steel_rotor_recipe);
        let alternate_cast_screw_recipe = Recipe {
            name: "Alternate: Cast Screw",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: iron_ingot.clone(),
                amount: 5,
            }],
            output: ItemAmount {
                item: screw.clone(),
                amount: 20,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Screw")
            .or_insert_with(Vec::new)
            .push(alternate_cast_screw_recipe);
        let alternate_steel_screw_recipe = Recipe {
            name: "Alternate: Steel Screw",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: steel_beam.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: screw.clone(),
                amount: 52,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Screw")
            .or_insert_with(Vec::new)
            .push(alternate_steel_screw_recipe);
        let alternate_cheap_silica_recipe = Recipe {
            name: "Alternate: Cheap Silica",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: raw_quartz.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: limestone.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: silica.clone(),
                amount: 7,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Silica")
            .or_insert_with(Vec::new)
            .push(alternate_cheap_silica_recipe);
        let alternate_quickwire_stator_recipe = Recipe {
            name: "Alternate: Quickwire Stator",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 15,
                },
            ],
            output: ItemAmount {
                item: stator.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 15f32,
            alternate: true,
        };
        recipe_registry
            .entry("Stator")
            .or_insert_with(Vec::new)
            .push(alternate_quickwire_stator_recipe);
        let turbofuel_recipe = Recipe {
            name: "Turbofuel",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: fuel.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: compacted_coal.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: turbofuel.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: true,
        };
        recipe_registry
            .entry("Turbofuel")
            .or_insert_with(Vec::new)
            .push(turbofuel_recipe);
        let alternate_turbo_electric_motor_recipe = Recipe {
            name: "Alternate: Turbo Electric Motor",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: motor.clone(),
                    amount: 7,
                },
                ItemAmount {
                    item: radio_control_unit.clone(),
                    amount: 9,
                },
                ItemAmount {
                    item: electromagnetic_control_rod.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: rotor.clone(),
                    amount: 7,
                },
            ],
            output: ItemAmount {
                item: turbo_motor.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 64f32,
            alternate: true,
        };
        recipe_registry
            .entry("Turbo Motor")
            .or_insert_with(Vec::new)
            .push(alternate_turbo_electric_motor_recipe);
        let alternate_infused_uranium_cell_recipe = Recipe {
            name: "Alternate: Infused Uranium Cell",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: uranium.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: silica.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: sulfur.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 15,
                },
            ],
            output: ItemAmount {
                item: encased_uranium_cell.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: true,
        };
        recipe_registry
            .entry("Encased Uranium Cell")
            .or_insert_with(Vec::new)
            .push(alternate_infused_uranium_cell_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: iron_ingot.clone(),
                amount: 5,
            }],
            output: ItemAmount {
                item: wire.clone(),
                amount: 9,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: true,
        };
        recipe_registry
            .entry("Wire")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
        let alternate_caterium_wire_recipe = Recipe {
            name: "Alternate: Caterium Wire",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: caterium_ingot.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: wire.clone(),
                amount: 8,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: true,
        };
        recipe_registry
            .entry("Wire")
            .or_insert_with(Vec::new)
            .push(alternate_caterium_wire_recipe);
        let ailimiter_recipe = Recipe {
            name: "AI Limiter",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: copper_sheet.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 20,
                },
            ],
            output: ItemAmount {
                item: ailimiter.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("AI Limiter")
            .or_insert_with(Vec::new)
            .push(ailimiter_recipe);
        let circuit_board_recipe = Recipe {
            name: "Circuit Board",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: copper_sheet.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: plastic.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: circuit_board.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: false,
        };
        recipe_registry
            .entry("Circuit Board")
            .or_insert_with(Vec::new)
            .push(circuit_board_recipe);
        let electromagnetic_control_rod_recipe = Recipe {
            name: "Electromagnetic Control Rod",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: stator.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: ailimiter.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: electromagnetic_control_rod.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 30f32,
            alternate: false,
        };
        recipe_registry
            .entry("Electromagnetic Control Rod")
            .or_insert_with(Vec::new)
            .push(electromagnetic_control_rod_recipe);
        let encased_industrial_beam_recipe = Recipe {
            name: "Encased Industrial Beam",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: steel_beam.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: concrete.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: encased_industrial_beam.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 10f32,
            alternate: false,
        };
        recipe_registry
            .entry("Encased Industrial Beam")
            .or_insert_with(Vec::new)
            .push(encased_industrial_beam_recipe);
        let heat_sink_recipe = Recipe {
            name: "Heat Sink",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: alclad_aluminum_sheet.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: copper_sheet.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: heat_sink.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: false,
        };
        recipe_registry
            .entry("Heat Sink")
            .or_insert_with(Vec::new)
            .push(heat_sink_recipe);
        let reinforced_iron_plate_recipe = Recipe {
            name: "Reinforced Iron Plate",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 12,
                },
            ],
            output: ItemAmount {
                item: reinforced_iron_plate.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Reinforced Iron Plate")
            .or_insert_with(Vec::new)
            .push(reinforced_iron_plate_recipe);
        let modular_frame_recipe = Recipe {
            name: "Modular Frame",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: reinforced_iron_plate.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: iron_rod.clone(),
                    amount: 12,
                },
            ],
            output: ItemAmount {
                item: modular_frame.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 60f32,
            alternate: false,
        };
        recipe_registry
            .entry("Modular Frame")
            .or_insert_with(Vec::new)
            .push(modular_frame_recipe);
        let motor_recipe = Recipe {
            name: "Motor",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: rotor.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: stator.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: motor.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Motor")
            .or_insert_with(Vec::new)
            .push(motor_recipe);
        let encased_plutonium_cell_recipe = Recipe {
            name: "Encased Plutonium Cell",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: plutonium_pellet.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: concrete.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: encased_plutonium_cell.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Encased Plutonium Cell")
            .or_insert_with(Vec::new)
            .push(encased_plutonium_cell_recipe);
        let pressure_conversion_cube_recipe = Recipe {
            name: "Pressure Conversion Cube",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: fused_modular_frame.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: radio_control_unit.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: pressure_conversion_cube.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 60f32,
            alternate: false,
        };
        recipe_registry
            .entry("Pressure Conversion Cube")
            .or_insert_with(Vec::new)
            .push(pressure_conversion_cube_recipe);
        let rotor_recipe = Recipe {
            name: "Rotor",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: iron_rod.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 25,
                },
            ],
            output: ItemAmount {
                item: rotor.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 15f32,
            alternate: false,
        };
        recipe_registry
            .entry("Rotor")
            .or_insert_with(Vec::new)
            .push(rotor_recipe);
        let stator_recipe = Recipe {
            name: "Stator",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: wire.clone(),
                    amount: 8,
                },
            ],
            output: ItemAmount {
                item: stator.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Stator")
            .or_insert_with(Vec::new)
            .push(stator_recipe);
        let encased_uranium_cell_recipe = Recipe {
            name: "Encased Uranium Cell",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: uranium.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: concrete.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: sulfuric_acid.clone(),
                    amount: 8,
                },
            ],
            output: ItemAmount {
                item: encased_uranium_cell.clone(),
                amount: 5,
            },
            byproduct: Some(ItemAmount {
                item: sulfuric_acid.clone(),
                amount: 2,
            }),
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Encased Uranium Cell")
            .or_insert_with(Vec::new)
            .push(encased_uranium_cell_recipe);
        let cooling_system_recipe = Recipe {
            name: "Cooling System",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: heat_sink.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: nitrogen_gas.clone(),
                    amount: 25,
                },
            ],
            output: ItemAmount {
                item: cooling_system.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 10f32,
            alternate: false,
        };
        recipe_registry
            .entry("Cooling System")
            .or_insert_with(Vec::new)
            .push(cooling_system_recipe);
        let fused_modular_frame_recipe = Recipe {
            name: "Fused Modular Frame",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: heavy_modular_frame.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: aluminum_casing.clone(),
                    amount: 50,
                },
                ItemAmount {
                    item: nitrogen_gas.clone(),
                    amount: 25,
                },
            ],
            output: ItemAmount {
                item: fused_modular_frame.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 40f32,
            alternate: false,
        };
        recipe_registry
            .entry("Fused Modular Frame")
            .or_insert_with(Vec::new)
            .push(fused_modular_frame_recipe);
        let nitric_acid_recipe = Recipe {
            name: "Nitric Acid",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: nitrogen_gas.clone(),
                    amount: 12,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: nitric_acid.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Nitric Acid")
            .or_insert_with(Vec::new)
            .push(nitric_acid_recipe);
        let non_fissile_uranium_recipe = Recipe {
            name: "Non-fissile Uranium",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: uranium_waste.clone(),
                    amount: 15,
                },
                ItemAmount {
                    item: silica.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: nitric_acid.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: sulfuric_acid.clone(),
                    amount: 6,
                },
            ],
            output: ItemAmount {
                item: non_fissile_uranium.clone(),
                amount: 20,
            },
            byproduct: Some(ItemAmount {
                item: water.clone(),
                amount: 6,
            }),
            manufacturing_duration: 24f32,
            alternate: false,
        };
        recipe_registry
            .entry("Non-fissile Uranium")
            .or_insert_with(Vec::new)
            .push(non_fissile_uranium_recipe);
        let alien_dnacapsule_recipe = Recipe {
            name: "Alien DNA Capsule",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: alien_protein.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: alien_dnacapsule.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alien DNA Capsule")
            .or_insert_with(Vec::new)
            .push(alien_dnacapsule_recipe);
        let aluminum_casing_recipe = Recipe {
            name: "Aluminum Casing",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: aluminum_ingot.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: aluminum_casing.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Aluminum Casing")
            .or_insert_with(Vec::new)
            .push(aluminum_casing_recipe);
        let alclad_aluminum_sheet_recipe = Recipe {
            name: "Alclad Aluminum Sheet",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: aluminum_ingot.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: copper_ingot.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: alclad_aluminum_sheet.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alclad Aluminum Sheet")
            .or_insert_with(Vec::new)
            .push(alclad_aluminum_sheet_recipe);
        let solid_biofuel_recipe = Recipe {
            name: "Solid Biofuel",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: biomass.clone(),
                amount: 8,
            }],
            output: ItemAmount {
                item: solid_biofuel.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Solid Biofuel")
            .or_insert_with(Vec::new)
            .push(solid_biofuel_recipe);
        let biomass_recipe = Recipe {
            name: "Biomass",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: alien_protein.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: biomass.clone(),
                amount: 100,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Biomass")
            .or_insert_with(Vec::new)
            .push(biomass_recipe);
        let biomass_recipe = Recipe {
            name: "Biomass",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: leaves.clone(),
                amount: 10,
            }],
            output: ItemAmount {
                item: biomass.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 5f32,
            alternate: false,
        };
        recipe_registry
            .entry("Biomass")
            .or_insert_with(Vec::new)
            .push(biomass_recipe);
        let biomass_recipe = Recipe {
            name: "Biomass",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: mycelia.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: biomass.clone(),
                amount: 10,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Biomass")
            .or_insert_with(Vec::new)
            .push(biomass_recipe);
        let biomass_recipe = Recipe {
            name: "Biomass",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: wood.clone(),
                amount: 4,
            }],
            output: ItemAmount {
                item: biomass.clone(),
                amount: 20,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Biomass")
            .or_insert_with(Vec::new)
            .push(biomass_recipe);
        let cable_recipe = Recipe {
            name: "Cable",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: wire.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: cable.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Cable")
            .or_insert_with(Vec::new)
            .push(cable_recipe);
        let concrete_recipe = Recipe {
            name: "Concrete",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: limestone.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: concrete.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Concrete")
            .or_insert_with(Vec::new)
            .push(concrete_recipe);
        let copper_powder_recipe = Recipe {
            name: "Copper Powder",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: copper_ingot.clone(),
                amount: 30,
            }],
            output: ItemAmount {
                item: copper_powder.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Copper Powder")
            .or_insert_with(Vec::new)
            .push(copper_powder_recipe);
        let copper_sheet_recipe = Recipe {
            name: "Copper Sheet",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: copper_ingot.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: copper_sheet.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Copper Sheet")
            .or_insert_with(Vec::new)
            .push(copper_sheet_recipe);
        let fabric_recipe = Recipe {
            name: "Fabric",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: mycelia.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: biomass.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: fabric.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Fabric")
            .or_insert_with(Vec::new)
            .push(fabric_recipe);
        let empty_canister_recipe = Recipe {
            name: "Empty Canister",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: plastic.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: empty_canister.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Empty Canister")
            .or_insert_with(Vec::new)
            .push(empty_canister_recipe);
        let empty_fluid_tank_recipe = Recipe {
            name: "Empty Fluid Tank",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: aluminum_ingot.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: empty_fluid_tank.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Empty Fluid Tank")
            .or_insert_with(Vec::new)
            .push(empty_fluid_tank_recipe);
        let iron_plate_recipe = Recipe {
            name: "Iron Plate",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: iron_ingot.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: iron_plate.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Iron Plate")
            .or_insert_with(Vec::new)
            .push(iron_plate_recipe);
        let iron_rod_recipe = Recipe {
            name: "Iron Rod",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: iron_ingot.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: iron_rod.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Iron Rod")
            .or_insert_with(Vec::new)
            .push(iron_rod_recipe);
        let uranium_fuel_rod_recipe = Recipe {
            name: "Uranium Fuel Rod",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: encased_uranium_cell.clone(),
                    amount: 50,
                },
                ItemAmount {
                    item: encased_industrial_beam.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: electromagnetic_control_rod.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: uranium_fuel_rod.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 150f32,
            alternate: false,
        };
        recipe_registry
            .entry("Uranium Fuel Rod")
            .or_insert_with(Vec::new)
            .push(uranium_fuel_rod_recipe);
        let power_shard_recipe = Recipe {
            name: "Power Shard",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: blue_power_slug.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: power_shard.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: false,
        };
        recipe_registry
            .entry("Power Shard")
            .or_insert_with(Vec::new)
            .push(power_shard_recipe);
        let power_shard_recipe = Recipe {
            name: "Power Shard",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: yellow_power_slug.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: power_shard.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Power Shard")
            .or_insert_with(Vec::new)
            .push(power_shard_recipe);
        let power_shard_recipe = Recipe {
            name: "Power Shard",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: purple_power_slug.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: power_shard.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: false,
        };
        recipe_registry
            .entry("Power Shard")
            .or_insert_with(Vec::new)
            .push(power_shard_recipe);
        let alien_protein_recipe = Recipe {
            name: "Alien Protein",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: hatcher_remains.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: alien_protein.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alien Protein")
            .or_insert_with(Vec::new)
            .push(alien_protein_recipe);
        let alien_protein_recipe = Recipe {
            name: "Alien Protein",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: hog_remains.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: alien_protein.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alien Protein")
            .or_insert_with(Vec::new)
            .push(alien_protein_recipe);
        let alien_protein_recipe = Recipe {
            name: "Alien Protein",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: plasma_spitter_remains.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: alien_protein.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alien Protein")
            .or_insert_with(Vec::new)
            .push(alien_protein_recipe);
        let alien_protein_recipe = Recipe {
            name: "Alien Protein",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: stinger_remains.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: alien_protein.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alien Protein")
            .or_insert_with(Vec::new)
            .push(alien_protein_recipe);
        let quartz_crystal_recipe = Recipe {
            name: "Quartz Crystal",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: raw_quartz.clone(),
                amount: 5,
            }],
            output: ItemAmount {
                item: quartz_crystal.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: false,
        };
        recipe_registry
            .entry("Quartz Crystal")
            .or_insert_with(Vec::new)
            .push(quartz_crystal_recipe);
        let quickwire_recipe = Recipe {
            name: "Quickwire",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: caterium_ingot.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: quickwire.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 5f32,
            alternate: false,
        };
        recipe_registry
            .entry("Quickwire")
            .or_insert_with(Vec::new)
            .push(quickwire_recipe);
        let screw_recipe = Recipe {
            name: "Screw",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: iron_rod.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: screw.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Screw")
            .or_insert_with(Vec::new)
            .push(screw_recipe);
        let silica_recipe = Recipe {
            name: "Silica",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: raw_quartz.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: silica.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: false,
        };
        recipe_registry
            .entry("Silica")
            .or_insert_with(Vec::new)
            .push(silica_recipe);
        let steel_beam_recipe = Recipe {
            name: "Steel Beam",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: steel_ingot.clone(),
                amount: 4,
            }],
            output: ItemAmount {
                item: steel_beam.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Steel Beam")
            .or_insert_with(Vec::new)
            .push(steel_beam_recipe);
        let steel_pipe_recipe = Recipe {
            name: "Steel Pipe",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: steel_ingot.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: steel_pipe.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Steel Pipe")
            .or_insert_with(Vec::new)
            .push(steel_pipe_recipe);
        let wire_recipe = Recipe {
            name: "Wire",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: copper_ingot.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: wire.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Wire")
            .or_insert_with(Vec::new)
            .push(wire_recipe);
        let beacon_recipe = Recipe {
            name: "Beacon",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: iron_plate.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: iron_rod.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: wire.clone(),
                    amount: 15,
                },
                ItemAmount {
                    item: cable.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: beacon.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: false,
        };
        recipe_registry
            .entry("Beacon")
            .or_insert_with(Vec::new)
            .push(beacon_recipe);
        let rifle_ammo_recipe = Recipe {
            name: "Rifle Ammo",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: copper_sheet.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: smokeless_powder.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: rifle_ammo.clone(),
                amount: 15,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Rifle Ammo")
            .or_insert_with(Vec::new)
            .push(rifle_ammo_recipe);
        let color_cartridge_recipe = Recipe {
            name: "Color Cartridge",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: flower_petals.clone(),
                amount: 5,
            }],
            output: ItemAmount {
                item: color_cartridge.clone(),
                amount: 10,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Color Cartridge")
            .or_insert_with(Vec::new)
            .push(color_cartridge_recipe);
        let gas_filter_recipe = Recipe {
            name: "Gas Filter",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: coal.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: fabric.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: gas_filter.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 8f32,
            alternate: false,
        };
        recipe_registry
            .entry("Gas Filter")
            .or_insert_with(Vec::new)
            .push(gas_filter_recipe);
        let iodine_infused_filter_recipe = Recipe {
            name: "Iodine Infused Filter",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: gas_filter.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 8,
                },
                ItemAmount {
                    item: aluminum_casing.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: iodine_infused_filter.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: false,
        };
        recipe_registry
            .entry("Iodine Infused Filter")
            .or_insert_with(Vec::new)
            .push(iodine_infused_filter_recipe);
        let black_powder_recipe = Recipe {
            name: "Black Powder",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: coal.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: sulfur.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: black_powder.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Black Powder")
            .or_insert_with(Vec::new)
            .push(black_powder_recipe);
        let smokeless_powder_recipe = Recipe {
            name: "Smokeless Powder",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: black_powder.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: heavy_oil_residue.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: smokeless_powder.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Smokeless Powder")
            .or_insert_with(Vec::new)
            .push(smokeless_powder_recipe);
        let nobelisk_recipe = Recipe {
            name: "Nobelisk",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: black_powder.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: nobelisk.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Nobelisk")
            .or_insert_with(Vec::new)
            .push(nobelisk_recipe);
        let iron_rebar_recipe = Recipe {
            name: "Iron Rebar",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: iron_rod.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: iron_rebar.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Iron Rebar")
            .or_insert_with(Vec::new)
            .push(iron_rebar_recipe);
        let snowball_recipe = Recipe {
            name: "Snowball",
            machine: Machine::Constructor,
            input: vec![ItemAmount {
                item: actual_snow.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: snowball.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 12f32,
            alternate: false,
        };
        recipe_registry
            .entry("Snowball")
            .or_insert_with(Vec::new)
            .push(snowball_recipe);
        let battery_recipe = Recipe {
            name: "Battery",
            machine: Machine::Blender,
            input: vec![
                ItemAmount {
                    item: sulfuric_acid.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: alumina_solution.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: aluminum_casing.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: battery.clone(),
                amount: 1,
            },
            byproduct: Some(ItemAmount {
                item: water.clone(),
                amount: 1,
            }),
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Battery")
            .or_insert_with(Vec::new)
            .push(battery_recipe);
        let computer_recipe = Recipe {
            name: "Computer",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: circuit_board.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: cable.clone(),
                    amount: 9,
                },
                ItemAmount {
                    item: plastic.clone(),
                    amount: 18,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 52,
                },
            ],
            output: ItemAmount {
                item: computer.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: false,
        };
        recipe_registry
            .entry("Computer")
            .or_insert_with(Vec::new)
            .push(computer_recipe);
        let supercomputer_recipe = Recipe {
            name: "Supercomputer",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: computer.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: ailimiter.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: high_speed_connector.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: plastic.clone(),
                    amount: 28,
                },
            ],
            output: ItemAmount {
                item: supercomputer.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 32f32,
            alternate: false,
        };
        recipe_registry
            .entry("Supercomputer")
            .or_insert_with(Vec::new)
            .push(supercomputer_recipe);
        let crystal_oscillator_recipe = Recipe {
            name: "Crystal Oscillator",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: quartz_crystal.clone(),
                    amount: 36,
                },
                ItemAmount {
                    item: cable.clone(),
                    amount: 28,
                },
                ItemAmount {
                    item: reinforced_iron_plate.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: crystal_oscillator.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 120f32,
            alternate: false,
        };
        recipe_registry
            .entry("Crystal Oscillator")
            .or_insert_with(Vec::new)
            .push(crystal_oscillator_recipe);
        let high_speed_connector_recipe = Recipe {
            name: "High-Speed Connector",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: quickwire.clone(),
                    amount: 56,
                },
                ItemAmount {
                    item: cable.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: circuit_board.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: high_speed_connector.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 16f32,
            alternate: false,
        };
        recipe_registry
            .entry("High-Speed Connector")
            .or_insert_with(Vec::new)
            .push(high_speed_connector_recipe);
        let heavy_modular_frame_recipe = Recipe {
            name: "Heavy Modular Frame",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: modular_frame.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: steel_pipe.clone(),
                    amount: 15,
                },
                ItemAmount {
                    item: encased_industrial_beam.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: screw.clone(),
                    amount: 100,
                },
            ],
            output: ItemAmount {
                item: heavy_modular_frame.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 30f32,
            alternate: false,
        };
        recipe_registry
            .entry("Heavy Modular Frame")
            .or_insert_with(Vec::new)
            .push(heavy_modular_frame_recipe);
        let turbo_motor_recipe = Recipe {
            name: "Turbo Motor",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: cooling_system.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: radio_control_unit.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: motor.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 24,
                },
            ],
            output: ItemAmount {
                item: turbo_motor.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 32f32,
            alternate: false,
        };
        recipe_registry
            .entry("Turbo Motor")
            .or_insert_with(Vec::new)
            .push(turbo_motor_recipe);
        let plutonium_fuel_rod_recipe = Recipe {
            name: "Plutonium Fuel Rod",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: encased_plutonium_cell.clone(),
                    amount: 30,
                },
                ItemAmount {
                    item: steel_beam.clone(),
                    amount: 18,
                },
                ItemAmount {
                    item: electromagnetic_control_rod.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: heat_sink.clone(),
                    amount: 10,
                },
            ],
            output: ItemAmount {
                item: plutonium_fuel_rod.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 240f32,
            alternate: false,
        };
        recipe_registry
            .entry("Plutonium Fuel Rod")
            .or_insert_with(Vec::new)
            .push(plutonium_fuel_rod_recipe);
        let radio_control_unit_recipe = Recipe {
            name: "Radio Control Unit",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: aluminum_casing.clone(),
                    amount: 32,
                },
                ItemAmount {
                    item: crystal_oscillator.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: computer.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: radio_control_unit.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 48f32,
            alternate: false,
        };
        recipe_registry
            .entry("Radio Control Unit")
            .or_insert_with(Vec::new)
            .push(radio_control_unit_recipe);
        let alumina_solution_recipe = Recipe {
            name: "Alumina Solution",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: bauxite.clone(),
                    amount: 12,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 18,
                },
            ],
            output: ItemAmount {
                item: alumina_solution.clone(),
                amount: 12,
            },
            byproduct: Some(ItemAmount {
                item: silica.clone(),
                amount: 5,
            }),
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alumina Solution")
            .or_insert_with(Vec::new)
            .push(alumina_solution_recipe);
        let aluminum_scrap_recipe = Recipe {
            name: "Aluminum Scrap",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: alumina_solution.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: coal.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: aluminum_scrap.clone(),
                amount: 6,
            },
            byproduct: Some(ItemAmount {
                item: water.clone(),
                amount: 2,
            }),
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Aluminum Scrap")
            .or_insert_with(Vec::new)
            .push(aluminum_scrap_recipe);
        let packaged_fuel_recipe = Recipe {
            name: "Packaged Fuel",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: fuel.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_fuel.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Fuel")
            .or_insert_with(Vec::new)
            .push(packaged_fuel_recipe);
        let liquid_biofuel_recipe = Recipe {
            name: "Liquid Biofuel",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: solid_biofuel.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: liquid_biofuel.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Liquid Biofuel")
            .or_insert_with(Vec::new)
            .push(liquid_biofuel_recipe);
        let fuel_recipe = Recipe {
            name: "Fuel",
            machine: Machine::Refinery,
            input: vec![ItemAmount {
                item: crude_oil.clone(),
                amount: 6,
            }],
            output: ItemAmount {
                item: fuel.clone(),
                amount: 4,
            },
            byproduct: Some(ItemAmount {
                item: polymer_resin.clone(),
                amount: 3,
            }),
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Fuel")
            .or_insert_with(Vec::new)
            .push(fuel_recipe);
        let packaged_liquid_biofuel_recipe = Recipe {
            name: "Packaged Liquid Biofuel",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: liquid_biofuel.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_liquid_biofuel.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Liquid Biofuel")
            .or_insert_with(Vec::new)
            .push(packaged_liquid_biofuel_recipe);
        let packaged_oil_recipe = Recipe {
            name: "Packaged Oil",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: crude_oil.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_oil.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Oil")
            .or_insert_with(Vec::new)
            .push(packaged_oil_recipe);
        let packaged_heavy_oil_residue_recipe = Recipe {
            name: "Packaged Heavy Oil Residue",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: heavy_oil_residue.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_heavy_oil_residue.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Heavy Oil Residue")
            .or_insert_with(Vec::new)
            .push(packaged_heavy_oil_residue_recipe);
        let packaged_turbofuel_recipe = Recipe {
            name: "Packaged Turbofuel",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: turbofuel.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_turbofuel.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Turbofuel")
            .or_insert_with(Vec::new)
            .push(packaged_turbofuel_recipe);
        let packaged_water_recipe = Recipe {
            name: "Packaged Water",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: water.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_water.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Water")
            .or_insert_with(Vec::new)
            .push(packaged_water_recipe);
        let petroleum_coke_recipe = Recipe {
            name: "Petroleum Coke",
            machine: Machine::Refinery,
            input: vec![ItemAmount {
                item: heavy_oil_residue.clone(),
                amount: 4,
            }],
            output: ItemAmount {
                item: petroleum_coke.clone(),
                amount: 12,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Petroleum Coke")
            .or_insert_with(Vec::new)
            .push(petroleum_coke_recipe);
        let plastic_recipe = Recipe {
            name: "Plastic",
            machine: Machine::Refinery,
            input: vec![ItemAmount {
                item: crude_oil.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: plastic.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: heavy_oil_residue.clone(),
                amount: 1,
            }),
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Plastic")
            .or_insert_with(Vec::new)
            .push(plastic_recipe);
        let fuel_recipe = Recipe {
            name: "Fuel",
            machine: Machine::Refinery,
            input: vec![ItemAmount {
                item: heavy_oil_residue.clone(),
                amount: 6,
            }],
            output: ItemAmount {
                item: fuel.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Fuel")
            .or_insert_with(Vec::new)
            .push(fuel_recipe);
        let plastic_recipe = Recipe {
            name: "Plastic",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: polymer_resin.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: plastic.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Plastic")
            .or_insert_with(Vec::new)
            .push(plastic_recipe);
        let rubber_recipe = Recipe {
            name: "Rubber",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: polymer_resin.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 4,
                },
            ],
            output: ItemAmount {
                item: rubber.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Rubber")
            .or_insert_with(Vec::new)
            .push(rubber_recipe);
        let rubber_recipe = Recipe {
            name: "Rubber",
            machine: Machine::Refinery,
            input: vec![ItemAmount {
                item: crude_oil.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: rubber.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: heavy_oil_residue.clone(),
                amount: 2,
            }),
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Rubber")
            .or_insert_with(Vec::new)
            .push(rubber_recipe);
        let sulfuric_acid_recipe = Recipe {
            name: "Sulfuric Acid",
            machine: Machine::Refinery,
            input: vec![
                ItemAmount {
                    item: sulfur.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: water.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: sulfuric_acid.clone(),
                amount: 5,
            },
            byproduct: None,
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Sulfuric Acid")
            .or_insert_with(Vec::new)
            .push(sulfuric_acid_recipe);
        let liquid_biofuel_recipe = Recipe {
            name: "Liquid Biofuel",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_liquid_biofuel.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: liquid_biofuel.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            }),
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Liquid Biofuel")
            .or_insert_with(Vec::new)
            .push(liquid_biofuel_recipe);
        let fuel_recipe = Recipe {
            name: "Fuel",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_fuel.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: fuel.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            }),
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Fuel")
            .or_insert_with(Vec::new)
            .push(fuel_recipe);
        let crude_oil_recipe = Recipe {
            name: "Crude Oil",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_oil.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: crude_oil.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            }),
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Crude Oil")
            .or_insert_with(Vec::new)
            .push(crude_oil_recipe);
        let heavy_oil_residue_recipe = Recipe {
            name: "Heavy Oil Residue",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_heavy_oil_residue.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: heavy_oil_residue.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            }),
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Heavy Oil Residue")
            .or_insert_with(Vec::new)
            .push(heavy_oil_residue_recipe);
        let turbofuel_recipe = Recipe {
            name: "Turbofuel",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_turbofuel.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: turbofuel.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            }),
            manufacturing_duration: 6f32,
            alternate: false,
        };
        recipe_registry
            .entry("Turbofuel")
            .or_insert_with(Vec::new)
            .push(turbofuel_recipe);
        let water_recipe = Recipe {
            name: "Water",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_water.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: water.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            }),
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Water")
            .or_insert_with(Vec::new)
            .push(water_recipe);
        let packaged_alumina_solution_recipe = Recipe {
            name: "Packaged Alumina Solution",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: alumina_solution.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_alumina_solution.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Alumina Solution")
            .or_insert_with(Vec::new)
            .push(packaged_alumina_solution_recipe);
        let packaged_nitric_acid_recipe = Recipe {
            name: "Packaged Nitric Acid",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: nitric_acid.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: empty_fluid_tank.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: packaged_nitric_acid.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Nitric Acid")
            .or_insert_with(Vec::new)
            .push(packaged_nitric_acid_recipe);
        let packaged_nitrogen_gas_recipe = Recipe {
            name: "Packaged Nitrogen Gas",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: nitrogen_gas.clone(),
                    amount: 4,
                },
                ItemAmount {
                    item: empty_fluid_tank.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: packaged_nitrogen_gas.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Nitrogen Gas")
            .or_insert_with(Vec::new)
            .push(packaged_nitrogen_gas_recipe);
        let packaged_sulfuric_acid_recipe = Recipe {
            name: "Packaged Sulfuric Acid",
            machine: Machine::Packager,
            input: vec![
                ItemAmount {
                    item: sulfuric_acid.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: empty_canister.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: packaged_sulfuric_acid.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Packaged Sulfuric Acid")
            .or_insert_with(Vec::new)
            .push(packaged_sulfuric_acid_recipe);
        let alumina_solution_recipe = Recipe {
            name: "Alumina Solution",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_alumina_solution.clone(),
                amount: 2,
            }],
            output: ItemAmount {
                item: alumina_solution.clone(),
                amount: 2,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 2,
            }),
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Alumina Solution")
            .or_insert_with(Vec::new)
            .push(alumina_solution_recipe);
        let nitric_acid_recipe = Recipe {
            name: "Nitric Acid",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_nitric_acid.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: nitric_acid.clone(),
                amount: 1,
            },
            byproduct: Some(ItemAmount {
                item: empty_fluid_tank.clone(),
                amount: 1,
            }),
            manufacturing_duration: 3f32,
            alternate: false,
        };
        recipe_registry
            .entry("Nitric Acid")
            .or_insert_with(Vec::new)
            .push(nitric_acid_recipe);
        let nitrogen_gas_recipe = Recipe {
            name: "Nitrogen Gas",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_nitrogen_gas.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: nitrogen_gas.clone(),
                amount: 4,
            },
            byproduct: Some(ItemAmount {
                item: empty_fluid_tank.clone(),
                amount: 1,
            }),
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Nitrogen Gas")
            .or_insert_with(Vec::new)
            .push(nitrogen_gas_recipe);
        let sulfuric_acid_recipe = Recipe {
            name: "Sulfuric Acid",
            machine: Machine::Packager,
            input: vec![ItemAmount {
                item: packaged_sulfuric_acid.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: sulfuric_acid.clone(),
                amount: 1,
            },
            byproduct: Some(ItemAmount {
                item: empty_canister.clone(),
                amount: 1,
            }),
            manufacturing_duration: 0f32,
            alternate: false,
        };
        recipe_registry
            .entry("Sulfuric Acid")
            .or_insert_with(Vec::new)
            .push(sulfuric_acid_recipe);
        let aluminum_ingot_recipe = Recipe {
            name: "Aluminum Ingot",
            machine: Machine::Foundry,
            input: vec![
                ItemAmount {
                    item: aluminum_scrap.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: silica.clone(),
                    amount: 5,
                },
            ],
            output: ItemAmount {
                item: aluminum_ingot.clone(),
                amount: 4,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Aluminum Ingot")
            .or_insert_with(Vec::new)
            .push(aluminum_ingot_recipe);
        let caterium_ingot_recipe = Recipe {
            name: "Caterium Ingot",
            machine: Machine::Smelter,
            input: vec![ItemAmount {
                item: caterium_ore.clone(),
                amount: 3,
            }],
            output: ItemAmount {
                item: caterium_ingot.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Caterium Ingot")
            .or_insert_with(Vec::new)
            .push(caterium_ingot_recipe);
        let copper_ingot_recipe = Recipe {
            name: "Copper Ingot",
            machine: Machine::Smelter,
            input: vec![ItemAmount {
                item: copper_ore.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: copper_ingot.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Copper Ingot")
            .or_insert_with(Vec::new)
            .push(copper_ingot_recipe);
        let iron_ingot_recipe = Recipe {
            name: "Iron Ingot",
            machine: Machine::Smelter,
            input: vec![ItemAmount {
                item: iron_ore.clone(),
                amount: 1,
            }],
            output: ItemAmount {
                item: iron_ingot.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 2f32,
            alternate: false,
        };
        recipe_registry
            .entry("Iron Ingot")
            .or_insert_with(Vec::new)
            .push(iron_ingot_recipe);
        let steel_ingot_recipe = Recipe {
            name: "Steel Ingot",
            machine: Machine::Foundry,
            input: vec![
                ItemAmount {
                    item: iron_ore.clone(),
                    amount: 3,
                },
                ItemAmount {
                    item: coal.clone(),
                    amount: 3,
                },
            ],
            output: ItemAmount {
                item: steel_ingot.clone(),
                amount: 3,
            },
            byproduct: None,
            manufacturing_duration: 4f32,
            alternate: false,
        };
        recipe_registry
            .entry("Steel Ingot")
            .or_insert_with(Vec::new)
            .push(steel_ingot_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: reinforced_iron_plate.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: rotor.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: smart_plating.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 30f32,
            alternate: false,
        };
        recipe_registry
            .entry("Smart Plating")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: modular_frame.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: steel_beam.clone(),
                    amount: 12,
                },
            ],
            output: ItemAmount {
                item: versatile_framework.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: false,
        };
        recipe_registry
            .entry("Versatile Framework")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: stator.clone(),
                    amount: 1,
                },
                ItemAmount {
                    item: cable.clone(),
                    amount: 20,
                },
            ],
            output: ItemAmount {
                item: automated_wiring.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 24f32,
            alternate: false,
        };
        recipe_registry
            .entry("Automated Wiring")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: motor.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: rubber.clone(),
                    amount: 15,
                },
                ItemAmount {
                    item: smart_plating.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: modular_engine.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 60f32,
            alternate: false,
        };
        recipe_registry
            .entry("Modular Engine")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: automated_wiring.clone(),
                    amount: 15,
                },
                ItemAmount {
                    item: circuit_board.clone(),
                    amount: 10,
                },
                ItemAmount {
                    item: heavy_modular_frame.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: computer.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: adaptive_control_unit.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 120f32,
            alternate: false,
        };
        recipe_registry
            .entry("Adaptive Control Unit")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: versatile_framework.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: electromagnetic_control_rod.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: battery.clone(),
                    amount: 10,
                },
            ],
            output: ItemAmount {
                item: magnetic_field_generator.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 120f32,
            alternate: false,
        };
        recipe_registry
            .entry("Magnetic Field Generator")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
        let assembly_director_system_recipe = Recipe {
            name: "Assembly Director System",
            machine: Machine::Assembler,
            input: vec![
                ItemAmount {
                    item: adaptive_control_unit.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: supercomputer.clone(),
                    amount: 1,
                },
            ],
            output: ItemAmount {
                item: assembly_director_system.clone(),
                amount: 1,
            },
            byproduct: None,
            manufacturing_duration: 80f32,
            alternate: false,
        };
        recipe_registry
            .entry("Assembly Director System")
            .or_insert_with(Vec::new)
            .push(assembly_director_system_recipe);
        let alternate_iron_wire_recipe = Recipe {
            name: "Alternate: Iron Wire",
            machine: Machine::Manufacturer,
            input: vec![
                ItemAmount {
                    item: modular_engine.clone(),
                    amount: 5,
                },
                ItemAmount {
                    item: turbo_motor.clone(),
                    amount: 2,
                },
                ItemAmount {
                    item: cooling_system.clone(),
                    amount: 6,
                },
                ItemAmount {
                    item: fused_modular_frame.clone(),
                    amount: 2,
                },
            ],
            output: ItemAmount {
                item: thermal_propulsion_rocket.clone(),
                amount: 2,
            },
            byproduct: None,
            manufacturing_duration: 120f32,
            alternate: false,
        };
        recipe_registry
            .entry("Thermal Propulsion Rocket")
            .or_insert_with(Vec::new)
            .push(alternate_iron_wire_recipe);
    }
    (item_registry, recipe_registry)
}
