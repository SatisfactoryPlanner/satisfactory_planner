import { Box, Card, CardContent, Grid, Typography } from "@mui/joy";
import MaterialDisplay from "../material/MaterialDisplay";

export default function Byproducts() {
  return (
    <Grid container xs>
      <Card variant="outlined" component={Grid} xs={12}>
        <Typography sx={{ textAlign: "center" }} level="h6">
          Byproducts
        </Typography>

        <Box sx={{ m: 0.5 }} />

        <CardContent>
          <MaterialDisplay name="Silica" perMinute={123} />
          <MaterialDisplay name="Water" perMinute={456} />
        </CardContent>
      </Card>
    </Grid>
  );
}
