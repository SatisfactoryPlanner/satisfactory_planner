import { Grid, Typography } from "@mui/joy";

type InfoLineProps = {
  text: string;
  value: string;
  children?: any;
};

export default function InfoLine(props: InfoLineProps) {
  return (
    <Grid container columnSpacing={2} alignItems="center">
      {props.children}

      <Grid>
        <Typography>{props.text}</Typography>
      </Grid>

      <Grid container direction="row-reverse" xs>
        <Grid>
          <Typography>{props.value}</Typography>
        </Grid>
      </Grid>
    </Grid>
  );
}
