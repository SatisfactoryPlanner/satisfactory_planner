import {useEffect, useState} from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import {
  Box,
  Button,
  Container, CssVarsProvider, extendTheme, Card, Divider,
  Grid,
  Typography, CssBaseline,
} from "@mui/joy";
import React from "react";
import Product from "./components/summary/Product";
import FactoryVisualization from "./components/factoryVisualization/FactoryVisualization";
import RawMaterials from "./components/summary/RawMaterials";
import Byproducts from "./components/summary/Byproducts";
import Power from "./components/summary/Power";
import PartsList from "./components/summary/PartsList";
import BuildingSummary from "./components/summary/BuildingSummary";
import Navbar from "./components/Navbar";

function Summary() {
  return (
      <Grid container rowSpacing={1.5}>
        <Grid xs={12}>
          <Product />
        </Grid>
        <Grid xs={12}>
          <RawMaterials />
        </Grid>
        <Grid xs={12}>
          <Byproducts />
        </Grid>
        <Grid xs={12}>
          <Power />
        </Grid>
        <Grid xs={12}>
          <PartsList />
        </Grid>
        <Grid xs={12}>
          <BuildingSummary />
        </Grid>
      </Grid>
  )
}

function App() {
  return (
    <React.Fragment>
      <CssVarsProvider>
        <CssBaseline />
        <Navbar/>

        <Grid container rowSpacing={2} columnSpacing={1.5} sx={{ m: 0.5 }}>
          <Grid xs={2}>
            <Summary />
          </Grid>
          <Grid xs>
            <FactoryVisualization/>
          </Grid>
        </Grid>
      </CssVarsProvider>
    </React.Fragment>
  );
}

export default App;
