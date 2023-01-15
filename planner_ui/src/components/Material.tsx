import { Box, Grid, Input, Typography } from "@mui/material";

type MaterialProps = {
  name: string;
  children?: any;
  per_minute?: number;
};

export default function Material(props: MaterialProps) {
  return (
    <Grid container alignItems="center">
      <Grid item>
        <img
          style={{
            height: "2em",
          }}
          src={`/item_icons/${props.name}.png`}
          loading="lazy"
          alt={`${props.name} icon`}
        />
      </Grid>

      <Box sx={{ m: 0.5 }} />

      <Grid item>
        <Typography>{props.name}</Typography>
        {props.per_minute != null && (
          <Typography variant="subtitle2">{props.per_minute} / min</Typography>
        )}
      </Grid>

      {props.children}
    </Grid>
  );
}
