import {
  Box,
  Button,
  Card,
  CardContent,
  Divider,
  Grid,
  Typography,
} from "@mui/joy";
import getMachineData, { MachineData, MachineType } from "./MachineData";
import "./FactoryVisualization.css";
import MaterialIcon from "../material/MaterialIcon";

type MachineProps = {
  machine: MachineData;
};

function Machine(props: MachineProps) {
  return (
    <Box
      className="machine"
      style={{
        width: `${props.machine.visualWidth}rem`,
        height: `${props.machine.visualHeight}rem`,
      }}
    >
      <Typography level="body1">{props.machine.machineSymbol}</Typography>
    </Box>
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
    <Box className={props.className} style={props.style}>
      <Typography>{props.name}</Typography>
      <Box sx={{ m: 0.2 }} />
      <MaterialIcon style={{ height: "1.5em" }} materialName={props.name}/>
      <Box sx={{ m: 0.2 }} />
      <Typography>{props.value}</Typography>
    </Box>
  );
}

type HeaderProps = {
  text: string;
};

function Header(props: HeaderProps) {
  return (
    <Box  className="sub-factory-tooltip-header">
      <Typography style={{ fontSize: "12px" }} level="body1">
        {props.text}
      </Typography>
    </Box>
  );
}

function SubFactory() {
  return (
    <Box
      className="sub-factory"
      style={{ "--sub-factory-color": "128, 128, 128" } as React.CSSProperties}
    >
      <Box className="sub-factory-content">
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
      </Box>
      <Box className="sub-factory-highlight">
        <MaterialIcon materialName={"Nuclear Pasta"} className="sub-factory-output"/>
        <Box className="sub-factory-tooltip">
          <Item
            className="sub-factory-tooltip-header"
            name="Nuclear Pasta"
            value="123/min"
          />
          <Divider />
          <Header text="Input" />
          <Item
            className="sub-factory-tooltip-input"
            name="Silica"
            value="30/min"
          />
          <Divider />
          <Header text="Machine" />
          <Item
            className="sub-factory-tooltip-input"
            name="Assembler"
            value="x1"
          />
        </Box>
      </Box>
    </Box>
  );
}

function Factory() {
  return (
    <Box className="factory">
      <Box className="line-grid" />
      <Box className="factory-container">
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
      </Box>
    </Box>
  );
}

export default function FactoryVisualization() {
  return (
    <Grid container>
      <Card
        component={Grid}
        variant="outlined"
        xs={12}
        style={{ overflow: "visible" }}
      >
        <Typography style={{ textAlign: "center" }} level="h6">
          Factory
        </Typography>

        <Box sx={{ m: 0.5 }}/>

        <CardContent>
          <Factory />
        </CardContent>
      </Card>
    </Grid>
  );
}
