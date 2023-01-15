import { Box, Card, CardContent, Grid, Typography } from "@mui/material";
import Material from "../Material";

export default function Byproducts() {
  return (
    <Grid container item style={{ margin: 5 }}>
      <Card component={Grid} item elevation={8} xs={12}>
        <Box sx={{ m: 1 }} />

        <Typography style={{ textAlign: "center" }} variant="h6">
          Byproducts
        </Typography>

        <CardContent>
          <Material name="Silica" per_minute={123} />
          <Material name="Water" per_minute={456} />
        </CardContent>
      </Card>
    </Grid>
  );
}
