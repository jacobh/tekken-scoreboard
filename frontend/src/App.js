// @flow
import React, { Component } from "react";
import { connect } from "react-redux";
import { Grid, Row, Col } from "react-bootstrap";
import { loadSet } from "./actions.js";
import CreateSetForm from "./components/CreateSetForm.js";
import SetList from "./components/SetList.js";
import "bootstrap/dist/css/bootstrap.css";
import "./App.css";

class App extends Component {
  componentDidMount() {
    fetch("/api/set/").then(res => res.json()).then((json: any[]) => {
      for (let setData of json) {
        this.props.dispatch(loadSet(setData));
      }
    });
  }
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

export default connect()(App);
