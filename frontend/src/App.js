// @flow
import React, { Component } from "react";
import { Grid, Row, Col } from "react-bootstrap";
import CreateMatchForm from "./components/CreateMatchForm.js";
import MatchList from "./components/MatchList.js";
import PlayerList from "./components/PlayerList.js";
import EloTable from "./components/EloTable.js";
import "bootstrap/dist/css/bootstrap.css";
import "./App.css";

export default class App extends Component {
  render() {
    return (
      <Grid>
        <Row>
          <Col md={12}>
            <CreateMatchForm />
            <hr />
            <PlayerList />
            <hr />
            <MatchList />
            <hr />
            <EloTable />
          </Col>
        </Row>
      </Grid>
    );
  }
}
