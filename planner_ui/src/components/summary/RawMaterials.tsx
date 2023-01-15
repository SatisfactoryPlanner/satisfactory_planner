import {
  Box,
  Button,
  Card,
  CardContent,
  CardHeader,
  FilledInput,
  FormControl,
  Grid,
  Input,
  Paper,
  TextField,
  Typography,
} from "@mui/material";
import Material from "../Material";

type MaterialInputProps = {
  name: string;
};

function MaterialInput(props: MaterialInputProps) {
  return (
    <Material name={props.name}>
      <Grid item container direction="row-reverse" xs>
        <Grid item>
          <Input size="small" style={{ maxWidth: "10ch" }} />
        </Grid>
      </Grid>
    </Material>
  );
}

export default function RawMaterials() {
  return (
    <Grid container item style={{ margin: 5 }}>
      <Card component={Grid} item elevation={8} xs={12}>
        <Box sx={{ m: 1 }} />
        <Typography style={{ textAlign: "center" }} variant="h6">
          Raw Materials
        </Typography>

        <CardContent>
          <MaterialInput name="Caterium Ore" />
          <MaterialInput name="Iron Ore" />
          <MaterialInput name="Rubber" />
        </CardContent>
      </Card>
    </Grid>
  );
}
