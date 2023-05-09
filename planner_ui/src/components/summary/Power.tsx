import { Box, Card, CardContent, Grid, Typography } from "@mui/joy";
import InfoLine from "./InfoLine";

export default function Power() {
  return (
    <Grid container xs>
      <Card variant="outlined" component={Grid} xs={12}>
        <Typography sx={{ textAlign: "center" }} level="h6">
          Power
        </Typography>

        <Box sx={{ m: 0.5 }} />

        <CardContent>
          <InfoLine text="Total power used:" value="123 MW" />
          <InfoLine text="Total power generated:" value="456 MW" />
          <InfoLine text="Net power used:" value="-333 MW" />
        </CardContent>
      </Card>
    </Grid>
  );
}
