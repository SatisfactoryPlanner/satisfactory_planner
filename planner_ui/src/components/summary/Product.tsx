import {
  Autocomplete,
  Box,
  Button,
  FormControl,
  FormHelperText,
  Grid,
  Stack,
  Input,
  Sheet,
  MenuItem,
  Select,
  TextField, AutocompleteOption, Card, CardContent, Typography, Option, Chip, Divider,
} from "@mui/joy";
import React, {useEffect, useMemo, useState} from "react";
import {FixedSizeList} from "react-window";

import {get_craftable_items, get_item_recipes} from "@rsw/planner_lib_wasm";
import AutoSizer from "react-virtualized-auto-sizer";
import ListboxComponent from "../../virtualization/ListboxComponent";
import MaterialIcon from "../material/MaterialIcon";
import Recipe from "../../data/Recipe";
import ItemRate from "../../data/ItemRate";
import MaterialDisplay from "../material/MaterialDisplay";
import RecipeView from "../recipe/RecipeView";
import Item from "../../data/Item";

export default function Product() {
  const [selectedItem, setSelectedItem] = useState<string | null>(null);
  const [perMinute, setPerMinute] = useState(60);
  const [production, setProduction] = useState("mk1");

  const handleSelectedChange = (event: React.SyntheticEvent, value: string | null): void => {
    setSelectedItem(value);
  };

  const handlePerMinuteChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setPerMinute(parseInt(event.target.value, 10));
  }

  const handleProductionChange = (event: React.SyntheticEvent | null, value: string | null): void => {
    if (value != null) {
      setProduction(value);
    }
  }

  const items = get_craftable_items() as Item[];


  const [selectedRecipe, setSelectedRecipe] = useState<Recipe | null>(null);
  const itemRecipes = useMemo(() => {
    if (selectedItem != null) {
      return get_item_recipes(selectedItem) as Recipe[];
    }
    return null;
  }, [selectedItem]);

  useEffect(() => {
    const defaultRecipe = itemRecipes?.find(e => !e.alternate);
    setSelectedRecipe(defaultRecipe ?? null);
  }, [itemRecipes])

  const handleRecipeChange = (event: React.SyntheticEvent | null, value: Recipe | null): void => {
    setSelectedRecipe(value)
  }


  return (
      <Card variant="outlined" sx={{ p: 1.5 }}>
        <Typography sx={{ textAlign: "center" }} level="h6">
          Recipe
        </Typography>

        <Box sx={{ m: 0.5 }} />

        <Grid container rowSpacing={1} direction="column">

          <Grid container xs={12} spacing={1}>
            <Grid xs={8}>
              
              <Autocomplete
                options={items.map(e => e.name)}
                autoHighlight
                onChange={handleSelectedChange}
                slots={{
                  listbox: ListboxComponent,
                }}
                startDecorator={selectedItem != null && <MaterialIcon style={{ height: "1.8em" }} materialName={selectedItem}/>}
                renderOption={(props, option) => [props, QueryBarEntry(option)] as React.ReactNode}
              />
            </Grid>

            <Grid xs>
              <Input
                size="md"
                style={{ maxWidth: "14ch" }}
                value={perMinute}
                endDecorator = {<Typography level="body1">/min</Typography>}
                type="number"
                onChange={handlePerMinuteChange}/>
            </Grid>
          </Grid>

          <Grid xs={12}>
            <Select value={production} onChange={handleProductionChange}>
              <Option value="mk1">Production MK1</Option>
              <Option value="mk2">Production MK2 (MK++ Mod)</Option>
              <Option value="mk3">Production MK3 (MK++ Mod)</Option>
              <Option value="mk4">Production MK4 (MK++ Mod)</Option>
            </Select>
          </Grid>

          {itemRecipes != null &&
            <Grid xs={12}>
              <Card variant="outlined" sx={{ p: 0 }}>
                <Select
                    className="select-recipe"
                    sx={{ px: 0.2, py: 2 }}
                    value={selectedRecipe}
                    onChange={handleRecipeChange}
                    renderValue={(selected) => (selected != null &&
                        <RecipeView recipe={selected.value}/>
                    )}>
                  {itemRecipes.map(e => {
                    return (
                      <Option value={e}>
                        <Stack direction="column" sx={{ width: "100%" }}>
                          <RecipeView recipe={e}/>
                          <Divider sx={{ mt: 1, mb: 0 }} />
                        </Stack>
                      </Option>
                    )
                  })}
                </Select>
              </Card>
            </Grid>
          }

          <Grid container justifyContent="center">
            <Grid>
              <Button size="md">
                Calculate
              </Button>
            </Grid>
          </Grid>
        </Grid>
      </Card>
  );
}
