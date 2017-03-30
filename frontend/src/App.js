// @flow
import React, { Component } from "react";
import { connect } from "react-redux";
import { Grid, Row, Col } from "react-bootstrap";
import { loadMatch } from "./actions.js";
import CreateMatchForm from "./components/CreateMatchForm.js";
import MatchList from "./components/MatchList.js";
import "bootstrap/dist/css/bootstrap.css";
import "./App.css";

class App extends Component {
  componentDidMount() {
    fetch("/api/match/").then(res => res.json()).then((json: any[]) => {
      for (let matchData of json) {
        this.props.dispatch(loadMatch(matchData));
      }
    });
  }
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

export default connect()(App);
