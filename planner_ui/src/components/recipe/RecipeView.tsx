import Recipe from "../../data/Recipe";
import {Box, Chip, Grid, Stack, Typography} from "@mui/joy";
import MaterialIcon from "../material/MaterialIcon";

type RecipeProps = {
  recipe: Recipe
};

export default function RecipeView(props: RecipeProps) {
  return (
    <Grid container sx={{ pr: 2 }} xs>
      <Grid xs={3}>
        <Grid container direction="column"
              alignItems="center"
              justifyContent="center">
          <Grid>
            <MaterialIcon style={{ maxHeight: "3em" }} materialName={props.recipe.output.item.name}/>
          </Grid>
          <Grid>
            {props.recipe.alternate && <Chip color="info" variant="soft" sx={{ "--Chip-radius": "8px", "--Chip-paddingInline": "18px" }}>Alt</Chip>}
          </Grid>
        </Grid>
      </Grid>
      <Grid xs>
        <Grid container direction="column" spacing={0.4}>
          <Grid>
            <Stack alignItems="flex-start">
              <Typography level="h6">{props.recipe.name}</Typography>
              <Typography level="h6">{props.recipe.output.amount * Recipe.getPerMinute(props.recipe)}/min</Typography>
            </Stack>
          </Grid>

          <Grid>
            <Grid container alignItems="center">
              <Grid>
                <Typography level="body1">Input</Typography>
              </Grid>

              <Grid container direction="row-reverse" xs>
                <Typography level="body2">Assembler</Typography>
              </Grid>
            </Grid>
            <Box sx={{ m: 0.5 }}/>
            {props.recipe.input.map(item => {
              return (
                <Grid container alignItems="center">
                  <Grid>
                    <Stack direction="row" alignItems="center" spacing={1}>
                      <MaterialIcon style={{ height: "1.6em" }} materialName={item.item.name}/>
                      <Typography level="body1">{item.item.name}</Typography>
                    </Stack>
                  </Grid>

                  <Grid container direction="row-reverse" xs>
                    <Grid>
                      <Typography level="body1">{item.amount * Recipe.getPerMinute(props.recipe)}/min</Typography>
                    </Grid>
                  </Grid>
                </Grid>
              )
            })}
          </Grid>
        </Grid>
      </Grid>
    </Grid>
  )
}
