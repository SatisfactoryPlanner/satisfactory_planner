import ItemRate from "./ItemRate";

export default class Recipe {
  constructor(public name: string, public machine: string, public manufacturingDuration: number, public output: ItemRate, public input: ItemRate[], public alternate: boolean) {
  }

  static getPerMinute(recipe: Recipe): number {
    return 60 / recipe.manufacturingDuration;
  }
}