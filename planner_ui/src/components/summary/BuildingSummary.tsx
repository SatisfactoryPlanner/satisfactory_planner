import {
  Box,
  Card,
  CardContent,
  Divider,
  Grid,
  Typography,
} from "@mui/joy";
import InfoLine from "./InfoLine";
import MaterialIcon from "../material/MaterialIcon";

type SummaryEntryProps = {
  machine: string;
  power: string;
  ingredients: string[];
};

function SummaryEntry(props: SummaryEntryProps) {
  return (
    <Grid container alignItems="center">
      <Grid xs={3}>
        <Typography level="body1">{props.machine}</Typography>
      </Grid>
      <Grid xs={2}>
        <Typography level="body1">{props.power}</Typography>
      </Grid>
      <Grid xs>
        {props.ingredients.map((e) => {
          return (
            <Typography
              style={{
                textAlign: "right",
                fontSize: "12px",
              }}
              level="body1"
            >
              {e}
            </Typography>
          );
        })}
      </Grid>
    </Grid>
  );
}

export default function BuildingSummary() {
  return (
    <Grid container xs>
      <Card variant="outlined" component={Grid} xs={12}>
        <Typography sx={{ textAlign: "center" }} level="h6">
          Building Summary
        </Typography>

        <Box sx={{ m: 0.5 }} />

        <CardContent>
          <InfoLine text="Power Shards" value="0">
            <MaterialIcon style={{ height: "2em" }} materialName="Power Shard"/>
          </InfoLine>

          <Divider sx={{ my: 1 }} />

          <SummaryEntry
            machine="9x Refinery (mk1)"
            power="123 MW"
            ingredients={[
              "Motor 90",
              "Encased Industrial Beam 90",
              "Steel Pipe 270",
              "Copper Sheet 180",
            ]}
          />
          <Divider sx={{ my: 1}} />

          <SummaryEntry
            machine="57x Constructor (mk1)"
            power="201 MW"
            ingredients={["Reinforced Iron Plate 144", "Cable 456"]}
          />
        </CardContent>
      </Card>
    </Grid>
  );
}
