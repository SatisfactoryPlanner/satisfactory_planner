import {Box, Grid, Typography} from "@mui/joy";
import MaterialIcon from "./MaterialIcon";

type MaterialProps = {
  name: string;
  children?: any;
  perMinute?: number;
};

export default function MaterialDisplay(props: MaterialProps) {
  return (
    <Grid container alignItems="center">
      <Grid>
        <MaterialIcon style={{ height: "2em" }} materialName={props.name}/>
      </Grid>

      <Box sx={{ m: 0.5 }} />

      <Grid>
        <Typography>{props.name}</Typography>
        {props.perMinute != null && (
          <Typography level="body1">{props.perMinute} / min</Typography>
        )}
      </Grid>

      {props.children}
    </Grid>
  );
}