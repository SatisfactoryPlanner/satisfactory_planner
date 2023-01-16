export const enum MachineType {
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

export class MachineData {
  constructor(
    public visualHeight: number,
    public visualWidth: number,
    public machineSymbol: string
  ) {}
}

export default function getMachineData(type: MachineType): MachineData {
  switch (type) {
    case MachineType.Assembler:
      return new MachineData(3.75, 2.5, "A");
    case MachineType.Blender:
      return new MachineData(4, 4.5, "B");
    case MachineType.Constructor:
      return new MachineData(2.5, 2, "C");
    case MachineType.Foundry:
      return new MachineData(2.25, 2.5, "F");
    case MachineType.Manufacturer:
      return new MachineData(4.75, 4.5, "M");
    case MachineType.Packager:
      return new MachineData(2, 2, "P");
    case MachineType.ParticleAccelerator:
      return new MachineData(9.5, 6, "PA");
    case MachineType.Refinery:
      return new MachineData(5, 2.5, "R");
    case MachineType.Smelter:
      return new MachineData(2.25, 1.5, "S");
  }
}
