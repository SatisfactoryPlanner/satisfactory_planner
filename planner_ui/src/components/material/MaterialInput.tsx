import MaterialDisplay from "./MaterialDisplay";
import {Grid, Input, Typography} from "@mui/joy";


type MaterialInputProps = {
  name: string;
};

export default function MaterialInput(props: MaterialInputProps) {
  return (
    <MaterialDisplay name={props.name}>
      <Grid container direction="row-reverse" xs>
        <Grid>
          <Input size="sm" style={{ maxWidth: "14ch" }} type="number" endDecorator={
            <Typography level="body1">/min</Typography>
          } />
        </Grid>
      </Grid>
    </MaterialDisplay>
  );
}
