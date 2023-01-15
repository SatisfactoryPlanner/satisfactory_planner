import { Box, Card, CardContent, Grid, Typography } from "@mui/material";
import InfoLine from "../InfoLine";

export default function Power() {
  return (
    <Grid container item style={{ margin: 5 }}>
      <Card component={Grid} item elevation={8} xs={12}>
        <Box sx={{ m: 1 }} />

        <Typography style={{ textAlign: "center" }} variant="h6">
          Power
        </Typography>

        <CardContent>
          <InfoLine text="Total power used:" value="123 MW" />
          <InfoLine text="Total power generated:" value="456 MW" />
          <InfoLine text="Net power used:" value="-333 MW" />
        </CardContent>
      </Card>
    </Grid>
  );
}
