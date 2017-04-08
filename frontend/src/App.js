// @flow
import React, { Component } from "react";
import { Grid, Row, Col } from "react-bootstrap";
import CreateMatchForm from "./components/CreateMatchForm.js";
import MatchList from "./components/MatchList.js";
import "bootstrap/dist/css/bootstrap.css";
import "./App.css";

export default class App extends Component {
  render() {
    return (
      <Grid>
        <Row>
          <Col>
            <CreateMatchForm />
            <hr />
            <MatchList />
          </Col>
        </Row>
      </Grid>
    );
  }
}
