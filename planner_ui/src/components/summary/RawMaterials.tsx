import {
  Box,
  Button,
  Card,
  CardContent,
  FormControl,
  Grid,
  Input,
  TextField,
  Typography,
} from "@mui/joy";
import MaterialInput from "../material/MaterialInput";

export default function RawMaterials() {
  return (
    <Grid container xs>
      <Card variant="outlined" component={Grid} xs={12}>
        <Typography sx={{ textAlign: "center" }} level="h6">
          Raw Materials
        </Typography>

        <Box sx={{ m: 0.5 }} />

        <CardContent>
          <MaterialInput name="Caterium Ore" />
          <MaterialInput name="Iron Ore" />
          <MaterialInput name="Rubber" />
        </CardContent>
      </Card>
    </Grid>
  );
}
