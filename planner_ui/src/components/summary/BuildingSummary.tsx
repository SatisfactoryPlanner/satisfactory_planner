import {
  Box,
  Card,
  CardContent,
  Divider,
  Grid,
  Typography,
} from "@mui/material";
import InfoLine from "../InfoLine";

type SummaryEntryProps = {
  machine: string;
  power: string;
  ingredients: string[];
};

function SummaryEntry(props: SummaryEntryProps) {
  return (
    <Grid container alignItems="center">
      <Grid item xs={3}>
        <Typography variant="subtitle2">{props.machine}</Typography>
      </Grid>
      <Grid item xs={2}>
        <Typography variant="subtitle2">{props.power}</Typography>
      </Grid>
      <Grid item xs>
        {props.ingredients.map((e) => {
          return (
            <Typography
              style={{
                textAlign: "right",
                fontSize: "12px",
              }}
              variant="subtitle2"
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
    <Grid container item style={{ margin: 5 }}>
      <Card component={Grid} item elevation={8} xs={12}>
        <Box sx={{ m: 1 }} />
        <Typography style={{ textAlign: "center" }} variant="h6">
          Building Summary
        </Typography>

        <CardContent>
          <InfoLine text="Power Shards" value="0">
            <img
              style={{ height: "2em" }}
              src="/item_icons/Power%20Shard.png"
              loading="lazy"
              alt="Power Shard icon"
            />
          </InfoLine>

          <Divider style={{ marginTop: 10, marginBottom: 10 }} light />

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
          <Divider style={{ marginTop: 10, marginBottom: 10 }} light />

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
