import {
  Box,
  Button,
  Card,
  CardContent,
  Divider,
  Grid,
  Typography,
} from "@mui/material";
import getMachineData, { MachineData, MachineType } from "./MachineData";
import "./FactoryVisualization.css";
import Material from "../Material";

type MachineProps = {
  machine: MachineData;
};

function Machine(props: MachineProps) {
  return (
    <div
      className="machine"
      style={{
        width: `${props.machine.visualWidth}rem`,
        height: `${props.machine.visualHeight}rem`,
      }}
    >
      <Typography variant="subtitle2">{props.machine.machineSymbol}</Typography>
    </div>
  );
}

type ItemProps = {
  name: string;
  value: string;
  className?: string;
  style?: React.CSSProperties;
};

function Item(props: ItemProps) {
  return (
    <div className={props.className} style={props.style}>
      <Typography>{props.name}</Typography>
      <Box sx={{ m: 0.2 }} />
      <img
        style={{
          height: "1.5em",
        }}
        src={`/item_icons/${props.name}.png`}
        loading="lazy"
        alt={`${props.name} icon`}
      />
      <Box sx={{ m: 0.2 }} />
      <Typography>{props.value}</Typography>
    </div>
  );
}

type HeaderProps = {
  text: string;
};

function Header(props: HeaderProps) {
  return (
    <div className="sub-factory-tooltip-header">
      <Typography style={{ fontSize: "12px" }} variant="subtitle2">
        {props.text}
      </Typography>
    </div>
  );
}

function SubFactory() {
  return (
    <div
      className="sub-factory"
      style={{ "--sub-factory-color": "128, 128, 128" } as React.CSSProperties}
    >
      <div className="sub-factory-content">
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
        <Machine machine={getMachineData(MachineType.Assembler)} />
      </div>
      <div className="sub-factory-highlight">
        <img
          src="/item_icons/Nuclear%20Pasta.png"
          className="sub-factory-output"
        />
        <div className="sub-factory-tooltip">
          <Item
            className="sub-factory-tooltip-header"
            name="Nuclear Pasta"
            value="123/min"
          />
          <Divider style={{ marginTop: 10, marginBottom: 10 }} light />
          <Header text="Input" />
          <Item
            className="sub-factory-tooltip-input"
            name="Silica"
            value="30/min"
          />
          <Divider style={{ marginTop: 10, marginBottom: 10 }} light />
          <Header text="Machine" />
          <Item
            className="sub-factory-tooltip-input"
            name="Assembler"
            value="x1"
          />
        </div>
      </div>
    </div>
  );
}

function Factory() {
  return (
    <div className="factory">
      <div className="line-grid" />
      <div className="factory-container">
        <Grid container>
          <SubFactory />
          <SubFactory />
          <SubFactory />
          <SubFactory />
          <SubFactory />
          <SubFactory />
          <SubFactory />
          <SubFactory />
          <SubFactory />
          <SubFactory />
        </Grid>
      </div>
    </div>
  );
}

export default function FactoryVisualization() {
  return (
    <Grid container item style={{ margin: 5 }}>
      <Card
        component={Grid}
        item
        elevation={8}
        xs={12}
        style={{ overflow: "visible" }}
      >
        <Box sx={{ m: 1 }} />
        <Typography style={{ textAlign: "center" }} variant="h6">
          Factory
        </Typography>

        <CardContent>
          <Factory />
        </CardContent>
      </Card>
    </Grid>
  );
}
