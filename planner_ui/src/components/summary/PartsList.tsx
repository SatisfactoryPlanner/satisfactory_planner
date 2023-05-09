import {
  Box,
  Card,
  CardContent,
  Checkbox,
  Grid,
  Typography,
} from "@mui/joy";
import InfoLine from "./InfoLine";
import MaterialIcon from "../material/MaterialIcon";

type PartProps = {
  name: string;
  amount: number;
};

function Part(props: PartProps) {
  return (
    <InfoLine text={props.name} value={`x${props.amount.toString()}`}>
      <Grid>
        <MaterialIcon style={{ height: "2em" }} materialName={props.name}/>
      </Grid>
      <Grid>
        <Checkbox size="sm" />
      </Grid>
    </InfoLine>
  );
}

export default function PartsList() {
  return (
    <Grid container xs>
      <Card variant="outlined" component={Grid} xs={12}>
        <Typography style={{ textAlign: "center" }} level="h6">
          Parts List
        </Typography>

        <Box sx={{ m: 0.5 }} />

        <CardContent>
          <Part name="Motor" amount={385} />
          <Part name="Steel Pipe" amount={90} />
        </CardContent>
      </Card>
    </Grid>
  )
}
