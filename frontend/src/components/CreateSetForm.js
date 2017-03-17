// @flow
import React from "react";
import * as Immutable from "immutable";
import { connect } from "react-redux";
import { Form, FormGroup, ControlLabel, FormControl } from "react-bootstrap";

type Props = {
  characters: Immutable.List<Immutable.Map<string, any>>,
  players: Immutable.List<Immutable.Map<string, any>>
};

class CreateSetForm extends React.Component {
  props: Props;

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
            {this.props.players.map(player => (
              <option value={player.get("uuid")} key={player.get("uuid")}>
                {player.get("name")}
              </option>
            ))}
          </FormControl>
        </FormGroup>
        <FormGroup>
          <ControlLabel>Character</ControlLabel>
          <FormControl componentClass="select">
            <option value="">Select Character</option>
            {this.props.characters.map(char => (
              <option value={char.get("uuid")} key={char.get("uuid")}>
                {char.get("name")}
              </option>
            ))}
          </FormControl>
        </FormGroup>
      </Form>
    );
  }
}

export default connect(state => {
  return {
    characters: state.get("characters"),
    players: state.get("players")
  };
})(CreateSetForm);
