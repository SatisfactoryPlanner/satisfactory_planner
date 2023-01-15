import {
  Box,
  Card,
  CardContent,
  Checkbox,
  Grid,
  Typography,
} from "@mui/material";
import InfoLine from "../InfoLine";

type PartProps = {
  name: string;
  amount: number;
};

function Part(props: PartProps) {
  return (
    <InfoLine text={props.name} value={props.amount.toString()}>
      <Grid item>
        <img
          style={{ height: "1.3em" }}
          src={`/item_icons/${props.name}.png`}
          loading="lazy"
          alt={`${props.name} icon`}
        />
      </Grid>
      <Grid item>
        <Checkbox sx={{ margin: 0, padding: 0 }} size="small" />
      </Grid>
    </InfoLine>
  );
}

export default function PartsList() {
  return (
    <Grid container item style={{ margin: 5 }}>
      <Card component={Grid} item elevation={8} xs={12}>
        <Box sx={{ m: 1 }} />

        <Typography style={{ textAlign: "center" }} variant="h6">
          Parts List
        </Typography>

        <CardContent>
          <Part name="Motor" amount={385} />
          <Part name="Steel Pipe" amount={90} />
        </CardContent>
      </Card>
    </Grid>
  );
}
