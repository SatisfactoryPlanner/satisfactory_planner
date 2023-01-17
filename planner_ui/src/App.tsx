import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import {
  AppBar,
  Box,
  Button,
  Container,
  Grid,
  Paper,
  Toolbar,
  Typography,
} from "@mui/material";
import React from "react";
import QueryBar from "./components/QueryBar";
import FactoryVisualization from "./components/factoryVisualization/FactoryVisualization";
import RawMaterials from "./components/summary/RawMaterials";
import Byproducts from "./components/summary/Byproducts";
import Power from "./components/summary/Power";
import PartsList from "./components/summary/PartsList";
import BuildingSummary from "./components/summary/BuildingSummary";

import init, { greet } from "@rsw/planner_lib_wasm/planner_lib_wasm";

function App() {
  return (
    <React.Fragment>
      <AppBar component="nav">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Satisfactory Planner
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />

      <Grid container>
        <QueryBar />
        <Grid container item xs={2.5}>
          <Grid container item xs={12}>
            <RawMaterials />
          </Grid>
          {/* todo: intermediate products */}
          <Grid container item xs={12}>
            <Byproducts />
          </Grid>
          <Grid container item xs={12}>
            <Power />
          </Grid>
          <Grid container item xs={12}>
            <PartsList />
          </Grid>
          <Grid container item xs={12}>
            <BuildingSummary />
          </Grid>
        </Grid>
        <Grid container item xs>
          <FactoryVisualization />
        </Grid>
      </Grid>
    </React.Fragment>
  );
}

export default App;
