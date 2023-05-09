import {Box, Divider, Typography} from "@mui/joy";
import {useEffect, useState} from "react";

export default function Navbar() {
    const [counter, setCounter] = useState(0);

    useEffect(() => {
        if (counter >= 5) {
            document.body.classList.add('yay');
        }
    }, [counter])
  return (
    <nav>
      <Box sx={{ width: "100%" }}>
        <Typography level="h4" sx={{ px: 2, py: 0.5, userSelect: 'none', cursor: 'pointer' }} onClick={() => {
            setCounter(counter + 1);
        }}>
            Satisfactory Planner
        </Typography>
        <Divider />
      </Box>
    </nav>
  );
}