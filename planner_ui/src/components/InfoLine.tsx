import { Grid, Typography } from "@mui/material";

type InfoLineProps = {
  text: string;
  value: string;
  children?: any;
};

export default function InfoLine(props: InfoLineProps) {
  return (
    <Grid container columnSpacing={1} alignItems="center">
      {props.children}

      <Grid item>
        <Typography>{props.text}</Typography>
      </Grid>

      <Grid item container direction="row-reverse" xs>
        <Grid item>
          <Typography>{props.value}</Typography>
        </Grid>
      </Grid>
    </Grid>
  );
}
