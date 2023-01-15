import {
  Box,
  Button,
  FormControl,
  FormHelperText,
  Grid,
  Input,
  InputAdornment,
  InputLabel,
  MenuItem,
  OutlinedInput,
  Paper,
  Select,
  SelectChangeEvent,
  TextField,
} from "@mui/material";
import SelectInput from "@mui/material/Select/SelectInput";
import { useState } from "react";

export default function QueryBar() {
  const [selected, setSelected] = useState("");

  const handleChange = (event: SelectChangeEvent) => {
    setSelected(event.target.value as string);
  };

  return (
    <Grid container item>
      <Paper
        component={Grid}
        container
        item
        elevation={8}
        style={{ padding: 12, margin: 5 }}
        alignItems="center"
      >
        <Box sx={{ m: 0.5 }} />

        <Grid item>
          <FormControl variant="standard" sx={{ minWidth: 120 }}>
            <FormHelperText>Selected Item</FormHelperText>
            <Select value={selected} onChange={handleChange} label="Item">
              <MenuItem value={10}>Ten</MenuItem>
              <MenuItem value={20}>Twenty</MenuItem>
              <MenuItem value={30}>Thirty</MenuItem>
            </Select>
          </FormControl>
        </Grid>

        <Box sx={{ m: 2 }} />

        <Grid item>
          <FormControl variant="standard" sx={{ width: "24ch" }}>
            <FormHelperText>Item Rate</FormHelperText>
            <Input
              endAdornment={
                <InputAdornment position="end">/min</InputAdornment>
              }
            />
          </FormControl>
        </Grid>
      </Paper>
    </Grid>
  );
}
