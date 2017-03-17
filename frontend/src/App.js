// @flow
import React, { Component } from "react";
import { Grid, Row, Col } from "react-bootstrap";
import CreateSetForm from "./components/CreateSetForm.js";
import SetList from "./components/SetList.js";
import "bootstrap/dist/css/bootstrap.css";
import "./App.css";

class App extends Component {
  render() {
    return (
      <Grid>
        <Row>
          <Col>
            <CreateSetForm />
            <hr />
            <SetList />
          </Col>
        </Row>
      </Grid>
    );
  }
}

export default App;
