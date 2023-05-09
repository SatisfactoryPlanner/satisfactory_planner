import {Box, Divider, Typography} from "@mui/joy";

export default function Navbar() {
  return (
    <nav>
      <Box sx={{ width: "100%" }}>
        <Typography level="h4" sx={{ px: 2, py: 0.5 }}>Satisfactory Planner</Typography>
        <Divider />
      </Box>
    </nav>
  );
}