import React, { Component } from "react";
import { Grid, Row, Col } from "react-bootstrap";
import "bootstrap/dist/css/bootstrap.css";
import "./App.css";
import data from "../../data.json";

class App extends Component {
  render() {
    return (
      <Grid>
        <Row>
          <Col>
            <div className="App">
              <pre>{JSON.stringify(data, null, 2)}</pre>
            </div>
          </Col>
        </Row>
      </Grid>
    );
  }
}

export default App;
