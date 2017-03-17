// @flow
import React from "react";
import { Form, FormGroup, ControlLabel, FormControl } from "react-bootstrap";

const PLAYERS = ["Jacob", "Nic", "Nick F.", "Tony"];

export default class CreateSetForm extends React.Component {
  render() {
    return (
      <Form horizontal>
        <h2>Create Set</h2>
        <hr />
        <h3>Player 1</h3>
        <FormGroup>
          <ControlLabel>Player</ControlLabel>
          <FormControl componentClass="select">
            <option value="">Select Player</option>
            {PLAYERS.map(player => <option value={player}>{player}</option>)}
          </FormControl>
        </FormGroup>
      </Form>
    );
  }
}
